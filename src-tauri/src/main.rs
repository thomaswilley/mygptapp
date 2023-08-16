// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use std::thread::current;
use mygpt::openai::OpenAI;
use mygpt::config::{self, GeneralConfig, OpenAiConfig};

struct MyGPTApp {
    mygpt: Mutex<Option<OpenAI>>,
}

impl MyGPTApp {
    fn new(config_path:String) -> MyGPTApp {
        let mygpt = match config::Config::new(Some(&config_path)) {
            Err(e) => Mutex::new(None),
            Ok(c) => Mutex::new(Some(OpenAI::new(c))),
        };
    
        println!("{:?}", mygpt);
    
        MyGPTApp {
            mygpt,
        }
    }

    pub fn get_current_path(&self) -> String {
        let full_path = std::fs::canonicalize("./mygpt.conf").unwrap();
        full_path.to_string_lossy().to_string().into()
    }
}

#[tauri::command]
fn get_config(mygpt: tauri::State<'_, MyGPTApp>) -> Result<config::Config, String> {
    let lock = mygpt.mygpt.lock();

    match lock {
        Ok(guard) => {
            match *guard {
                Some(ref openai) => Ok(openai.config.clone()),
                None => {
                    Ok(config::Config {
                        general: Some(GeneralConfig { username: Some("".into()) }),
                        openai: OpenAiConfig { api_key: "".into() },
                    })
                }
            }
        },
        Err(_) => Err("Mutex was poisoned.".into())
    }
}

#[tauri::command]
fn get_config_path(mygpt: tauri::State<'_, MyGPTApp>) -> Result<String, String> {
    let lock = mygpt.mygpt.lock();
    Ok(mygpt.get_current_path().clone())
}

#[tauri::command]
fn save_config(mygpt: tauri::State<'_, MyGPTApp>, new_config: config::Config) -> Result<String, String> {
    let lock = mygpt.mygpt.lock();
    let current_path = mygpt.get_current_path().clone();

    match lock {
        Ok(mut guard) => {
            match *guard {
                Some(ref mut openai) => {
                    // If OpenAI instance exists, just update its config
                    openai.config = new_config.clone();
                    openai.config.save(Some(current_path.as_str()))?; 
                },
                None => {
                    return Err("mygpt instance doesnt exist".into());
                    /* 
                    // If OpenAI instance doesn't exist, create a new one and save it
                    *guard = Some(OpenAI {
                        config: new_config.clone(),
                        // other fields of OpenAI...
                    });
                */
                }
            }

            // Return the new config
            Ok(current_path)
        },
        Err(_) => Err("Mutex was poisoned.".into())
    }
}

#[tauri::command]
async fn generate_response(mygpt: tauri::State<'_, MyGPTApp>, prompt: String) -> Result<String, String> {
    let maybe_mygpt = {
        let guard = mygpt.mygpt.lock().map_err(|_| "Mutex was poisoned")?;
        guard.as_ref().cloned()
    };

    let mygpt = match maybe_mygpt {
        Some(mygpt) => mygpt,
        None => return Err("No instance exists. Check config file.".into()),
    };

    match mygpt.immutable_get_response(prompt, None).await {
        Ok(response) => Ok(response),
        Err(e) => Err(e.into()),
    }
}

#[tauri::command]
async fn generate_dummy_response(mygpt: tauri::State<'_, MyGPTApp>, prompt: String) -> Result<String, String> {
    let response = r#"{
        "choices": [
            {
                "finish_reason": "stop",
                "index": 0,
                "message": {
                    "content": "There once was a man from Nantucket,\nWhose tales were known far and wide, no bucket\nCould hold his wit and charm,\nIn his words, he'd disarm,\nWith a twinkle in his eye, and a smile so snugget.",
                    "role": "assistant"
                }
            }
        ],
        "created": 1689267410,
        "id": "chatcmpl-7btuU2VRnKdadz1SYBcnA3ux9Cv8l",
        "model": "gpt-3.5-turbo-0613",
        "object": "chat.completion",
        "usage": {
            "completion_tokens": 9,
            "prompt_tokens": 8,
            "total_tokens": 17
        }
    }"#;
    Ok(response.into())
}

fn main() {
    tauri::Builder::default()
        .manage(MyGPTApp::new("./mygpt.conf".into()))
        .invoke_handler(tauri::generate_handler![generate_response, generate_dummy_response, get_config, save_config, get_config_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
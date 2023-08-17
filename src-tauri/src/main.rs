// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use mygpt::gpterror::GPTError;
use mygpt::openai::{ OpenAI, ModelInfo, ModelInfoMessage };
use mygpt::config::{self, GeneralConfig, OpenAiConfig};
use tauri::api::path::app_data_dir;

struct MyGPTApp {
    mygpt: Mutex<Option<OpenAI>>,
    config_path: String,
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
            config_path,
        }
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
    Ok(mygpt.config_path.clone())
}

#[tauri::command]
fn save_config(mygpt: tauri::State<'_, MyGPTApp>, new_config: config::Config) -> Result<String, String> {
    let lock = mygpt.mygpt.lock();

    match lock {
        Ok(mut guard) => {
            match *guard {
                Some(ref mut openai) => {
                    // If OpenAI instance exists, just update its config
                    openai.config = new_config.clone();
                    openai.config.save(Some(mygpt.config_path.as_str()))?;
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
            Ok(mygpt.config_path.clone())
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

    let model_info = ModelInfo {
        model: "gpt-4".into(),
        messages: vec![ModelInfoMessage::new("system".into(), "".into())],
        temperature: 0.5,
    };

    match mygpt.immutable_get_response(prompt, Some(model_info)).await {
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
        "id": "chatcmpl-7btuU2VRZZKd4dz1SYBcnA3ux9Cv8l",
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

fn main() -> Result<(),GPTError> {
    let config = tauri::Config::default(); 
    let config_base_dir= tauri::api::path::app_config_dir(&config).ok_or(GPTError::Other("cant load config".into()))?;

    let config_dir = config_base_dir.join("mygptapp");
    if !config_dir.exists() {
        std::fs::create_dir(&config_dir)?;
    }

    let config_path = config_dir.join("mygpt.conf");

    if !config_path.exists() {
        let cfg = config::Config {
            general: Some(GeneralConfig { username: Some("user".into()) }),
            openai: OpenAiConfig { api_key: "your-key-here".into() }
        };
        cfg.save(Some(&config_path.display().to_string().as_str()))?;
    }

    tauri::Builder::default()
        .manage(MyGPTApp::new(config_path.display().to_string()))
        .invoke_handler(tauri::generate_handler![generate_response, generate_dummy_response, get_config, save_config, get_config_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
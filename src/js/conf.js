const { invoke } = window.__TAURI__.tauri;
import { AlertError } from "/js/alerts.js";

export function openSettings() {
    var modal = document.querySelector('#modal');
    console.log('open settings');
}

export async function loadSettings() {
    var settings = await invoke("get_config");
    let config_path = await invoke("get_config_path");
    let openai__api_key = settings?.openai?.api_key || '';
    let general__username = settings?.general?.username || '';

    document.getElementById('openai__api_key').value = openai__api_key;
    document.getElementById('general__username').value = general__username;
    document.getElementById('config_path').value = config_path;
    return settings;
}

export async function saveSettings(newSettings) {
    console.log('new settings', newSettings);
    var savedSettings = await invoke("save_config", { newConfig: newSettings }).catch((e)=>{
        AlertError('Unable to save settings', e);
    });
    if (savedSettings) {
        console.log('saved settings', savedSettings);
    }
}

export function toggleSettingsVisibility() {
    var modal = document.querySelector("#modal");
    var cover = modal.firstElementChild;
    var panel = document.querySelector('.panel');
    var bShouldOpen = cover.classList.contains('hidden');
    if (bShouldOpen) {
        loadSettings().then(() => {
            var classes = "transform transition ease-in-out duration-501 sm:duration-700 translate-x-0".split(" ");
            panel.classList.remove('translate-x-full');
            panel.classList.add(...classes);
            cover.classList.remove('hidden');
            cover.hidden = false;
        })
    } else {
        panel.classList.remove('translate-x-0');
        panel.classList.add('translate-x-full');
        cover.classList.add('hidden');
        cover.hidden = true;
    }
}

window.addEventListener("DOMContentLoaded", () => {
    let settingsForm = document.querySelector("#settings-form");
    settingsForm.addEventListener("submit", (e) => {
        e.preventDefault();

        switch (e.submitter.getAttribute('type')) {
            case 'submit': {
                let formData = new FormData(settingsForm);
                let newSettings = {};
                for(let [id, value] of formData.entries()) {
                    if (id.includes('__')) {
                        // Splitting id into parts based on double underscore
                        let parts = id.split('__');
            
                        // Making sure the parent property exists
                        if (!newSettings[parts[0]]) newSettings[parts[0]] = {};
            
                        // Assigning the value to the nested property
                        newSettings[parts[0]][parts[1]] = value;
                    } else {
                        formObject[id] = value;
                    }
                }
                saveSettings(newSettings);
                break;
            }
            case 'cancel': {
                break;
            }
        }
        
        toggleSettingsVisibility();
});
});

window.openSettings = toggleSettingsVisibility;
window.closeSettings = toggleSettingsVisibility;
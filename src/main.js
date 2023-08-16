const { invoke } = window.__TAURI__.tauri;
import { loadSettings } from "/js/conf.js";
import { hideNotification } from "/js/alerts.js";

let promptTextareaEl;
let chatHistoryEl;
let originalScrollHeight;

async function generate_response() {
  var prompt = promptTextareaEl.value;
  var settings = await loadSettings();

  appendPromptToHistory(prompt, settings);
  promptTextareaEl.value = '';
  let responseEl = appendEmptyResponseToHistory();
  updateSubmitUI();

  try {
    var raw_response = await invoke("generate_response", { prompt: prompt });
    try {
      var response = parseMyGPTResponse(raw_response)
      appendResponseToHistory(responseEl, response.choices[0].message.content);
    } catch (e) {
      appendResponseToHistory(responseEl, e);
    }
  } catch (e) {
    console.log('err..', e)
  }
}

function appendPromptToHistory(message, settings) {
  var template = document.getElementById('prompt-message-template');
  var clone = document.importNode(template.content, true);
  clone.querySelector('.pfp').textContent = settings.general.username || 'user';
  clone.querySelector('.message').textContent = message;
  chatHistoryEl.appendChild(clone);
}

function appendEmptyResponseToHistory() {
  var template = document.getElementById('response-message-template');
  var clone = document.importNode(template.content, true);
  clone.querySelector('.pfp').textContent = 'gpt4';
  chatHistoryEl.appendChild(clone);
  var appendedEl = chatHistoryEl.lastElementChild;
  return appendedEl;
}

function appendResponseToHistory(responseEl, message) {
  responseEl.querySelector('.message').textContent = message;
}


function updateSubmitUI() {
  var textarea = document.getElementById('prompt-textarea');
  let submitEl = textarea.nextElementSibling;
  if (textarea.value.trim() !== '') {
    submitEl.classList.add('bg-fuchsia-500');
    submitEl.removeAttribute('disabled');
  } else {
    submitEl.setAttribute('disabled', 'true');
    submitEl.style.backgroundColor = null;
    submitEl.classList.remove('bg-fuchsia-500');
  }
  autoResize(textarea);
}

function autoResize(textarea) {
  textarea.style.height = 'auto';
  if (textarea.value != '') {
    textarea.style.height = (textarea.scrollHeight) + 'px';
  } else {
    textarea.style.height = originalScrollHeight;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  promptTextareaEl = document.querySelector("#prompt-textarea");
  chatHistoryEl = document.querySelector("#chat-history");

  originalScrollHeight = promptTextareaEl.scrollHeight;

  promptTextareaEl.addEventListener('keydown', function (e) {
    if (e.keyCode === 13 && !e.shiftKey) {
      e.preventDefault();
      document.querySelector('#submit').click()
    }
  });

  document.querySelector("#prompt-form").addEventListener("submit", (e) => {
    e.preventDefault();
    generate_response();
  });

  updateSubmitUI();
  promptTextareaEl.addEventListener('input', function () {
    autoResize(this);
    updateSubmitUI();
  }, false);

  // wire up alerts.js
  let notifs = ['alert_error', ];
  notifs.map(notif => {
    let elNotif = document.querySelector(`#${notif}`);
    if (elNotif) {
      let elBtn = elNotif.querySelector('button[type=button]');
      if (elBtn) {
        elBtn.addEventListener('click', () => {
          hideNotification(elNotif);
        })
      }
    }
  })
});

function parseMyGPTResponse(response) {
  try {
    const data = JSON.parse(response);
    const expectedKeys = ["choices", "created", "id", "model", "object", "usage"];
    const hasAllKeys = expectedKeys.every((key) => key in data);

    if (!hasAllKeys) {
      throw new Error(response);
    }

    return data;
  } catch (error) {
    throw new Error(`Failed to parse API response: ${error.message}`);
  }
}
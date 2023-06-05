import init, { process_str, help_text, get_env_json, get_env_tuples } from "./pkg/pils_web.js";

let messages, btn, input;

function sendMessage() {
  let msg = input.value;
  input.value = "";
  addMessage("query", msg, "item-secondary");
  if (msg.startsWith("help")) {
    addMessage("pils", help_text(), "item-primary");
  } else {
    let result = process_str(msg);
    addMessage("pils", result, "item-primary");
    let env = get_env_tuples();
    updateEnvironment(env);
  }
}

function addMessage(from, msg, klass) {
  let formatted = `${from}: ${msg}`;
  console.log(formatted);
  let message = document.createElement("li");
  message.classList.add("message-item", klass);
  message.innerHTML = msg;
  messages.appendChild(message);
  messages.scrollTop = messages.scrollHeight;
}

function updateEnvironment(env) {
  console.log(env)
}

document.addEventListener("DOMContentLoaded", () => {
  messages = document.querySelector(".message-list");
  btn = document.getElementById("btn");
  input = document.getElementById("input");
  input.focus();

  init().then(() => {
    btn.addEventListener("click", () => {
      sendMessage();
      input.focus();
    });
    window.addEventListener("keyup", function (e) {
      if (e.key == "Enter") sendMessage();
    });
    addMessage("pils", help_text(), "item-primary");
  });
});

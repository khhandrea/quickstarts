const form = document.querySelector(".js-form"),
    input = document.querySelector("input"),
    greeting = document.querySelector(".js-greetings");

function saveName(text) {
    localStorage.setItem("currentUser", text);
}

function handleSubmit(event) {
    event.preventDefault();
    const currentValue = input.value;
    paintGreeting(currentValue);
    saveName(currentValue);
}

function askForName() {
    form.classList.add("showing");
    form.addEventListener("submit", handleSubmit);
}

function paintGreeting(text) {
    form.classList.remove("showing");
    greeting.classList.add("showing");
    greeting.innerText = `${text}님, 환영합니다`;
}

function loadName() {
    const currentUser = localStorage.getItem("currentUser");
    if (currentUser === null) {
        askForName();
    } else {
        paintGreeting(currentUser);
    }
}

function init() {
    loadName();
}

init();
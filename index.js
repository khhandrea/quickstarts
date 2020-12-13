const body = document.querySelector("#body");

function handleClick() {
    if (body.className == "clicked") {
        body.className = "";
    } else {
        body.className = "clicked";
    }
}

function init() {
    body.addEventListener("click", handleClick);
}
init(); //changed
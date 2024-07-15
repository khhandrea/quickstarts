const body = document.querySelector('body');

const IMG_NUMBER = 3;


function paintImage(imgNumber) {
    const image = new Image();
    image.src = `./Images/${imgNumber}.jpg`;
    image.classList.add('bgImage');
    body.appendChild(image);
}

function genRandom() {
    return Math.floor(Math.random() * IMG_NUMBER);
}

function init() {
    const randomNumber = genRandom();
    paintImage(randomNumber);
}

init();
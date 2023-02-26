/* Imports */
import { Board, Direction } from "game_2048";

/* HTML elements */
const GAME_CONTAINER = document.getElementById("game-container");

/* Main */
let board = new Board();
board.set_random();
board.set_random();

/* Constants */
const SIZE = 4;

/* Functionality */
const merge_in_direction = (direction) => {
    let steps = JSON.parse(board.merge_all(direction));
    board.set_random();

    animate(steps, () => {
        draw_grid();
    });
};

/* Event listeners */
document.addEventListener("keydown", e => {
    switch (e.key) {
        case "a":
        case "ArrowLeft":
            merge_in_direction(Direction.Left);
            break;

        case "d":
        case "ArrowRight":
            merge_in_direction(Direction.Right);
            break;

        case "w":
        case "ArrowUp":
            merge_in_direction(Direction.Up);
            break;

        case "s":
        case "ArrowDown":
            merge_in_direction(Direction.Down);
            break;
            
        default:
            break;
    };
});

/* Draw grid */
const draw_grid = () => {
    const pieces = board.get_pieces();

    /* Clear grid */
    GAME_CONTAINER.innerHTML = "";

    /* Update */
    pieces.forEach((piece, index) => {
        const brick = document.createElement("div");
        brick.classList.add("brick");
        let y = Math.floor(index / 4);
        let x = Math.floor(index % 4);
        brick.id = `${x}-${y}`
        
        if (piece !== 0) {
            const number = document.createElement("p");
            number.innerText = piece;
            brick.classList.add(`brick-${piece}`);

            brick.appendChild(number);
        };

        const brick_container = document.createElement("div");
        brick_container.classList.add("brick-container");
        brick_container.appendChild(brick);

        GAME_CONTAINER.appendChild(brick_container);
    });
};

/* Animate steps */
const animate = (steps, callback) => {
    steps.forEach(step => {
        let { from_x, from_y, to_x, to_y } = step;
        let brick = document.getElementById(`${from_x}-${from_y}`);

        let x_add = to_x - from_x;
        let y_add = to_y - from_y;
        let x_perc = x_add*100;
        let y_perc = y_add*100;

        brick.animate([
            { transform: `translate(0%, 0%)` },
            { transform: `translate(calc(${x_perc}% + ${x_add}vmin), calc(${y_perc}% + ${y_add}vmin))` },
        ], {
            duration: 80,
            easing: "ease-in-out"
        })
    });

    setTimeout(() => {
        callback();
    }, 80);
};

/* Init */
draw_grid();

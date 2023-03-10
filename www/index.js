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
const ANIMATION_SPEED = 130; // ms

/* Functionality */
const merge_in_direction = (direction) => {
    let steps = JSON.parse(board.merge_all(direction));
    if (steps.length > 0) {
        let [x, y, num] = board.set_random();
        
        animate_popup(x, y, num);
        animate(steps, () => {
            draw_grid();
        });
    }
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
    GAME_CONTAINER.innerHTML = '<div class="shell-container" id="shell-container"></div>';

    /* Update */
    pieces.forEach((piece, index) => {
        const brick = document.createElement("div");
        brick.classList.add("brick");
        let y = Math.floor(index / 4);
        let x = Math.floor(index % 4);
        brick.id = `${x}-${y}`
        
        const brick_container = document.createElement("div");
        brick_container.classList.add("brick-container");
        brick_container.id = `shell-${x}-${y}`;
        
        if (piece !== 0) {
            const number = document.createElement("p");
            number.innerText = piece;
            brick.classList.add(`brick-${piece}`);

            brick_container.classList.add("top-container");
            brick.appendChild(number);
        };
        
        // brick_container.appendChild(brick);
        GAME_CONTAINER.appendChild(brick);
    });

    draw_shells();
};

/* Animate steps */
const animate = (steps, callback) => {
    steps.forEach(step => {
        let { from_x, from_y, to_x, to_y, is_merge } = step;
        let brick = document.getElementById(`${from_x}-${from_y}`);

        let x_add = to_x - from_x;
        let y_add = to_y - from_y;
        let x_perc = x_add*100;
        let y_perc = y_add*100;

        if (is_merge) {
            aniamte_bump(to_x, to_y);

            brick.style.zIndex = 2;
            brick.animate([
                { transform: `translate(0%, 0%) scale(1)` },
                { transform: `translate(calc(${x_perc}% + ${x_add}vmin), calc(${y_perc}% + ${y_add}vmin)) scale(0.6)` },
            ], {
                duration: ANIMATION_SPEED,
                easing: "ease-in-out"
            })

        }else {
            brick.classList.add("top");
            brick.animate([
                { transform: `translate(0%, 0%)` },
                { transform: `translate(calc(${x_perc}% + ${x_add}vmin), calc(${y_perc}% + ${y_add}vmin))` },
            ], {
                duration: ANIMATION_SPEED,
                easing: "ease-in-out"
            })
        }
    });

    setTimeout(() => {
        callback();
    }, ANIMATION_SPEED);
};
const aniamte_bump = (x, y) => {
    setTimeout(() => {
        document.getElementById(`${x}-${y}`).animate([
            { transform: `scale(1)` },
            { transform: `scale(1.025)` },
            { transform: `scale(1.06)` },
            { transform: `scale(1.075)` },
            { transform: `scale(1.085)` },
            { transform: `scale(1.07)` },
            { transform: `scale(1.055)` },
            { transform: `scale(1.03)` },
            { transform: `scale(1.01)` },
            { transform: `scale(1)` },
        ], {
            duration: ANIMATION_SPEED / 2,
            easing: "ease-in-out"
        })
    }, ANIMATION_SPEED + 50);
}

/* Animate piece getting set randomly */
const animate_popup = (x, y, num) => {
    const shell = document.getElementById(`shell-${x}-${y}`);

    const brick = document.createElement("div");
    brick.classList.add("brick-intro");
    
    const number = document.createElement("p");
    number.innerText = num;
    brick.classList.add(`brick-${num}`);

    brick.appendChild(number);
    shell.appendChild(brick);

};

/* Draw the light background of every brick */
const draw_shells = () => {
    const pieces = board.get_pieces();
    document.getElementById("shell-container").innerHTML = "";

    /* Update */
    pieces.forEach((_, index) => {
        const shell = document.createElement("div");
        shell.classList.add("shell");
        let y = Math.floor(index / 4);
        let x = Math.floor(index % 4);
        shell.id = `shell-${x}-${y}`
        
        document.getElementById("shell-container").appendChild(shell);
    });
}

/* Init */
draw_grid();

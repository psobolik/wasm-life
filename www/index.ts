import * as wasm from "../pkg/wasm_life.js";
import "./styles/style.css";

const CELL_COUNT = 100;
const GRID_SIZE = 900;

let running = false;
let time = 100;
let timeout_id = 0;
let generations = 0;

const handleClickInGrid = (event: MouseEvent) => {
    if (running) return;

    const canvas = event.target as HTMLCanvasElement;
    const cell = life_grid.cell_from_point(event.clientX - canvas.offsetLeft, event.clientY - canvas.offsetTop);
    clearGenerations();
    life_grid.toggle_cell_state(cell.row(), cell.col());
    life_grid.draw();
}
const insertRandomPattern = () => {
    const random = (min: number, max: number)=> {
        return Math.random() * (max - min) + min;
    }
    stop();
    clearGenerations();
    let cells: wasm.Cell[] = [];
    const lowerLimit = CELL_COUNT / 5;
    const upperLimit = lowerLimit * 4;
    for (var i = 0; i < GRID_SIZE; ++i) {
        let row = random(lowerLimit, upperLimit);
        let col = random(lowerLimit, upperLimit);
        cells.push(wasm.Cell.new(row, col));
    }
    insertCells(cells);
}
const step = () => {
    stop();
    evolve();
}
const insertPattern = (pattern: wasm.Pattern)=> {
    // console.log(pattern.cells())
    // console.log(pattern.metadata())
    insertCells(pattern.cells());
}
const insertCells = (cells: wasm.Cell[]) => {
    life_grid.vacate_all_cells();
    for(var i = 0; i < cells.length; ++i) {
        // console.log(`${i}: ${cells[i].row()} ${cells[i].col()}`)
        life_grid.set_cell_state(cells[i].row(), cells[i].col(), wasm.CellState.Populated);
    }
    life_grid.draw();
}
const clearGrid = () => {
    stop();
    clearGenerations();
    life_grid.vacate_all_cells();
    life_grid.draw();
}
const rotateClockwise = () => {
    stop();
    life_grid.rotate_clockwise();
    life_grid.draw();
}
const rotateCounterClockwise = () => {
    stop();
    life_grid.rotate_counter_clockwise();
    life_grid.draw();
}
const flipHorizontal = () => {
    stop();
    life_grid.flip_horizontal();
    life_grid.draw();
}
const flipVertical = () => {
    stop();
    life_grid.flip_vertical();
    life_grid.draw();
}
const shiftUp = () => {
    stop();
    life_grid.shift_up();
    life_grid.draw();
}
const shiftDown = () => {
    stop();
    life_grid.shift_down();
    life_grid.draw();
}
const shiftLeft = () => {
    stop();
    life_grid.shift_left();
    life_grid.draw();
}
const shiftRight = () => {
    stop();
    life_grid.shift_right();
    life_grid.draw();
}
const clearGenerations = () => setGenerations(0);
const bumpGenerations = () => setGenerations(generations + 1);
const setGenerations = (value: number)=> {
    generations = value;
    (document.getElementById("generations") as HTMLSpanElement).innerText = `Generations: ${generations}`;
}
const evolve = () => {
    bumpGenerations();
    life_grid.evolve();
    life_grid.draw();
    if (running) timeout_id = setTimeout(evolve, time);
}
const startStop = () => {
    if (running)
        stop();
    else
        start();
}
const start = () => {
    setRunning(true);
    evolve();
}
const stop = () => {
    clearTimeout(timeout_id);
    setRunning(false);
}
const setRunning = (flag: boolean) => {
    running = flag;
    (document.getElementById("play") as HTMLElement).hidden = flag;
    (document.getElementById("pause") as HTMLElement).hidden = !flag;
}
const handleFile = (event: Event) => {
    const docPicker = event.target as HTMLInputElement;
    const textArea = document.getElementById("pattern") as HTMLTextAreaElement;
    if (docPicker.files && docPicker.files.length) {
        const file = docPicker.files[0];
        const reader = new FileReader();
        reader.onload = () => {
            if (reader.result) {
                const content = reader.result.toString();
                textArea.textContent = content;
                if (file.name.endsWith(".cells")) {
                    insertPattern(wasm.PatternParser.parse_cells_data(content));
                } else if (file.name.endsWith(".rle")) {
                    insertPattern(wasm.PatternParser.parse_rle_data(content));
                }
            }
        }
        reader.readAsText(file);
    }
}
const handleKeyup = (event: KeyboardEvent) => {
    // Non-repeating keys
    switch (event.key) {
        case " ":
            event.preventDefault();
            startStop();
            break;
        case "r":
            insertRandomPattern();
            break;
        case "c":
            clearGrid();
            break;
        case "F3":
            flipHorizontal();
            break;
        case "F4":
            flipVertical();
            break;
        case "F1":
            rotateClockwise();
            break;
        case "F2":
            rotateCounterClockwise();
            break;
    }
}
const handleKeydown = (event: KeyboardEvent) => {
    // Repeating keys
    switch (event.key) {
        case "ArrowUp":
            if (!event.ctrlKey) shiftUp();
            break;
        case "ArrowDown":
            if (!event.ctrlKey) shiftDown();
            break;
        case "ArrowLeft":
            if (!event.ctrlKey) shiftLeft();
            break;
        case "ArrowRight":
            if (!event.ctrlKey) shiftRight();
            break;
        case "s":
            step();
            break;
    }
}
document.getElementById("canvas")?.addEventListener('click', handleClickInGrid);
document.getElementById("random")?.addEventListener('click', insertRandomPattern);
document.getElementById("clear")?.addEventListener('click', clearGrid);
document.getElementById("step")?.addEventListener('click', step);
document.getElementById("play")?.addEventListener('click', startStop);
document.getElementById("pause")?.addEventListener('click', startStop);
document.getElementById("rotateCw")?.addEventListener('click', rotateClockwise);
document.getElementById("rotateCcw")?.addEventListener('click', rotateCounterClockwise);
document.getElementById("flipHoriz")?.addEventListener('click', flipHorizontal);
document.getElementById("flipVert")?.addEventListener('click', flipVertical);
document.getElementById("shiftUp")?.addEventListener('click', shiftUp);
document.getElementById("shiftDown")?.addEventListener('click', shiftDown);
document.getElementById("shiftRight")?.addEventListener('click', shiftRight);
document.getElementById("shiftLeft")?.addEventListener('click', shiftLeft);
document.getElementById("docPicker")?.addEventListener('change', handleFile);
document.addEventListener("keydown", handleKeydown);
document.addEventListener("keyup", handleKeyup);

const life_grid = wasm.LifeGrid.new(GRID_SIZE, CELL_COUNT, "canvas");
stop();
life_grid.draw();
clearGenerations();

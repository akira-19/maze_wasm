import init, { Field, Direction, GameStatus } from 'maze_wasm';
import { rnd } from './utils/rnd';

init().then((wasm: any) => {
  const fieldSize = 19;
  const field = Field.new(fieldSize);
  const width = field.width();
  const startIdx = fieldSize + 1;
  const goalIdx = (fieldSize + 1) * (fieldSize - 2);
  let player_idx = field.player_idx();
  let time: number;

  const CELL_SIZE = 20;

  const gameControlBtn = document.getElementById('game-control-btn');
  const canvas = <HTMLCanvasElement>document.getElementById('maze-canvas');
  const ctx = canvas.getContext('2d');
  canvas.height = width * CELL_SIZE;
  canvas.width = width * CELL_SIZE;
  gameControlBtn.addEventListener('click', (_) => {
    if (field.status() !== GameStatus.BeforePlaying) return;
    time = new Date().getTime();
    field.generate_maze();
    drawField();
    colorWalls();
    colorGoal();
    colorPlayer();
  });

  document.addEventListener('keydown', (e) => {
    if (field.status() !== GameStatus.Playing) return;
    e.code === 'ArrowUp' && field.move_player(Direction.Up);
    e.code === 'ArrowDown' && field.move_player(Direction.Down);
    e.code === 'ArrowLeft' && field.move_player(Direction.Left);
    e.code === 'ArrowRight' && field.move_player(Direction.Right);
    resetPlayerColor();
    colorPlayer();

    if (field.status() === GameStatus.Done) {
      setTimeout(() => {
        const timeDiff = Math.floor((new Date().getTime() - time) / 1000);
        alert(`Your Record is ${timeDiff} seconds!`);
      }, 100);
    }
  });
  ctx.font = '12px serif';
  function drawField() {
    if (ctx === null) return;
    ctx.beginPath();
    for (let x = 0; x < width + 1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, CELL_SIZE * width);
    }
    for (let y = 0; y < width + 1; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(CELL_SIZE * width, CELL_SIZE * y);
    }
    ctx.stroke();
  }

  function colorWalls() {
    if (ctx === null) return;
    const walls = field.walls();
    for (let i = 0; i < width * width; i++) {
      if (walls[i] === 1) {
        const x = i % width;
        const y = Math.floor(i / width);
        ctx.fillStyle = 'rgba(0, 0, 0, 0.5)';
        ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
      }
    }
  }

  function colorGoal() {
    if (ctx === null) return;
    const x = goalIdx % width;
    const y = Math.floor(goalIdx / width);
    ctx.fillStyle = 'rgba(0, 0, 255, 0.5)';
    ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
  }

  function colorPlayer() {
    if (ctx === null) return;
    const x = field.player_idx() % width;
    const y = Math.floor(field.player_idx() / width);
    ctx.fillStyle = 'rgba(255, 0, 0, 0.5)';
    ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }

  function resetPlayerColor() {
    if (ctx === null) return;
    const x = player_idx % width;
    const y = Math.floor(player_idx / width);
    ctx.fillStyle = 'rgb(255, 255, 255)';
    ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    player_idx = field.player_idx();
  }
});

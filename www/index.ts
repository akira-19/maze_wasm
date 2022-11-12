import init, { Field } from 'maze_wasm';
import { rnd } from './utils/rnd';

init().then((wasm: any) => {
  const field = Field.new(9);
  const width = field.width();
  const walls = field.walls();
  const walls2 = [];
  for (let i = 0; i < width; i++) {
    const row = [];
    for (let j = 0; j < width; j++) {
      row.push(walls[i * width + j]);
    }
    walls2.push(row);
  }

  const maze = field.generate_maze();

  console.log(maze);

  // const CELL_SIZE = 20;
  // const WORLD_WIDTH = 8;
  // const SNAKE_SPAWN_IDX = rnd(WORLD_WIDTH * WORLD_WIDTH);
  // const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_IDX);
  // const worldWidth = world.width();
  // const gameStatus = document.getElementById('game-status');
  // const points = document.getElementById('points');
  // const gameControlBtn = document.getElementById('game-control-btn');
  // const canvas = <HTMLCanvasElement>document.getElementById('snake-canvas');
  // const ctx = canvas.getContext('2d');
  // canvas.height = worldWidth * CELL_SIZE;
  // canvas.width = worldWidth * CELL_SIZE;
  // gameControlBtn.addEventListener('click', (_) => {
  //   const status = world.game_status();
  //   if (status === undefined) {
  //     gameControlBtn.textContent = 'Playing...';
  //     world.start_game();
  //     play();
  //   } else {
  //     location.reload();
  //   }
  // });
  // const snakeCellPtr = world.snake_cells();
  // const snakeLen = world.snake_length();
  // const snakeCells = new Uint32Array(
  //   wasm.memory.buffer,
  //   snakeCellPtr,
  //   snakeLen,
  // );
  // document.addEventListener('keydown', (e) => {
  //   e.code === 'ArrowUp' && world.change_snake_dir(Direction.Up);
  //   e.code === 'ArrowDown' && world.change_snake_dir(Direction.Down);
  //   e.code === 'ArrowLeft' && world.change_snake_dir(Direction.Left);
  //   e.code === 'ArrowRight' && world.change_snake_dir(Direction.Right);
  // });
  // function drawWorld() {
  //   if (ctx === null) return;
  //   ctx.beginPath();
  //   for (let x = 0; x < worldWidth + 1; x++) {
  //     ctx.moveTo(CELL_SIZE * x, 0);
  //     ctx.lineTo(CELL_SIZE * x, CELL_SIZE * worldWidth);
  //   }
  //   for (let y = 0; y < worldWidth + 1; y++) {
  //     ctx.moveTo(0, CELL_SIZE * y);
  //     ctx.lineTo(CELL_SIZE * worldWidth, CELL_SIZE * y);
  //   }
  //   ctx.stroke();
  // }
  // function drawReward() {
  //   const idx = world.reward_cell();
  //   const col = idx % worldWidth;
  //   const row = Math.floor(idx / worldWidth);
  //   ctx.beginPath();
  //   ctx.fillStyle = '#FF0000';
  //   ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
  //   ctx.stroke();
  // }
  // function drawSnake() {
  //   const snakeCells = new Uint32Array(
  //     wasm.memory.buffer,
  //     world.snake_cells(),
  //     world.snake_length(),
  //   );
  //   snakeCells
  //     // .filter((cellIdx, i) => {
  //     //   i > 0 && cellIdx === snakeCells[0];
  //     // })
  //     .slice()
  //     .reverse()
  //     .forEach((cellIdx, i) => {
  //       const col = cellIdx % worldWidth;
  //       const row = Math.floor(cellIdx / worldWidth);
  //       ctx.fillStyle = i === snakeCells.length - 1 ? '#7878db' : '#000000';
  //       ctx.beginPath();
  //       ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
  //     });
  //   ctx.stroke();
  // }
  // function drawGameStatus() {
  //   const status = world.game_status();
  //   gameStatus.textContent = world.game_status_text();
  //   points.textContent = world.points().toString();
  //   if (status === GameStatus.Won || status === GameStatus.Lost) {
  //     gameControlBtn.textContent = 'Re-Play';
  //   }
  // }
  // function paint() {
  //   drawWorld();
  //   drawSnake();
  //   drawReward();
  //   drawGameStatus();
  // }
  // function play() {
  //   const status = world.game_status();
  //   if (status === GameStatus.Won || status === GameStatus.Lost) {
  //     gameControlBtn.textContent = 'Re-Play';
  //     return;
  //   }
  //   if (ctx === null) return;
  //   const fps = 5;
  //   setTimeout(() => {
  //     ctx.clearRect(0, 0, canvas.width, canvas.height);
  //     world.step();
  //     paint();
  //     // the method takes a callback to invoke before the next repaint
  //     requestAnimationFrame(play);
  //   }, 1000 / fps);
  // }
  // paint();
});

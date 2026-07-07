/**
 * LeetCode 353 - Design Snake Game
 * Snake starts at (0,0) with length 1 on a width x height grid. Food appears one item at a time;
 * move(direction) returns the score, or -1 if the snake hits a wall or its own body.
 * Approach: deque of body cells + Set of occupied cells; tail is freed before the collision check.
 * Time: O(1) amortized per move. Space: O(width * height + food).
 */
type Dir = "U" | "D" | "L" | "R";
const DELTA: Record<Dir, [number, number]> = { U: [-1, 0], D: [1, 0], L: [0, -1], R: [0, 1] };

export class SnakeGame {
  private readonly body: number[] = [0]; // cells encoded as r * width + c; body[head..] with head at the end
  private start = 0;
  private readonly occupied = new Set<number>([0]);
  private foodIdx = 0;
  private score = 0;

  constructor(
    private readonly width: number,
    private readonly height: number,
    private readonly food: Array<[number, number]>,
  ) {}

  move(direction: Dir): number {
    if (this.score < 0) return -1;
    const headCell = this.body[this.body.length - 1];
    const r = Math.floor(headCell / this.width) + DELTA[direction][0];
    const c = (headCell % this.width) + DELTA[direction][1];
    if (r < 0 || r >= this.height || c < 0 || c >= this.width) return (this.score = -1);

    const f = this.food[this.foodIdx];
    const eats = f !== undefined && f[0] === r && f[1] === c;
    if (eats) {
      this.foodIdx++;
      this.score++;
    } else {
      this.occupied.delete(this.body[this.start++]);
    }
    const cell = r * this.width + c;
    if (this.occupied.has(cell)) return (this.score = -1);
    this.occupied.add(cell);
    this.body.push(cell);
    return this.score;
  }
}

const game = new SnakeGame(3, 2, [[1, 2], [0, 1]]);
console.log(["R", "D", "R", "U", "L", "U"].map((d) => game.move(d as Dir))); // [0, 0, 1, 1, 2, -1]

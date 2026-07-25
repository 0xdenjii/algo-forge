/**
 * LeetCode 353 - Design Snake Game
 * Snake starts at (0,0) on a width x height grid and eats food in order.
 * move(dir) returns the score, or -1 if the snake hits a wall or itself.
 * Approach: queue of body cells plus a Set for O(1) collision checks; the tail leaves before
 * the collision test unless food is eaten.
 * Time: O(1) amortized per move (shift is O(n) worst case on arrays). Space: O(W * H).
 */
export class SnakeGame {
  private body: number[] = [0];
  private occupied = new Set<number>([0]);
  private fi = 0;
  constructor(private w: number, private h: number, private food: number[][]) {}

  move(dir: "U" | "D" | "L" | "R"): number {
    const head = this.body[this.body.length - 1];
    let r = Math.floor(head / this.w);
    let c = head % this.w;
    if (dir === "U") r--;
    else if (dir === "D") r++;
    else if (dir === "L") c--;
    else c++;
    if (r < 0 || c < 0 || r >= this.h || c >= this.w) return -1;
    const next = r * this.w + c;
    const f = this.food[this.fi];
    if (f && f[0] === r && f[1] === c) this.fi++;
    else this.occupied.delete(this.body.shift()!);
    if (this.occupied.has(next)) return -1;
    this.body.push(next);
    this.occupied.add(next);
    return this.fi;
  }
}

const sg = new SnakeGame(3, 2, [[1, 2], [0, 1]]);
console.log((["R", "D", "R", "U", "L", "U"] as const).map(d => sg.move(d))); // [0,0,1,1,2,-1]

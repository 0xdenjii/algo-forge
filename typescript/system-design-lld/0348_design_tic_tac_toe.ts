/**
 * LeetCode 348 - Design Tic-Tac-Toe
 * n x n board; move(row, col, player) returns the winner (1 or 2) or 0 if no one has won yet.
 * Approach: row/column/diagonal counters; player 1 adds +1, player 2 adds -1; |n| means a win.
 * Time: O(1) per move. Space: O(n).
 */
export class TicTacToe {
  private rows: number[];
  private cols: number[];
  private diag = 0;
  private anti = 0;
  constructor(private readonly n: number) {
    this.rows = new Array(n).fill(0);
    this.cols = new Array(n).fill(0);
  }
  move(r: number, c: number, player: 1 | 2): number {
    const d = player === 1 ? 1 : -1;
    this.rows[r] += d;
    this.cols[c] += d;
    if (r === c) this.diag += d;
    if (r + c === this.n - 1) this.anti += d;
    const won = [this.rows[r], this.cols[c], this.diag, this.anti].some(v => Math.abs(v) === this.n);
    return won ? player : 0;
  }
}

const ttt = new TicTacToe(3);
const moves: [number, number, 1 | 2][] = [[0, 0, 1], [0, 2, 2], [2, 2, 1], [1, 1, 2], [2, 0, 1], [1, 0, 2], [2, 1, 1]];
console.log(moves.map(([r, c, p]) => ttt.move(r, c, p))); // [0,0,0,0,0,0,1]

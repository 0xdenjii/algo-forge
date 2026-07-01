/**
 * LeetCode 348 - Design Tic-Tac-Toe
 * n x n board, players 1 and 2 alternate valid moves. move(row, col, player) returns the
 * winner (1 or 2) once a full row/column/diagonal is owned by one player, else 0.
 * Approach: signed counters per row, column and both diagonals (+1 for player 1, -1 for player 2).
 * Time: O(1) per move. Space: O(n).
 */
export class TicTacToe {
  private readonly rows: number[];
  private readonly cols: number[];
  private diag = 0;
  private anti = 0;

  constructor(private readonly n: number) {
    this.rows = new Array<number>(n).fill(0);
    this.cols = new Array<number>(n).fill(0);
  }

  move(row: number, col: number, player: 1 | 2): 0 | 1 | 2 {
    const d = player === 1 ? 1 : -1;
    this.rows[row] += d;
    this.cols[col] += d;
    if (row === col) this.diag += d;
    if (row + col === this.n - 1) this.anti += d;
    const target = d * this.n;
    if (this.rows[row] === target || this.cols[col] === target || this.diag === target || this.anti === target) {
      return player;
    }
    return 0;
  }
}

const ttt = new TicTacToe(3);
const moves: Array<[number, number, 1 | 2]> = [[0, 0, 1], [0, 2, 2], [2, 2, 1], [1, 1, 2], [2, 0, 1], [1, 0, 2], [2, 1, 1]];
console.log(moves.map(([r, c, p]) => ttt.move(r, c, p))); // [0,0,0,0,0,0,1]

/**
 * Title: Template Method - Turn-based Game Skeleton (Nim and Tic-Tac-Toe)
 * Problem: Many two-player games share a loop: initialise, while not over
 *   let the current player move, then switch players and report a winner.
 *   Implement the loop once and plug in two different games.
 * Approach: Abstract Game.play() is the template; subclasses supply init,
 *   makeMove, isOver and winner. Players are deterministic for the demo.
 * Complexity: Nim O(heap) moves; Tic-Tac-Toe O(9 * 8) per game.
 */
abstract class Game {
  protected current = 0;
  play(maxTurns = 1000): string {
    this.init();
    for (let t = 0; t < maxTurns && !this.isOver(); t++) {
      this.makeMove(this.current);
      if (!this.isOver()) this.current = 1 - this.current;
    }
    return this.winner();
  }
  protected abstract init(): void;
  protected abstract makeMove(player: number): void;
  protected abstract isOver(): boolean;
  protected abstract winner(): string;
}

/** Nim (take 1-3, whoever takes last wins). Players use the optimal strategy. */
class Nim extends Game {
  private stones = 0;
  constructor(private readonly start: number) { super(); }
  protected init() { this.stones = this.start; this.current = 0; }
  protected makeMove() { const r = this.stones % 4; this.stones -= r === 0 ? 1 : r; }
  protected isOver() { return this.stones === 0; }
  protected winner() { return `Nim(${this.start}): player ${this.current + 1} wins`; }
}

class TicTacToe extends Game {
  private b: (string | null)[] = [];
  private static lines = [[0,1,2],[3,4,5],[6,7,8],[0,3,6],[1,4,7],[2,5,8],[0,4,8],[2,4,6]];
  protected init() { this.b = Array(9).fill(null); this.current = 0; }
  private mark(p: number) { return p === 0 ? "X" : "O"; }
  private line(): string | null {
    for (const [a, b, c] of TicTacToe.lines) if (this.b[a] && this.b[a] === this.b[b] && this.b[a] === this.b[c]) return this.b[a];
    return null;
  }
  protected makeMove(p: number) {
    const me = this.mark(p), them = this.mark(1 - p);
    const free = this.b.map((v, i) => (v ? -1 : i)).filter((i) => i >= 0);
    const wins = (m: string) => free.find((i) => { this.b[i] = m; const ok = this.line() === m; this.b[i] = null; return ok; });
    const pick = wins(me) ?? wins(them) ?? [4, 0, 2, 6, 8, 1, 3, 5, 7].find((i) => this.b[i] === null)!;
    this.b[pick] = me;
  }
  protected isOver() { return this.line() !== null || this.b.every((v) => v); }
  protected winner() { return `TicTacToe: ${this.line() ?? "draw"}`; }
}

console.log(new Nim(10).play()); // player 1 wins (10 % 4 != 0)
console.log(new Nim(8).play());  // player 2 wins
console.log(new TicTacToe().play()); // draw with sensible play

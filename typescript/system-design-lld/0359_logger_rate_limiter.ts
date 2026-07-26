/**
 * LeetCode 359 - Logger Rate Limiter
 * shouldPrintMessage(ts, msg) returns true only if msg was not printed in the last 10 seconds.
 * Approach: map message -> next timestamp at which it may be printed.
 * Time: O(1). Space: O(distinct messages).
 */
export class Logger {
  private next = new Map<string, number>();
  shouldPrintMessage(ts: number, msg: string): boolean {
    if ((this.next.get(msg) ?? -Infinity) > ts) return false;
    this.next.set(msg, ts + 10);
    return true;
  }
}

const lg = new Logger();
console.log(
  lg.shouldPrintMessage(1, "foo"),
  lg.shouldPrintMessage(3, "foo"),
  lg.shouldPrintMessage(11, "foo"),
); // true false true

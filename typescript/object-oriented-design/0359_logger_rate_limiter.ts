/**
 * LeetCode 359 - Logger Rate Limiter
 * shouldPrintMessage(timestamp, message) returns true iff the same message was not printed
 * in the last 10 seconds; if it returns true the message counts as printed at that timestamp.
 * Approach: Map message -> next allowed timestamp.
 * Time: O(1) per call. Space: O(distinct messages).
 */
export class Logger {
  private readonly nextAllowed = new Map<string, number>();

  shouldPrintMessage(timestamp: number, message: string): boolean {
    if (timestamp < (this.nextAllowed.get(message) ?? -Infinity)) return false;
    this.nextAllowed.set(message, timestamp + 10);
    return true;
  }
}

const lg = new Logger();
console.log(
  lg.shouldPrintMessage(1, "foo"), // true
  lg.shouldPrintMessage(2, "bar"), // true
  lg.shouldPrintMessage(3, "foo"), // false
  lg.shouldPrintMessage(11, "foo"), // true
);

/**
 * Title: Adapter - Legacy Printer
 * Problem: A client expects a `Printer` interface with `print(text)`, but the only available
 *   implementation is a legacy class exposing `printLine(chars: string[], newline: boolean)`.
 *   Make the legacy class usable without modifying it.
 * Approach: Object adapter wrapping the legacy instance and translating calls.
 * Complexity: O(n) time per print (splitting the text), O(n) space.
 */
interface Printer {
  print(text: string): void;
}

class LegacyLinePrinter {
  readonly output: string[] = [];
  printLine(chars: string[], newline: boolean): void {
    this.output.push(chars.join("") + (newline ? "\n" : ""));
  }
}

class LegacyPrinterAdapter implements Printer {
  constructor(private readonly legacy: LegacyLinePrinter) {}
  print(text: string): void {
    for (const line of text.split("\n")) this.legacy.printLine([...line], true);
  }
}

function report(printer: Printer): void {
  printer.print("Report\n=====\nAll good");
}

const legacy = new LegacyLinePrinter();
report(new LegacyPrinterAdapter(legacy));
console.log(legacy.output.join(""));

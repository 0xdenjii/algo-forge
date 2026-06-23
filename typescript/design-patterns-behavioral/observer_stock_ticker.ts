/**
 * Title: Observer - Stock Ticker
 * Problem: A ticker publishes price updates. Displays, alert rules and
 *   loggers subscribe, are notified only on actual change, and can
 *   unsubscribe at any time.
 * Approach: Subject keeps a Set of observers; subscribe returns an unsubscribe fn.
 * Complexity: O(n) per notify for n observers; O(n + symbols) space.
 */
type Quote = { symbol: string; price: number };
type Observer = (q: Quote) => void;

class StockTicker {
  private observers = new Set<Observer>();
  private prices = new Map<string, number>();

  subscribe(o: Observer): () => void {
    this.observers.add(o);
    return () => { this.observers.delete(o); };
  }

  update(symbol: string, price: number): void {
    if (this.prices.get(symbol) === price) return;
    this.prices.set(symbol, price);
    for (const o of [...this.observers]) o({ symbol, price }); // copy: safe if observers unsubscribe
  }
}

const ticker = new StockTicker();
const log: string[] = [];
const unsubDisplay = ticker.subscribe((q) => log.push(`display ${q.symbol}=${q.price}`));
ticker.subscribe((q) => { if (q.price > 200) log.push(`ALERT ${q.symbol} above 200`); });
ticker.update("AAPL", 190);
ticker.update("AAPL", 190); // unchanged, ignored
ticker.update("AAPL", 210);
unsubDisplay();
ticker.update("AAPL", 220);
console.log(log.join("\n"));

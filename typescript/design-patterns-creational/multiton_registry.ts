/**
 * Title: Multiton (One Instance per Key)
 * Problem: Provide one CurrencyFormatter per locale+currency; requesting the same
 *   key twice returns the same object.
 * Approach: static Map<key, instance> with lazy creation.
 * Complexity: O(1) average lookup, O(k) space for k keys.
 */
export class CurrencyFormatter {
  private static readonly pool = new Map<string, CurrencyFormatter>();
  private readonly fmt: Intl.NumberFormat;
  private constructor(readonly locale: string, currency: string) {
    this.fmt = new Intl.NumberFormat(locale, { style: "currency", currency });
  }
  static of(locale: string, currency: string): CurrencyFormatter {
    const key = `${locale}|${currency}`;
    let inst = this.pool.get(key);
    if (!inst) { inst = new CurrencyFormatter(locale, currency); this.pool.set(key, inst); }
    return inst;
  }
  format(n: number): string { return this.fmt.format(n); }
}

console.log(CurrencyFormatter.of("en-US", "USD") === CurrencyFormatter.of("en-US", "USD")); // true
console.log(CurrencyFormatter.of("de-DE", "EUR").format(1234.5));

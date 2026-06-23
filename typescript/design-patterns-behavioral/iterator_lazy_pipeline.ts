/**
 * Title: Iterator - Lazy Sequence Pipeline (generators)
 * Problem: Build a chainable lazy sequence (map, filter, take, chunk, zip)
 *   over any Iterable, including infinite ones, so that only the elements
 *   actually consumed are computed.
 * Approach: Seq<T> wraps an Iterable; every operator returns a new Seq
 *   backed by a generator (external iterator protocol).
 * Complexity: O(k) work for k consumed elements; O(1) space per stage
 *   (O(size) for chunk).
 */
class Seq<T> implements Iterable<T> {
  constructor(private readonly src: Iterable<T>) {}
  static naturals(start = 0): Seq<number> {
    return new Seq((function* () { for (let i = start; ; i++) yield i; })());
  }
  [Symbol.iterator]() { return this.src[Symbol.iterator](); }

  map<U>(f: (x: T) => U): Seq<U> { const s = this.src; return new Seq((function* () { for (const x of s) yield f(x); })()); }
  filter(p: (x: T) => boolean): Seq<T> { const s = this.src; return new Seq((function* () { for (const x of s) if (p(x)) yield x; })()); }
  take(n: number): Seq<T> {
    const s = this.src;
    return new Seq((function* () { if (n <= 0) return; let i = 0; for (const x of s) { yield x; if (++i >= n) return; } })());
  }
  chunk(size: number): Seq<T[]> {
    if (size < 1) throw new RangeError("size must be >= 1");
    const s = this.src;
    return new Seq((function* () { let buf: T[] = []; for (const x of s) { buf.push(x); if (buf.length === size) { yield buf; buf = []; } } if (buf.length) yield buf; })());
  }
  zip<U>(other: Iterable<U>): Seq<[T, U]> {
    const s = this.src;
    return new Seq((function* () {
      const a = s[Symbol.iterator](), b = other[Symbol.iterator]();
      for (;;) { const x = a.next(), y = b.next(); if (x.done || y.done) return; yield [x.value, y.value] as [T, U]; }
    })());
  }
  toArray(): T[] { return [...this.src]; }
}

let evaluated = 0;
const primes = Seq.naturals(2).filter((n) => {
  evaluated++;
  for (let d = 2; d * d <= n; d++) if (n % d === 0) return false;
  return true;
});
console.log(primes.take(10).toArray(), "checked:", evaluated); // first 10 primes, 28 checks (2..29)
console.log(Seq.naturals(1).map((x) => x * x).take(7).chunk(3).toArray());
console.log(new Seq("abc").zip(Seq.naturals()).toArray());

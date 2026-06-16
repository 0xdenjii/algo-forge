/**
 * Title: Decorator - Data Source Pipeline (compression + encryption)
 * Problem: A DataSource writes/reads strings. Add optional run-length compression and a
 *   (toy) XOR+base64 encryption layer that can be stacked in any order, transparently.
 * Approach: Decorators transform data on write and invert the transform on read.
 * Complexity: O(n) per layer per read/write.
 */
interface DataSource { write(data: string): void; read(): string }

class MemoryDataSource implements DataSource {
  private stored = "";
  write(data: string): void { this.stored = data; }
  read(): string { return this.stored; }
  raw(): string { return this.stored; }
}

abstract class DataSourceDecorator implements DataSource {
  constructor(protected wrappee: DataSource) {}
  write(data: string): void { this.wrappee.write(this.encode(data)); }
  read(): string { return this.decode(this.wrappee.read()); }
  protected abstract encode(s: string): string;
  protected abstract decode(s: string): string;
}

/** Run-length encoding: "aaab" -> "3a1b". Input must not contain digits (they would be ambiguous). */
class RleCompression extends DataSourceDecorator {
  protected encode(s: string): string {
    if (/\d/.test(s)) throw new Error("RLE input must not contain digits");
    let out = "";
    for (let i = 0; i < s.length; ) {
      let j = i;
      while (j < s.length && s[j] === s[i]) j++;
      out += `${j - i}${s[i]}`;
      i = j;
    }
    return out;
  }
  protected decode(s: string): string {
    let out = "";
    let i = 0;
    while (i < s.length) {
      let n = "";
      while (/\d/.test(s[i])) n += s[i++];
      out += s[i++].repeat(Number(n));
    }
    return out;
  }
}

class XorEncryption extends DataSourceDecorator {
  constructor(w: DataSource, private key: number) { super(w); }
  protected encode(s: string): string {
    const bytes = [...s].map((c) => String.fromCharCode(c.charCodeAt(0) ^ this.key));
    return btoa(bytes.join(""));
  }
  protected decode(s: string): string {
    return [...atob(s)].map((c) => String.fromCharCode(c.charCodeAt(0) ^ this.key)).join("");
  }
}

const sink = new MemoryDataSource();
const pipeline: DataSource = new XorEncryption(new RleCompression(sink), 42);
pipeline.write("aaaaabbbcccccccd");
console.log("stored:", sink.raw());
console.log("read back:", pipeline.read());

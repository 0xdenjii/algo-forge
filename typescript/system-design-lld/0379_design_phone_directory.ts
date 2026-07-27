/**
 * LeetCode 379 - Design Phone Directory
 * Manage numbers 0..max-1: get() hands out an available number, check(n), release(n).
 * Approach: stack of free numbers plus a used-flag array.
 * Time: O(1) per op. Space: O(max).
 */
export class PhoneDirectory {
  private free: number[];
  private used: boolean[];
  constructor(max: number) {
    this.free = Array.from({ length: max }, (_, i) => max - 1 - i);
    this.used = new Array(max).fill(false);
  }
  get(): number {
    const n = this.free.pop();
    if (n === undefined) return -1;
    this.used[n] = true;
    return n;
  }
  check(n: number): boolean {
    return !this.used[n];
  }
  release(n: number): void {
    if (this.used[n]) {
      this.used[n] = false;
      this.free.push(n);
    }
  }
}

const pd = new PhoneDirectory(3);
console.log(pd.get(), pd.get(), pd.check(2), pd.get(), pd.check(2)); // 0 1 true 2 false
pd.release(2);
console.log(pd.check(2)); // true

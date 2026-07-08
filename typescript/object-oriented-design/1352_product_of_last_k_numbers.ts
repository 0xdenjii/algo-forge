/**
 * LeetCode 1352 - Product of the Last K Numbers
 * add(num) appends to a stream; getProduct(k) returns the product of the last k numbers.
 * Approach: prefix products reset whenever a 0 arrives; if k reaches past the last zero, answer is 0.
 * Time: O(1) per op. Space: O(n).
 */
export class ProductOfNumbers {
  private prefix: number[] = [1];

  add(num: number): void {
    if (num === 0) this.prefix = [1];
    else this.prefix.push(this.prefix[this.prefix.length - 1] * num);
  }

  getProduct(k: number): number {
    const n = this.prefix.length;
    if (k >= n) return 0;
    return this.prefix[n - 1] / this.prefix[n - 1 - k];
  }
}

const pn = new ProductOfNumbers();
[3, 0, 2, 5, 4].forEach((x) => pn.add(x));
console.log(pn.getProduct(2), pn.getProduct(3), pn.getProduct(4)); // 20 40 0
pn.add(8);
console.log(pn.getProduct(2)); // 32

/**
 * Title: Generic Constructor Factory
 * Problem: Write create(Ctor, ...args) that instantiates any class with
 *   correctly typed constructor arguments, plus a counting factory wrapper.
 * Approach: `new (...args: A) => T` constraint; TS infers A from the constructor.
 * Complexity: O(1).
 */
type Ctor<T, A extends unknown[]> = new (...args: A) => T;
export function create<T, A extends unknown[]>(C: Ctor<T, A>, ...args: A): T { return new C(...args); }

export function countingFactory<T, A extends unknown[]>(C: Ctor<T, A>) {
  let n = 0;
  return { make: (...args: A): T => { n++; return new C(...args); }, get count() { return n; } };
}

class Point { constructor(readonly x: number, readonly y: number) {} }
class User { constructor(readonly name: string, readonly admin = false) {} }
const p = create(Point, 1, 2);
const u = create(User, "ada", true);
// create(Point, "1", 2)  -> compile error: string is not assignable to number
const pf = countingFactory(Point); pf.make(0, 0); pf.make(3, 4);
console.log(p, u, pf.count); // Point{1,2} User{ada,true} 2

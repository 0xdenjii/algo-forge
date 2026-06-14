/**
 * Title: Composite - Organization Chart
 * Problem: Employees are either individual contributors or managers with reports. Compute
 *   the total salary cost of any subtree, its headcount, and the depth of the hierarchy.
 * Approach: Common Employee interface; Manager aggregates its reports recursively.
 * Complexity: O(n) per query, O(depth) stack.
 */
interface Employee {
  name: string;
  cost(): number;
  headcount(): number;
  depth(): number;
}

class Engineer implements Employee {
  constructor(public name: string, private salary: number) {}
  cost(): number { return this.salary; }
  headcount(): number { return 1; }
  depth(): number { return 1; }
}

class Manager implements Employee {
  private reports: Employee[] = [];
  constructor(public name: string, private salary: number) {}
  addReport(e: Employee): this { this.reports.push(e); return this; }
  cost(): number { return this.salary + this.reports.reduce((s, r) => s + r.cost(), 0); }
  headcount(): number { return 1 + this.reports.reduce((s, r) => s + r.headcount(), 0); }
  depth(): number { return 1 + Math.max(0, ...this.reports.map((r) => r.depth())); }
}

const cto = new Manager("Ada", 250_000)
  .addReport(new Manager("Linus", 180_000).addReport(new Engineer("Grace", 140_000)).addReport(new Engineer("Ken", 130_000)))
  .addReport(new Engineer("Barbara", 150_000));
console.log({ cost: cto.cost(), headcount: cto.headcount(), depth: cto.depth() }); // 850000, 5, 3

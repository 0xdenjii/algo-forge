/**
 * LLD - Library Management System
 * Members borrow and return book copies, with a per-member loan limit, due dates, late fines,
 * and FIFO holds: a returned copy is reserved for the first member waiting on that title.
 * Approach: Catalog (isbn -> copies), Loan records, hold queues per isbn; fine policy injected.
 * Time: borrow/return O(copies of title). Space: O(books + loans + holds).
 */
interface Copy { barcode: string; isbn: string; status: "AVAILABLE" | "LOANED" | "ON_HOLD"; holdFor?: string }
interface Loan { barcode: string; member: string; due: number }

const DAY = 86_400_000;

export class Library {
  private copies = new Map<string, Copy[]>();
  private loans = new Map<string, Loan>();
  private holds = new Map<string, string[]>();
  constructor(private maxLoans = 3, private loanDays = 14, private finePerDay = 25) {}

  addCopy(isbn: string, barcode: string): void {
    if (!this.copies.has(isbn)) this.copies.set(isbn, []);
    this.copies.get(isbn)!.push({ barcode, isbn, status: "AVAILABLE" });
  }

  private loansOf(member: string): number {
    return [...this.loans.values()].filter(l => l.member === member).length;
  }

  borrow(member: string, isbn: string, now: number): Loan | "HOLD_PLACED" {
    if (this.loansOf(member) >= this.maxLoans) throw new Error("loan limit reached");
    const list = this.copies.get(isbn) ?? [];
    const c =
      list.find(x => x.status === "ON_HOLD" && x.holdFor === member) ?? list.find(x => x.status === "AVAILABLE");
    if (!c) {
      if (!this.holds.has(isbn)) this.holds.set(isbn, []);
      this.holds.get(isbn)!.push(member);
      return "HOLD_PLACED";
    }
    c.status = "LOANED";
    c.holdFor = undefined;
    const loan = { barcode: c.barcode, member, due: now + this.loanDays * DAY };
    this.loans.set(c.barcode, loan);
    return loan;
  }

  giveBack(barcode: string, now: number): number {
    const loan = this.loans.get(barcode);
    if (!loan) throw new Error("not on loan");
    this.loans.delete(barcode);
    const copy = [...this.copies.values()].flat().find(c => c.barcode === barcode)!;
    const next = this.holds.get(copy.isbn)?.shift();
    copy.status = next ? "ON_HOLD" : "AVAILABLE";
    copy.holdFor = next;
    const lateDays = Math.max(0, Math.ceil((now - loan.due) / DAY));
    return lateDays * this.finePerDay;
  }
}

const lib = new Library();
lib.addCopy("978-0", "B1");
const loan = lib.borrow("ann", "978-0", 0) as Loan;
console.log(lib.borrow("ben", "978-0", 0)); // HOLD_PLACED
console.log(lib.giveBack(loan.barcode, 16 * DAY)); // 50 (2 days late)
console.log(lib.borrow("ben", "978-0", 16 * DAY)); // loan for ben

/**
 * LLD - Library Management System
 * Members borrow and return book copies; each member has a loan limit, overdue returns accrue
 * a fine, and members with unpaid fines cannot borrow. Books can be reserved when no copy is free,
 * and a returned copy goes to the first reservation holder.
 * Approach: Book (title metadata) vs BookCopy (physical item); Library service enforces policies;
 * results use a discriminated union instead of exceptions for expected business failures.
 * Time: borrow/return O(copies of that book). Space: O(books + copies + loans).
 */
type Result<T> = { ok: true; value: T } | { ok: false; error: string };
const ok = <T>(value: T): Result<T> => ({ ok: true, value });
const err = (error: string): Result<never> => ({ ok: false, error });

interface Book {
  isbn: string;
  title: string;
}
interface BookCopy {
  id: string;
  isbn: string;
  borrower: Member | null;
  due: number;
}

export class Member {
  readonly loans = new Set<BookCopy>();
  fines = 0;
  constructor(readonly id: string, readonly maxLoans = 3) {}
}

const DAY = 86_400_000;

export class Library {
  private readonly books = new Map<string, Book>();
  private readonly copies = new Map<string, BookCopy[]>();
  private readonly reservations = new Map<string, Member[]>();
  constructor(private readonly loanDays = 14, private readonly finePerDay = 0.5) {}

  addBook(isbn: string, title: string, nCopies: number): void {
    this.books.set(isbn, { isbn, title });
    const list = this.copies.get(isbn) ?? [];
    for (let i = 0; i < nCopies; i++) list.push({ id: `${isbn}#${list.length}`, isbn, borrower: null, due: 0 });
    this.copies.set(isbn, list);
  }

  borrow(m: Member, isbn: string, now: number): Result<BookCopy> {
    if (!this.books.has(isbn)) return err("no such book");
    if (m.fines > 0) return err(`unpaid fines ${m.fines}`);
    if (m.loans.size >= m.maxLoans) return err("loan limit reached");
    const queue = this.reservations.get(isbn) ?? [];
    if (queue.length && queue[0] !== m) return err("reserved by another member");
    const copy = this.copies.get(isbn)!.find((c) => c.borrower === null);
    if (!copy) return err("no free copy; reserve it");
    if (queue[0] === m) queue.shift();
    copy.borrower = m;
    copy.due = now + this.loanDays * DAY;
    m.loans.add(copy);
    return ok(copy);
  }

  reserve(m: Member, isbn: string): Result<number> {
    const q = this.reservations.get(isbn) ?? [];
    if (!q.includes(m)) q.push(m);
    this.reservations.set(isbn, q);
    return ok(q.indexOf(m) + 1);
  }

  giveBack(copy: BookCopy, now: number): Result<number> {
    const m = copy.borrower;
    if (!m) return err("copy is not on loan");
    const lateDays = Math.max(0, Math.ceil((now - copy.due) / DAY));
    const fine = lateDays * this.finePerDay;
    m.fines += fine;
    m.loans.delete(copy);
    copy.borrower = null;
    return ok(fine);
  }

  pay(m: Member, amount: number): void {
    m.fines = Math.max(0, m.fines - amount);
  }
}

const lib = new Library();
lib.addBook("978-0", "Design Patterns", 1);
const alice = new Member("alice");
const bob = new Member("bob");
const loan = lib.borrow(alice, "978-0", 0);
console.log(loan.ok, lib.borrow(bob, "978-0", 0)); // true { ok: false, error: 'no free copy; reserve it' }
console.log(lib.reserve(bob, "978-0")); // { ok: true, value: 1 }
if (loan.ok) console.log(lib.giveBack(loan.value, 17 * DAY)); // { ok: true, value: 1.5 }
console.log(lib.borrow(alice, "978-0", 17 * DAY).ok, lib.borrow(bob, "978-0", 17 * DAY).ok); // false true

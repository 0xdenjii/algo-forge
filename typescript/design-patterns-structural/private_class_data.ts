/**
 * Title: Private Class Data - Immutable Bank Account State
 * Problem: A BankAccount's owner, IBAN and opening date must be set once and never changed,
 *   while the balance evolves only through deposit/withdraw. Prevent accidental mutation
 *   even from inside the class.
 * Approach: Keep write-once attributes in a frozen data object held in an ES `#private`
 *   field; balance is a separate private field mutated only by guarded methods.
 * Complexity: O(1) per operation.
 */
interface AccountData { readonly owner: string; readonly iban: string; readonly openedOn: string }

class BankAccount {
  readonly #data: Readonly<AccountData>;
  #balanceCents = 0;

  constructor(data: AccountData) {
    if (!/^[A-Z]{2}\d{2}[A-Z0-9]{10,30}$/.test(data.iban)) throw new Error("invalid IBAN format");
    this.#data = Object.freeze({ ...data });
  }
  get owner(): string { return this.#data.owner; }
  get iban(): string { return this.#data.iban; }
  get balance(): number { return this.#balanceCents; }

  deposit(cents: number): void {
    if (!Number.isInteger(cents) || cents <= 0) throw new RangeError("deposit must be a positive integer");
    this.#balanceCents += cents;
  }
  withdraw(cents: number): void {
    if (!Number.isInteger(cents) || cents <= 0) throw new RangeError("withdrawal must be a positive integer");
    if (cents > this.#balanceCents) throw new Error("insufficient funds");
    this.#balanceCents -= cents;
  }
  toJSON(): object { return { ...this.#data, balance: this.#balanceCents }; }
}

const acct = new BankAccount({ owner: "Jane Doe", iban: "FR7630006000011234567890189", openedOn: "2026-01-02" });
acct.deposit(10_000);
acct.withdraw(2_500);
try { acct.withdraw(1_000_000); } catch (e) { console.log((e as Error).message); }
try { (acct as unknown as { owner: string }).owner = "Mallory"; } catch (e) { console.log("blocked:", (e as Error).message); }
console.log(JSON.stringify(acct));

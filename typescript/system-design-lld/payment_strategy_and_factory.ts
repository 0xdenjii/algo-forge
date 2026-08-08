/**
 * LLD - Checkout with Strategy + Factory patterns
 * A checkout must support several payment methods (card, wallet, bank transfer) with different
 * fees and validation, and new methods should be addable without modifying checkout code.
 * Approach: PaymentStrategy interface; a registry-based factory maps method names to creators.
 * Time: O(1) per payment. Space: O(registered methods).
 */
export interface PaymentResult { ok: boolean; charged: number; ref?: string; error?: string }

export interface PaymentStrategy {
  readonly name: string;
  pay(amountCents: number): PaymentResult;
}

class CardPayment implements PaymentStrategy {
  readonly name = "card";
  constructor(private cardNumber: string) {}
  private luhn(n: string): boolean {
    let sum = 0;
    [...n].reverse().forEach((ch, i) => {
      let d = Number(ch);
      if (i % 2) d = d * 2 > 9 ? d * 2 - 9 : d * 2;
      sum += d;
    });
    return n.length >= 12 && sum % 10 === 0;
  }
  pay(amount: number): PaymentResult {
    if (!this.luhn(this.cardNumber)) return { ok: false, charged: 0, error: "invalid card" };
    const charged = amount + Math.round(amount * 0.029) + 30;
    return { ok: true, charged, ref: `card-****${this.cardNumber.slice(-4)}` };
  }
}

class WalletPayment implements PaymentStrategy {
  readonly name = "wallet";
  constructor(private balance: { cents: number }) {}
  pay(amount: number): PaymentResult {
    if (this.balance.cents < amount) return { ok: false, charged: 0, error: "insufficient funds" };
    this.balance.cents -= amount;
    return { ok: true, charged: amount, ref: "wallet" };
  }
}

class BankTransfer implements PaymentStrategy {
  readonly name = "bank";
  constructor(private iban: string) {}
  pay(amount: number): PaymentResult {
    return /^[A-Z]{2}\d{2}[A-Z0-9]{10,30}$/.test(this.iban)
      ? { ok: true, charged: amount + 50, ref: `sepa-${this.iban.slice(0, 4)}` }
      : { ok: false, charged: 0, error: "invalid IBAN" };
  }
}

type Creator = (params: Record<string, unknown>) => PaymentStrategy;

export class PaymentFactory {
  private static registry = new Map<string, Creator>([
    ["card", p => new CardPayment(String(p.number))],
    ["wallet", p => new WalletPayment(p.balance as { cents: number })],
    ["bank", p => new BankTransfer(String(p.iban))],
  ]);
  static register(name: string, c: Creator): void {
    this.registry.set(name, c);
  }
  static create(name: string, params: Record<string, unknown>): PaymentStrategy {
    const c = this.registry.get(name);
    if (!c) throw new Error(`unknown payment method ${name}`);
    return c(params);
  }
}

export function checkout(total: number, method: PaymentStrategy): PaymentResult {
  return method.pay(total);
}

console.log(checkout(1000, PaymentFactory.create("card", { number: "4111111111111111" })));
const wallet = { cents: 500 };
console.log(checkout(1000, PaymentFactory.create("wallet", { balance: wallet })));
console.log(checkout(1000, PaymentFactory.create("bank", { iban: "DE89370400440532013000" })));

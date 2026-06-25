/**
 * Title: Strategy - Payment Processing
 * Problem: A checkout must charge customers via card, PayPal or crypto. Each
 *   method has its own fee rules. Swap methods at runtime without changing
 *   the checkout code.
 * Approach: PaymentStrategy interface; Checkout holds a strategy reference.
 * Complexity: O(1) time per payment, O(1) space.
 */
interface PaymentStrategy {
  readonly name: string;
  fee(amount: number): number;
  pay(amount: number): string;
}

const round2 = (x: number): number => Math.round(x * 100) / 100;

class CardPayment implements PaymentStrategy {
  readonly name = "card";
  constructor(private readonly last4: string) {}
  fee(amount: number): number { return round2(amount * 0.029 + 0.3); }
  pay(amount: number): string { return `Charged ${round2(amount + this.fee(amount))} to card ****${this.last4}`; }
}

class PayPalPayment implements PaymentStrategy {
  readonly name = "paypal";
  constructor(private readonly email: string) {}
  fee(amount: number): number { return round2(amount * 0.034); }
  pay(amount: number): string { return `PayPal ${this.email} paid ${round2(amount + this.fee(amount))}`; }
}

class CryptoPayment implements PaymentStrategy {
  readonly name = "crypto";
  constructor(private readonly wallet: string) {}
  fee(): number { return 1; }
  pay(amount: number): string { return `Sent ${amount + this.fee()} USDC from ${this.wallet}`; }
}

class Checkout {
  constructor(private strategy: PaymentStrategy) {}
  setStrategy(s: PaymentStrategy): void { this.strategy = s; }
  complete(amount: number): string {
    if (amount <= 0) throw new RangeError("amount must be positive");
    return this.strategy.pay(amount);
  }
}

const checkout = new Checkout(new CardPayment("4242"));
console.log(checkout.complete(100));
checkout.setStrategy(new PayPalPayment("a@b.com"));
console.log(checkout.complete(100));
checkout.setStrategy(new CryptoPayment("0xabc"));
console.log(checkout.complete(100));

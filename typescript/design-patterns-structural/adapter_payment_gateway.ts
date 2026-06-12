/**
 * Title: Adapter - Payment Gateways
 * Problem: A checkout service charges via `PaymentProcessor.pay(amountCents, currency)`.
 *   Two third-party SDKs have incompatible APIs (one takes dollars as a float, one takes an
 *   options object and is async). Unify them behind one interface.
 * Approach: One adapter class per SDK; checkout depends only on the target interface.
 * Complexity: O(1) per call.
 */
interface PaymentResult { ok: boolean; reference: string }
interface PaymentProcessor { pay(amountCents: number, currency: string): Promise<PaymentResult> }

// Third-party SDK A (synchronous, dollars)
class StripeLikeSdk {
  charge(dollars: number, cur: string): { id: string; status: "succeeded" | "failed" } {
    return { id: `ch_${Math.round(dollars * 100)}_${cur}`, status: dollars > 0 ? "succeeded" : "failed" };
  }
}
// Third-party SDK B (async, options object)
class PaypalLikeSdk {
  async createPayment(opts: { total: string; currency_code: string }): Promise<{ state: string; txn: string }> {
    return { state: Number(opts.total) > 0 ? "approved" : "denied", txn: `PP-${opts.total}` };
  }
}

class StripeAdapter implements PaymentProcessor {
  constructor(private sdk: StripeLikeSdk) {}
  async pay(amountCents: number, currency: string): Promise<PaymentResult> {
    const r = this.sdk.charge(amountCents / 100, currency.toLowerCase());
    return { ok: r.status === "succeeded", reference: r.id };
  }
}
class PaypalAdapter implements PaymentProcessor {
  constructor(private sdk: PaypalLikeSdk) {}
  async pay(amountCents: number, currency: string): Promise<PaymentResult> {
    const r = await this.sdk.createPayment({ total: (amountCents / 100).toFixed(2), currency_code: currency.toUpperCase() });
    return { ok: r.state === "approved", reference: r.txn };
  }
}

async function checkout(p: PaymentProcessor, cents: number): Promise<void> {
  const res = await p.pay(cents, "eur");
  console.log(res.ok ? `paid: ${res.reference}` : `declined: ${res.reference}`);
}

(async () => {
  await checkout(new StripeAdapter(new StripeLikeSdk()), 1999);
  await checkout(new PaypalAdapter(new PaypalLikeSdk()), 2500);
  await checkout(new PaypalAdapter(new PaypalLikeSdk()), 0);
})();

/**
 * Title: Facade - E-commerce Order Checkout
 * Problem: Placing an order touches inventory, pricing/tax, payment and shipping services,
 *   and must roll back reservations if payment fails. Provide `OrderFacade.placeOrder`.
 * Approach: The facade orchestrates the steps and compensates on failure (release stock).
 * Complexity: O(items) per order.
 */
interface LineItem { sku: string; qty: number }

class InventoryService {
  constructor(private stock: Map<string, number>) {}
  reserve(items: LineItem[]): void {
    for (const it of items) if ((this.stock.get(it.sku) ?? 0) < it.qty) throw new Error(`out of stock: ${it.sku}`);
    for (const it of items) this.stock.set(it.sku, this.stock.get(it.sku)! - it.qty);
  }
  release(items: LineItem[]): void {
    for (const it of items) this.stock.set(it.sku, (this.stock.get(it.sku) ?? 0) + it.qty);
  }
  available(sku: string): number { return this.stock.get(sku) ?? 0; }
}
class PricingService {
  constructor(private prices: Record<string, number>, private taxRate: number) {}
  total(items: LineItem[]): number {
    const sub = items.reduce((s, it) => s + this.prices[it.sku] * it.qty, 0);
    return Math.round(sub * (1 + this.taxRate));
  }
}
class PaymentService {
  constructor(private balanceCents: number) {}
  charge(cents: number): string {
    if (cents > this.balanceCents) throw new Error("payment declined");
    this.balanceCents -= cents;
    return `pay_${cents}`;
  }
}
class ShippingService {
  private seq = 0;
  schedule(address: string): string { return `TRK-${++this.seq}-${address.slice(0, 3).toUpperCase()}`; }
}

type OrderResult = { ok: true; total: number; payment: string; tracking: string } | { ok: false; error: string };

class OrderFacade {
  constructor(
    private inventory: InventoryService,
    private pricing: PricingService,
    private payment: PaymentService,
    private shipping: ShippingService,
  ) {}
  placeOrder(items: LineItem[], address: string): OrderResult {
    try {
      this.inventory.reserve(items);
    } catch (e) {
      return { ok: false, error: (e as Error).message };
    }
    try {
      const total = this.pricing.total(items);
      const payment = this.payment.charge(total);
      return { ok: true, total, payment, tracking: this.shipping.schedule(address) };
    } catch (e) {
      this.inventory.release(items); // compensate
      return { ok: false, error: (e as Error).message };
    }
  }
}

const inv = new InventoryService(new Map([["tent", 2], ["stove", 5]]));
const shop = new OrderFacade(inv, new PricingService({ tent: 12000, stove: 3500 }, 0.2), new PaymentService(30000), new ShippingService());
console.log(shop.placeOrder([{ sku: "tent", qty: 1 }, { sku: "stove", qty: 2 }], "Lyon"));
console.log(shop.placeOrder([{ sku: "tent", qty: 1 }], "Paris"), "tents left:", inv.available("tent")); // declined, rollback -> 1
console.log(shop.placeOrder([{ sku: "tent", qty: 5 }], "Nice"));

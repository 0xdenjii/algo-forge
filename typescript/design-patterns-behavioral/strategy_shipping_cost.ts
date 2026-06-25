/**
 * Title: Strategy - Shipping Cost Calculator (function strategies)
 * Problem: Compute shipping for a parcel under several carrier pricing rules
 *   and pick the cheapest. Rules are plain functions registered by name.
 * Approach: Strategies as typed functions in a Map registry (idiomatic TS).
 * Complexity: O(k) time to compare k strategies, O(k) space.
 */
interface Parcel { weightKg: number; distanceKm: number; express: boolean }
type ShippingStrategy = (p: Parcel) => number;

const strategies = new Map<string, ShippingStrategy>([
  ["flat", () => 9.99],
  ["byWeight", (p) => 2 + p.weightKg * 1.5 + (p.express ? 10 : 0)],
  ["byDistance", (p) => 1 + p.distanceKm * 0.02 + p.weightKg * 0.5],
]);

function cheapest(p: Parcel): { carrier: string; cost: number } {
  let best = { carrier: "", cost: Infinity };
  for (const [carrier, fn] of strategies) {
    const cost = Math.round(fn(p) * 100) / 100;
    if (cost < best.cost) best = { carrier, cost };
  }
  return best;
}

console.log(cheapest({ weightKg: 1, distanceKm: 100, express: false }));  // byWeight 3.5 (ties keep the first registered)
console.log(cheapest({ weightKg: 10, distanceKm: 900, express: false })); // flat 9.99

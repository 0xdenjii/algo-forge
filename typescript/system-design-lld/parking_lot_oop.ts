/**
 * LLD - Parking Lot
 * Multi-level lot with motorcycle/compact/large spots. Vehicles park in the smallest spot that
 * fits; parking issues a ticket, leaving computes an hourly fee and frees the spot.
 * Approach: per-size free-spot pools (min level first), ticket registry, pluggable pricing.
 * Time: park/leave O(1) amortized. Space: O(spots + active tickets).
 */
export enum Size { Small = 0, Medium = 1, Large = 2 }

export interface Vehicle { plate: string; size: Size }
export interface Spot { id: string; level: number; size: Size }
export interface Ticket { id: number; vehicle: Vehicle; spot: Spot; entry: number }

export type PricingPolicy = (spot: Spot, hours: number) => number;
const defaultPricing: PricingPolicy = (spot, hours) => [1, 2, 4][spot.size] * Math.max(1, Math.ceil(hours));

export class ParkingLot {
  private free: Spot[][] = [[], [], []];
  private tickets = new Map<number, Ticket>();
  private byPlate = new Map<string, number>();
  private nextId = 1;

  constructor(levels: { small: number; medium: number; large: number }[], private pricing = defaultPricing) {
    levels.forEach((lv, li) => {
      const add = (n: number, size: Size) => {
        for (let i = 0; i < n; i++) this.free[size].push({ id: `L${li}-${Size[size]}-${i}`, level: li, size });
      };
      add(lv.small, Size.Small);
      add(lv.medium, Size.Medium);
      add(lv.large, Size.Large);
    });
    this.free.forEach(pool => pool.reverse()); // pop() yields lowest level first
  }

  park(v: Vehicle, now = Date.now()): Ticket | null {
    if (this.byPlate.has(v.plate)) throw new Error(`${v.plate} already parked`);
    for (let s = v.size; s <= Size.Large; s++) {
      const spot = this.free[s].pop();
      if (spot) {
        const t: Ticket = { id: this.nextId++, vehicle: v, spot, entry: now };
        this.tickets.set(t.id, t);
        this.byPlate.set(v.plate, t.id);
        return t;
      }
    }
    return null;
  }

  leave(ticketId: number, now = Date.now()): number {
    const t = this.tickets.get(ticketId);
    if (!t) throw new Error("invalid ticket");
    this.tickets.delete(ticketId);
    this.byPlate.delete(t.vehicle.plate);
    this.free[t.spot.size].push(t.spot);
    return this.pricing(t.spot, (now - t.entry) / 3_600_000);
  }

  available(size: Size): number {
    return this.free[size].length;
  }
}

const lot = new ParkingLot([{ small: 1, medium: 1, large: 1 }]);
const t1 = lot.park({ plate: "BIKE-1", size: Size.Small }, 0)!;
const t2 = lot.park({ plate: "CAR-1", size: Size.Small }, 0)!; // overflows into medium
console.log(t1.spot.id, t2.spot.id, lot.available(Size.Medium)); // L0-Small-0 L0-Medium-0 0
console.log(lot.leave(t2.id, 2.5 * 3_600_000)); // 6 (medium, 3 hours)

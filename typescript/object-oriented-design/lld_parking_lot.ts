/**
 * LLD - Parking Lot
 * Multi-level lot with Motorcycle/Compact/Large spots. A vehicle may park in any spot at least its size.
 * Parking issues a ticket; unparking frees the spot and computes the fee from duration and vehicle type.
 * Approach: Vehicle hierarchy, Spot objects per level, ParkingLot facade with a pluggable PricingStrategy.
 * Time: park O(total spots) linear scan (index by size for O(1)); unpark O(1). Space: O(spots + tickets).
 */
export enum SpotSize {
  Motorcycle = 0,
  Compact = 1,
  Large = 2,
}

export abstract class Vehicle {
  constructor(readonly plate: string) {}
  abstract readonly size: SpotSize;
}
export class Motorcycle extends Vehicle {
  readonly size = SpotSize.Motorcycle;
}
export class Car extends Vehicle {
  readonly size = SpotSize.Compact;
}
export class Truck extends Vehicle {
  readonly size = SpotSize.Large;
}

class Spot {
  occupant: Vehicle | null = null;
  constructor(readonly id: string, readonly size: SpotSize) {}
  fits(v: Vehicle): boolean {
    return this.occupant === null && this.size >= v.size;
  }
}

export interface Ticket {
  readonly id: number;
  readonly spotId: string;
  readonly vehicle: Vehicle;
  readonly entry: number;
}

export interface PricingStrategy {
  fee(vehicle: Vehicle, hours: number): number;
}
export const hourlyPricing: PricingStrategy = {
  fee: (v, hours) => Math.max(1, Math.ceil(hours)) * [1, 2, 4][v.size],
};

export class ParkingLot {
  private readonly spots: Spot[] = [];
  private readonly active = new Map<number, { ticket: Ticket; spot: Spot }>();
  private nextTicket = 1;

  constructor(levels: Array<Record<SpotSize, number>>, private readonly pricing: PricingStrategy = hourlyPricing) {
    levels.forEach((lvl, li) => {
      for (const size of [SpotSize.Motorcycle, SpotSize.Compact, SpotSize.Large]) {
        for (let i = 0; i < lvl[size]; i++) this.spots.push(new Spot(`L${li}-${SpotSize[size]}-${i}`, size));
      }
    });
  }

  park(v: Vehicle, now: number = Date.now()): Ticket | null {
    // spots are ordered smallest-first per level, so the first fit is the tightest on that level
    const spot = this.spots.find((s) => s.fits(v));
    if (!spot) return null;
    spot.occupant = v;
    const ticket: Ticket = { id: this.nextTicket++, spotId: spot.id, vehicle: v, entry: now };
    this.active.set(ticket.id, { ticket, spot });
    return ticket;
  }

  unpark(ticketId: number, now: number = Date.now()): number {
    const rec = this.active.get(ticketId);
    if (!rec) throw new Error(`unknown ticket ${ticketId}`);
    rec.spot.occupant = null;
    this.active.delete(ticketId);
    return this.pricing.fee(rec.ticket.vehicle, (now - rec.ticket.entry) / 3_600_000);
  }

  freeSpots(): number {
    return this.spots.filter((s) => s.occupant === null).length;
  }
}

const lot = new ParkingLot([{ [SpotSize.Motorcycle]: 1, [SpotSize.Compact]: 1, [SpotSize.Large]: 1 }]);
const t1 = lot.park(new Car("CAR-1"), 0)!;
const t2 = lot.park(new Car("CAR-2"), 0)!; // compact full -> takes large
console.log(t1.spotId, t2.spotId, lot.park(new Truck("TRK-1"), 0)); // L0-Compact-0 L0-Large-0 null
console.log(lot.unpark(t1.id, 2.5 * 3_600_000), lot.freeSpots()); // 6 2

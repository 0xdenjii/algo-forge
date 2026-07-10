/**
 * LeetCode 1603 - Design Parking System
 * A lot has a fixed number of big, medium and small slots. addCar(carType) parks a car of
 * type 1 (big), 2 (medium) or 3 (small) in a slot of the same size, returning success.
 * Approach: remaining-capacity array indexed by car type.
 * Time: O(1). Space: O(1).
 */
export const enum CarType {
  Big = 1,
  Medium = 2,
  Small = 3,
}

export class ParkingSystem {
  private readonly free: [number, number, number, number];

  constructor(big: number, medium: number, small: number) {
    this.free = [0, big, medium, small];
  }

  addCar(carType: CarType): boolean {
    if (this.free[carType] === 0) return false;
    this.free[carType]--;
    return true;
  }
}

const ps = new ParkingSystem(1, 1, 0);
console.log(ps.addCar(CarType.Big), ps.addCar(CarType.Medium), ps.addCar(CarType.Small), ps.addCar(CarType.Big)); // true true false false

/**
 * LeetCode 1603 - Design Parking System
 * Parking lot with fixed big/medium/small slot counts; addCar(type) returns whether it parked.
 * Approach: counter array indexed by car type (1 = big, 2 = medium, 3 = small).
 * Time: O(1). Space: O(1).
 */
export class ParkingSystem {
  private slots: number[];
  constructor(big: number, medium: number, small: number) {
    this.slots = [0, big, medium, small];
  }
  addCar(type: 1 | 2 | 3): boolean {
    if (this.slots[type] === 0) return false;
    this.slots[type]--;
    return true;
  }
}

const ps = new ParkingSystem(1, 1, 0);
console.log(ps.addCar(1), ps.addCar(2), ps.addCar(3), ps.addCar(1)); // true true false false

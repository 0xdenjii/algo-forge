/**
 * LLD - Elevator System (SCAN / LOOK scheduling)
 * A building with several elevators receives hall calls; each call is assigned to the elevator
 * with the lowest estimated cost. Each car serves stops in its current direction before reversing.
 * Approach: per-car sorted up/down stop sets, cost = distance with penalty for wrong direction,
 * discrete step() simulation.
 * Time: assign O(E), step O(E * S). Space: O(E * floors).
 */
type Dir = "up" | "down" | "idle";

class Elevator {
  floor = 0;
  dir: Dir = "idle";
  private stops = new Set<number>();
  constructor(readonly id: number) {}

  addStop(f: number): void {
    if (f !== this.floor) this.stops.add(f);
  }

  cost(f: number): number {
    const d = Math.abs(this.floor - f);
    if (this.dir === "idle") return d;
    const onTheWay = (this.dir === "up" && f >= this.floor) || (this.dir === "down" && f <= this.floor);
    return onTheWay ? d : d + 2 * this.stops.size + 10;
  }

  step(log: string[]): void {
    if (this.stops.size === 0) {
      this.dir = "idle";
      return;
    }
    const above = [...this.stops].some(s => s > this.floor);
    const below = [...this.stops].some(s => s < this.floor);
    if (this.dir === "idle" || (this.dir === "up" && !above) || (this.dir === "down" && !below)) {
      this.dir = above ? "up" : "down";
    }
    this.floor += this.dir === "up" ? 1 : -1;
    if (this.stops.delete(this.floor)) log.push(`E${this.id} stops at ${this.floor}`);
  }
}

export class ElevatorController {
  readonly cars: Elevator[];
  readonly log: string[] = [];
  constructor(n: number, private floors: number) {
    this.cars = Array.from({ length: n }, (_, i) => new Elevator(i));
  }
  call(floor: number): number {
    if (floor < 0 || floor >= this.floors) throw new RangeError("bad floor");
    const best = this.cars.reduce((a, b) => (b.cost(floor) < a.cost(floor) ? b : a));
    best.addStop(floor);
    return best.id;
  }
  select(car: number, floor: number): void {
    this.cars[car].addStop(floor);
  }
  step(): void {
    this.cars.forEach(c => c.step(this.log));
  }
}

const ctl = new ElevatorController(2, 10);
const a = ctl.call(5);
ctl.step();
const b = ctl.call(0); // other idle car is closer
console.log(a, b);
ctl.select(a, 8);
for (let i = 0; i < 10; i++) ctl.step();
console.log(ctl.log);

/**
 * LLD - Elevator System (SCAN / LOOK scheduling)
 * N elevators serve floors; hall calls are dispatched to the best car and each car services
 * stops in its current direction before reversing. Simulated in discrete ticks.
 * Approach: Elevator keeps up/down stop sets and moves one floor per tick (LOOK algorithm);
 * Controller dispatches each request to the car with the lowest estimated cost.
 * Time: dispatch O(N), step O(N * stops). Space: O(N * floors).
 */
type Direction = -1 | 0 | 1;

class Elevator {
  floor = 0;
  dir: Direction = 0;
  private readonly stops = new Set<number>();
  readonly log: number[] = [];

  constructor(readonly id: number) {}

  addStop(f: number): void {
    if (f !== this.floor || this.dir !== 0) this.stops.add(f);
  }

  cost(f: number): number {
    if (this.dir === 0) return Math.abs(f - this.floor);
    const ahead = (f - this.floor) * this.dir >= 0;
    return ahead ? Math.abs(f - this.floor) : Math.abs(f - this.floor) + 2 * this.stops.size + 10;
  }

  step(): void {
    if (this.stops.size === 0) {
      this.dir = 0;
      return;
    }
    const hasAhead = (d: Direction) => [...this.stops].some((s) => (s - this.floor) * d > 0);
    if (this.dir === 0 || !hasAhead(this.dir)) {
      this.dir = [...this.stops].some((s) => s > this.floor) ? 1 : -1;
    }
    this.floor += this.dir;
    if (this.stops.delete(this.floor)) {
      this.log.push(this.floor);
      if (this.stops.size === 0) this.dir = 0;
    }
  }
}

export class ElevatorController {
  readonly cars: Elevator[];
  constructor(count: number, private readonly floors: number) {
    this.cars = Array.from({ length: count }, (_, i) => new Elevator(i));
  }

  request(floor: number): number {
    if (floor < 0 || floor >= this.floors) throw new RangeError(`no floor ${floor}`);
    const car = this.cars.reduce((best, c) => (c.cost(floor) < best.cost(floor) ? c : best));
    car.addStop(floor);
    return car.id;
  }

  /** passenger inside car presses a floor button */
  press(carId: number, floor: number): void {
    this.cars[carId].addStop(floor);
  }

  tick(n = 1): void {
    for (let i = 0; i < n; i++) this.cars.forEach((c) => c.step());
  }
}

const ctl = new ElevatorController(2, 10);
const a = ctl.request(5);
ctl.tick(2); // car a is at floor 2 going up
const b = ctl.request(1); // the idle car is cheaper
ctl.press(a, 8);
ctl.press(a, 3);
ctl.tick(10);
console.log(a, b, ctl.cars[a].log, ctl.cars[b].log); // 0 1 [3, 5, 8] [1]

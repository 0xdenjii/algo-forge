/**
 * Title: LLD - Elevator System (State + Strategy)
 * Problem: Several elevators serve floors. Hall calls are dispatched to an
 *   elevator by a pluggable strategy; each elevator moves floor by floor
 *   using the SCAN (elevator) algorithm and has states Idle/MovingUp/
 *   MovingDown/DoorsOpen. Simulate in discrete steps.
 * Approach: Dispatch strategy (nearest-car cost) + per-elevator state enum
 *   with a step() transition function and up/down sorted stop sets.
 * Complexity: dispatch O(e) for e elevators; step O(stops) per elevator.
 */
type ElevatorState = "IDLE" | "UP" | "DOWN" | "DOORS_OPEN";

class Elevator {
  state: ElevatorState = "IDLE";
  floor = 0;
  readonly stops = new Set<number>();
  readonly log: string[] = [];
  constructor(readonly id: number) {}

  addStop(f: number) { if (f !== this.floor || this.state !== "DOORS_OPEN") this.stops.add(f); }

  private hasAbove() { for (const s of this.stops) if (s > this.floor) return true; return false; }
  private hasBelow() { for (const s of this.stops) if (s < this.floor) return true; return false; }

  step(): void {
    switch (this.state) {
      case "DOORS_OPEN":
      case "IDLE": {
        const prevUp = this.lastDir !== "DOWN";
        if (this.stops.delete(this.floor)) { this.open(); return; }
        if (prevUp ? this.hasAbove() : !this.hasBelow() && this.hasAbove()) this.state = "UP";
        else if (this.hasBelow()) this.state = "DOWN";
        else { this.state = "IDLE"; return; }
        this.move();
        return;
      }
      case "UP":
      case "DOWN":
        if (this.stops.delete(this.floor)) { this.open(); return; }
        if (this.state === "UP" && !this.hasAbove()) this.state = this.hasBelow() ? "DOWN" : "IDLE";
        else if (this.state === "DOWN" && !this.hasBelow()) this.state = this.hasAbove() ? "UP" : "IDLE";
        if (this.state !== "IDLE") this.move();
    }
  }
  private lastDir: "UP" | "DOWN" = "UP";
  private move() { this.lastDir = this.state as "UP" | "DOWN"; this.floor += this.state === "UP" ? 1 : -1; }
  private open() { this.state = "DOORS_OPEN"; this.log.push(`E${this.id} open@${this.floor}`); }
}

interface DispatchStrategy { choose(elevators: Elevator[], floor: number): Elevator }

const nearestCar: DispatchStrategy = {
  choose(es, floor) {
    const cost = (e: Elevator) => {
      const d = Math.abs(e.floor - floor);
      const onTheWay = (e.state === "UP" && floor >= e.floor) || (e.state === "DOWN" && floor <= e.floor);
      return e.state === "IDLE" || onTheWay ? d : d + 2 * e.stops.size + 10;
    };
    return es.reduce((best, e) => (cost(e) < cost(best) ? e : best));
  },
};

class Building {
  constructor(readonly elevators: Elevator[], private strategy: DispatchStrategy) {}
  hallCall(floor: number) { this.strategy.choose(this.elevators, floor).addStop(floor); }
  carCall(id: number, floor: number) { this.elevators[id].addStop(floor); }
  tick(n = 1) { for (let i = 0; i < n; i++) this.elevators.forEach((e) => e.step()); }
}

const b = new Building([new Elevator(0), new Elevator(1)], nearestCar);
b.elevators[1].floor = 10;
b.hallCall(2);   // E0 (closer)
b.hallCall(9);   // E1
b.tick(3);
b.carCall(0, 5);
b.carCall(1, 0);
b.tick(15);
b.elevators.forEach((e) => console.log(e.log.join(", "), "| now", e.state, "at", e.floor));

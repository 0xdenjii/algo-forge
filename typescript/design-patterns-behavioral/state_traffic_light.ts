/**
 * Title: State - Traffic Light Controller (table-driven)
 * Problem: Model a traffic light cycling GREEN -> YELLOW -> RED -> GREEN with
 *   per-state durations, plus a pedestrian button that shortens GREEN and a
 *   fault mode that flashes YELLOW until reset.
 * Approach: Discriminated-union states + a transition table; tick(dt) drives time.
 * Complexity: O(1) per tick/event; O(1) space.
 */
type Light = "GREEN" | "YELLOW" | "RED" | "FLASHING";
type Event = "TIMER" | "PED_BUTTON" | "FAULT" | "RESET";

const durations: Record<Light, number> = { GREEN: 30, YELLOW: 4, RED: 20, FLASHING: Infinity };

const transitions: Record<Light, Partial<Record<Event, Light>>> = {
  GREEN: { TIMER: "YELLOW", FAULT: "FLASHING" },
  YELLOW: { TIMER: "RED", FAULT: "FLASHING" },
  RED: { TIMER: "GREEN", FAULT: "FLASHING" },
  FLASHING: { RESET: "RED" },
};

class TrafficLight {
  private elapsed = 0;
  readonly history: Light[] = [];
  constructor(private state: Light = "RED") { this.history.push(state); }

  get current(): Light { return this.state; }

  send(e: Event): void {
    if (e === "PED_BUTTON") {
      // Shorten remaining green to at most 5 s.
      if (this.state === "GREEN") this.elapsed = Math.max(this.elapsed, durations.GREEN - 5);
      return;
    }
    const next = transitions[this.state][e];
    if (!next) return; // event ignored in this state
    this.state = next;
    this.elapsed = 0;
    this.history.push(next);
  }

  tick(dt: number): void {
    this.elapsed += dt;
    while (this.elapsed >= durations[this.state]) {
      const over = this.elapsed - durations[this.state];
      this.send("TIMER");
      this.elapsed = over;
    }
  }
}

const tl = new TrafficLight();
tl.tick(20);              // RED -> GREEN
tl.tick(10);
tl.send("PED_BUTTON");    // green ends in 5s
tl.tick(5);               // GREEN -> YELLOW
tl.tick(4);               // YELLOW -> RED
tl.send("FAULT");
tl.tick(1000);            // stays flashing
tl.send("RESET");
console.log(tl.history.join(" -> ")); // RED -> GREEN -> YELLOW -> RED -> FLASHING -> RED

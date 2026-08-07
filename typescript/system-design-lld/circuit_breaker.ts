/**
 * LLD - Circuit Breaker
 * Wrap calls to an unreliable dependency. After N consecutive failures the breaker OPENs and
 * fails fast; after a cooldown it goes HALF_OPEN and lets one trial call through, closing on
 * success or re-opening on failure.
 * Approach: explicit state machine with injectable clock for testability.
 * Time: O(1) overhead per call. Space: O(1).
 */
type BreakerState = "CLOSED" | "OPEN" | "HALF_OPEN";

export class CircuitOpenError extends Error {}

export class CircuitBreaker {
  state: BreakerState = "CLOSED";
  private failures = 0;
  private openedAt = 0;
  private trialInFlight = false;

  constructor(
    private threshold: number,
    private cooldownMs: number,
    private now: () => number = Date.now,
  ) {}

  async call<T>(fn: () => Promise<T>): Promise<T> {
    if (this.state === "OPEN") {
      if (this.now() - this.openedAt < this.cooldownMs) throw new CircuitOpenError("circuit open");
      this.state = "HALF_OPEN";
    }
    if (this.state === "HALF_OPEN") {
      if (this.trialInFlight) throw new CircuitOpenError("trial in progress");
      this.trialInFlight = true;
    }
    try {
      const r = await fn();
      this.failures = 0;
      this.state = "CLOSED";
      return r;
    } catch (e) {
      this.failures++;
      if (this.state === "HALF_OPEN" || this.failures >= this.threshold) {
        this.state = "OPEN";
        this.openedAt = this.now();
      }
      throw e;
    } finally {
      this.trialInFlight = false;
    }
  }
}

(async () => {
  let t = 0;
  const cb = new CircuitBreaker(2, 1000, () => t);
  const fail = () => Promise.reject(new Error("boom"));
  for (let i = 0; i < 3; i++) await cb.call(fail).catch(e => console.log(cb.state, e.constructor.name));
  t = 1500;
  console.log(await cb.call(async () => "ok"), cb.state); // ok CLOSED
})();

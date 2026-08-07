/**
 * LeetCode 1396 - Design Underground System
 * Record check-ins and check-outs and report the average travel time between two stations.
 * Approach: active trips map id -> [station, t]; route stats map "a->b" -> [sum, count].
 * Time: O(1) per op. Space: O(passengers + routes).
 */
export class UndergroundSystem {
  private active = new Map<number, [string, number]>();
  private stats = new Map<string, [number, number]>();

  checkIn(id: number, station: string, t: number): void {
    this.active.set(id, [station, t]);
  }

  checkOut(id: number, station: string, t: number): void {
    const trip = this.active.get(id);
    if (!trip) throw new Error(`passenger ${id} is not checked in`);
    this.active.delete(id);
    const key = `${trip[0]}->${station}`;
    const s = this.stats.get(key) ?? [0, 0];
    this.stats.set(key, [s[0] + t - trip[1], s[1] + 1]);
  }

  getAverageTime(a: string, b: string): number {
    const s = this.stats.get(`${a}->${b}`);
    if (!s) throw new Error("no trips on this route");
    return s[0] / s[1];
  }
}

const us = new UndergroundSystem();
us.checkIn(1, "A", 3);
us.checkIn(2, "A", 8);
us.checkOut(1, "B", 15);
us.checkOut(2, "B", 20);
console.log(us.getAverageTime("A", "B")); // 12

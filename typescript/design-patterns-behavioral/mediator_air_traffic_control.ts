/**
 * Title: Mediator - Air Traffic Control Tower
 * Problem: Aircraft request landing on a limited set of runways. Planes never
 *   talk to each other; the tower grants runways, queues others by priority
 *   (emergencies first, then request order) and reassigns freed runways.
 * Approach: Tower mediator with a free-runway list and a priority queue.
 * Complexity: request/release O(log q) with a binary heap; O(q + r) space.
 */
class MinHeap<T> {
  private a: T[] = [];
  constructor(private less: (x: T, y: T) => boolean) {}
  get size() { return this.a.length; }
  push(x: T) {
    const a = this.a; a.push(x);
    for (let i = a.length - 1; i > 0;) {
      const p = (i - 1) >> 1;
      if (!this.less(a[i], a[p])) break;
      [a[i], a[p]] = [a[p], a[i]]; i = p;
    }
  }
  pop(): T | undefined {
    const a = this.a; if (!a.length) return undefined;
    const top = a[0], last = a.pop()!;
    if (a.length) {
      a[0] = last;
      for (let i = 0; ;) {
        const l = 2 * i + 1, r = l + 1; let m = i;
        if (l < a.length && this.less(a[l], a[m])) m = l;
        if (r < a.length && this.less(a[r], a[m])) m = r;
        if (m === i) break;
        [a[i], a[m]] = [a[m], a[i]]; i = m;
      }
    }
    return top;
  }
}

class Aircraft {
  readonly messages: string[] = [];
  runway: string | null = null;
  constructor(readonly callsign: string, readonly emergency = false) {}
  notify(msg: string) { this.messages.push(msg); }
}

type Waiting = { plane: Aircraft; seq: number };

class Tower {
  private free: string[];
  private seq = 0;
  private queue = new MinHeap<Waiting>((x, y) =>
    x.plane.emergency !== y.plane.emergency ? x.plane.emergency : x.seq < y.seq);
  constructor(runways: string[]) { this.free = [...runways]; }

  requestLanding(p: Aircraft) {
    const rw = this.free.shift();
    if (rw) this.assign(p, rw);
    else { this.queue.push({ plane: p, seq: this.seq++ }); p.notify("hold, queued"); }
  }
  landed(p: Aircraft) {
    if (!p.runway) throw new Error(`${p.callsign} holds no runway`);
    const rw = p.runway; p.runway = null;
    const next = this.queue.pop();
    if (next) this.assign(next.plane, rw); else this.free.push(rw);
  }
  private assign(p: Aircraft, rw: string) { p.runway = rw; p.notify(`cleared to land ${rw}`); }
}

const tower = new Tower(["27L"]);
const a = new Aircraft("AF1"), b = new Aircraft("BA2"), c = new Aircraft("LH3", true);
tower.requestLanding(a);
tower.requestLanding(b);
tower.requestLanding(c);
tower.landed(a);  // emergency LH3 jumps ahead of BA2
tower.landed(c);
for (const p of [a, b, c]) console.log(p.callsign, p.messages);

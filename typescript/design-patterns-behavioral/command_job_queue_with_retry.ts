/**
 * Title: Command - Async Job Queue with Retry and Concurrency Limit
 * Problem: Encapsulate units of work as job objects so they can be queued,
 *   run with at most N in parallel, retried with exponential backoff on
 *   failure, and reported on afterwards.
 * Approach: Job command objects with run(); a worker-pool runner pulls from
 *   a shared queue. The sleep function is injectable for tests.
 * Complexity: O(j * attempts) runs; O(j) space for results.
 */
interface Job<T = unknown> { readonly id: string; run(): Promise<T> }
type Result = { id: string; ok: boolean; attempts: number; value?: unknown; error?: string };

class JobQueue {
  private queue: Job[] = [];
  constructor(
    private concurrency = 2,
    private maxAttempts = 3,
    private baseDelayMs = 10,
    private sleep = (ms: number) => new Promise<void>((r) => setTimeout(r, ms)),
  ) {}

  add(job: Job): this { this.queue.push(job); return this; }

  private async exec(job: Job): Promise<Result> {
    for (let attempt = 1; ; attempt++) {
      try {
        return { id: job.id, ok: true, attempts: attempt, value: await job.run() };
      } catch (e) {
        if (attempt >= this.maxAttempts) return { id: job.id, ok: false, attempts: attempt, error: String((e as Error).message ?? e) };
        await this.sleep(this.baseDelayMs * 2 ** (attempt - 1));
      }
    }
  }

  async drain(): Promise<Result[]> {
    const results: Result[] = [];
    const worker = async () => {
      for (let job = this.queue.shift(); job; job = this.queue.shift()) results.push(await this.exec(job));
    };
    await Promise.all(Array.from({ length: Math.max(1, this.concurrency) }, worker));
    return results;
  }
}

const flaky = (id: string, failTimes: number): Job => {
  let calls = 0;
  return { id, run: async () => { if (calls++ < failTimes) throw new Error(`${id} transient`); return `${id} done`; } };
};

(async () => {
  const q = new JobQueue(2, 3, 1)
    .add(flaky("email", 0))
    .add(flaky("thumbnail", 2))
    .add(flaky("webhook", 5))
    .add({ id: "sum", run: async () => [1, 2, 3].reduce((a, b) => a + b) });
  for (const r of await q.drain()) console.log(r);
})();

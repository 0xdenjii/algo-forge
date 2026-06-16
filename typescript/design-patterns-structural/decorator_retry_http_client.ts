/**
 * Title: Decorator - Resilient HTTP Client (retry with backoff, timeout, auth header)
 * Problem: An HttpClient interface `get(url)` must gain retries with exponential backoff,
 *   per-request timeouts and auth headers, each as an independent, stackable layer.
 * Approach: Each decorator implements HttpClient and wraps another one. A flaky fake
 *   transport is used for the demo so it runs offline.
 * Complexity: O(retries) attempts per request; O(1) extra space.
 */
interface Request { url: string; headers: Record<string, string> }
interface Response { status: number; body: string }
interface HttpClient { send(req: Request): Promise<Response> }

const sleep = (ms: number) => new Promise<void>((r) => setTimeout(r, ms));

class FlakyTransport implements HttpClient {
  calls = 0;
  constructor(private failuresBeforeSuccess: number) {}
  async send(req: Request): Promise<Response> {
    this.calls++;
    if (this.calls <= this.failuresBeforeSuccess) return { status: 503, body: "unavailable" };
    return { status: 200, body: `ok ${req.url} auth=${req.headers["Authorization"] ?? "none"}` };
  }
}

class AuthDecorator implements HttpClient {
  constructor(private inner: HttpClient, private token: string) {}
  send(req: Request): Promise<Response> {
    return this.inner.send({ ...req, headers: { ...req.headers, Authorization: `Bearer ${this.token}` } });
  }
}

class TimeoutDecorator implements HttpClient {
  constructor(private inner: HttpClient, private ms: number) {}
  send(req: Request): Promise<Response> {
    let timer: ReturnType<typeof setTimeout> | undefined;
    const timeout = new Promise<never>((_, rej) => { timer = setTimeout(() => rej(new Error("timeout")), this.ms); });
    return Promise.race([this.inner.send(req), timeout]).finally(() => clearTimeout(timer));
  }
}

class RetryDecorator implements HttpClient {
  constructor(private inner: HttpClient, private maxRetries: number, private baseDelayMs: number) {}
  async send(req: Request): Promise<Response> {
    for (let attempt = 0; ; attempt++) {
      try {
        const res = await this.inner.send(req);
        if (res.status < 500 || attempt >= this.maxRetries) return res;
      } catch (e) {
        if (attempt >= this.maxRetries) throw e;
      }
      await sleep(this.baseDelayMs * 2 ** attempt);
    }
  }
}

(async () => {
  const transport = new FlakyTransport(2);
  const client = new RetryDecorator(new TimeoutDecorator(new AuthDecorator(transport, "s3cr3t"), 500), 3, 10);
  console.log(await client.send({ url: "/api/items", headers: {} }), "attempts:", transport.calls);

  const hopeless = new FlakyTransport(10);
  const res = await new RetryDecorator(hopeless, 2, 5).send({ url: "/x", headers: {} });
  console.log(res.status, "after", hopeless.calls, "attempts"); // 503 after 3
})();

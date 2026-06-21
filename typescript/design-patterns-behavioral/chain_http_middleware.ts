/**
 * Title: Chain of Responsibility - Async HTTP Middleware Pipeline (Koa-style)
 * Problem: Compose middleware (logging, auth, rate limiting, routing) where
 *   each can act before and after the next, or short-circuit the request.
 *   Calling next() twice in one middleware is an error.
 * Approach: compose() builds a dispatcher with an index guard ("onion" model).
 * Complexity: O(m) per request for m middlewares; O(m) call depth.
 */
interface Ctx { path: string; headers: Record<string, string>; status: number; body: string; trace: string[] }
type Next = () => Promise<void>;
type Middleware = (ctx: Ctx, next: Next) => Promise<void>;

function compose(mws: Middleware[]): (ctx: Ctx) => Promise<void> {
  return (ctx) => {
    let last = -1;
    const dispatch = async (i: number): Promise<void> => {
      if (i <= last) throw new Error("next() called multiple times");
      last = i;
      const mw = mws[i];
      if (mw) await mw(ctx, () => dispatch(i + 1));
    };
    return dispatch(0);
  };
}

const logger: Middleware = async (ctx, next) => {
  ctx.trace.push(`-> ${ctx.path}`);
  await next();
  ctx.trace.push(`<- ${ctx.status}`);
};
const auth: Middleware = async (ctx, next) => {
  if (ctx.headers.authorization !== "Bearer ok") { ctx.status = 401; ctx.body = "unauthorized"; return; }
  await next();
};
const router: Middleware = async (ctx) => {
  if (ctx.path === "/hello") { ctx.status = 200; ctx.body = "hi"; }
  else { ctx.status = 404; ctx.body = "not found"; }
};

const app = compose([logger, auth, router]);
const mk = (path: string, token?: string): Ctx =>
  ({ path, headers: token ? { authorization: `Bearer ${token}` } : {}, status: 0, body: "", trace: [] });

(async () => {
  for (const ctx of [mk("/hello", "ok"), mk("/hello"), mk("/nope", "ok")]) {
    await app(ctx);
    console.log(ctx.status, ctx.body, ctx.trace);
  }
})();

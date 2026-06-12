/**
 * Title: Adapter - Node-style Callback to Promise (promisify)
 * Problem: Legacy APIs use `(args..., cb(err, result))`. Write a generic, type-safe adapter
 *   that turns any such function into one returning a Promise.
 * Approach: Variadic tuple types infer argument and result types; wrap in `new Promise`.
 * Complexity: O(1) overhead per call.
 */
type Callback<R> = (err: Error | null, result?: R) => void;

function promisify<A extends unknown[], R>(fn: (...args: [...A, Callback<R>]) => void): (...args: A) => Promise<R> {
  return (...args: A) =>
    new Promise<R>((resolve, reject) => {
      fn(...args, (err: Error | null, result?: R) => (err ? reject(err) : resolve(result as R)));
    });
}

// Legacy API
function readConfig(name: string, strict: boolean, cb: Callback<Record<string, string>>): void {
  setTimeout(() => {
    if (strict && name !== "app") cb(new Error(`unknown config ${name}`));
    else cb(null, { name, env: "prod" });
  }, 5);
}

const readConfigAsync = promisify(readConfig);
(async () => {
  console.log(await readConfigAsync("app", true));
  try {
    await readConfigAsync("db", true);
  } catch (e) {
    console.log("error:", (e as Error).message);
  }
})();

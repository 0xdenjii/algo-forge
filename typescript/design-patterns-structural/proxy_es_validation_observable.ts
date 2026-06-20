/**
 * Title: Proxy - ES Proxy for Validation and Change Tracking
 * Problem: Wrap a plain settings object so that (1) writes are validated against a schema,
 *   (2) unknown keys are rejected, and (3) observers are notified of changes, including
 *   changes to nested objects - without the caller using a special API.
 * Approach: Built-in `Proxy` with get/set/deleteProperty traps; nested objects are wrapped
 *   lazily on access and cached in a WeakMap so identity stays stable.
 * Complexity: O(1) per trap plus O(path length) to build change paths.
 */
type Validator = (v: unknown) => boolean;
type Listener = (path: string, oldValue: unknown, newValue: unknown) => void;

function observable<T extends object>(target: T, schema: Record<string, Validator>, onChange: Listener): T {
  const cache = new WeakMap<object, unknown>();
  const wrap = <O extends object>(obj: O, base: string): O => {
    if (cache.has(obj)) return cache.get(obj) as O;
    const proxy = new Proxy(obj, {
      get(t, key, recv) {
        const v = Reflect.get(t, key, recv);
        return typeof key === "string" && v !== null && typeof v === "object" ? wrap(v as object, `${base}${key}.`) : v;
      },
      set(t, key, value, recv) {
        if (typeof key !== "string") return Reflect.set(t, key, value, recv);
        const path = base + key;
        const validate = schema[path];
        if (!validate) throw new TypeError(`unknown setting "${path}"`);
        if (!validate(value)) throw new TypeError(`invalid value for "${path}": ${JSON.stringify(value)}`);
        const old = Reflect.get(t, key, recv);
        const ok = Reflect.set(t, key, value, recv);
        if (old !== value) onChange(path, old, value);
        return ok;
      },
      deleteProperty(_t, key) {
        throw new TypeError(`cannot delete setting "${base}${String(key)}"`);
      },
    });
    cache.set(obj, proxy);
    return proxy;
  };
  return wrap(target, "");
}

const settings = observable(
  { theme: "dark", fontSize: 14, editor: { tabSize: 2, wordWrap: false } },
  {
    theme: (v) => v === "dark" || v === "light",
    fontSize: (v) => typeof v === "number" && v >= 8 && v <= 72,
    "editor.tabSize": (v) => v === 2 || v === 4 || v === 8,
    "editor.wordWrap": (v) => typeof v === "boolean",
  },
  (path, o, n) => console.log(`changed ${path}: ${JSON.stringify(o)} -> ${JSON.stringify(n)}`),
);

settings.theme = "light";
settings.editor.tabSize = 4;
for (const attempt of [() => { settings.fontSize = 200; }, () => { (settings as Record<string, unknown>).color = "red"; }]) {
  try { attempt(); } catch (e) { console.log("rejected:", (e as Error).message); }
}
console.log(settings.editor === settings.editor, JSON.stringify(settings)); // true, stable identity

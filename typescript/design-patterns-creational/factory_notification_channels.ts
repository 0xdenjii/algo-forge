/**
 * Title: Factory (Notification Channels) - classic LLD exercise
 * Problem: Send notifications via Email, SMS or Push based on user preference;
 *   each channel validates its recipient format. Adding a channel must not change callers.
 * Approach: Notifier interface + factory map keyed by channel type (keyof gives type safety).
 * Complexity: O(1) creation.
 */
interface Notifier { validate(to: string): boolean; send(to: string, msg: string): string }
const notifiers = {
  email: (): Notifier => ({ validate: (t) => /^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(t), send: (t, m) => `EMAIL to ${t}: ${m}` }),
  sms: (): Notifier => ({ validate: (t) => /^\+\d{8,15}$/.test(t), send: (t, m) => `SMS to ${t}: ${m.slice(0, 160)}` }),
  push: (): Notifier => ({ validate: (t) => /^[a-f0-9]{32}$/.test(t), send: (t, m) => `PUSH to ${t.slice(0, 6)}...: ${m}` }),
} as const;
type Channel = keyof typeof notifiers;

export function notify(channel: Channel, to: string, msg: string): string {
  const n = notifiers[channel]();
  if (!n.validate(to)) throw new Error(`invalid ${channel} recipient: ${to}`);
  return n.send(to, msg);
}

console.log(notify("email", "a@b.co", "Hi"));
console.log(notify("sms", "+33612345678", "Code 1234"));
console.log(notify("push", "0123456789abcdef0123456789abcdef", "Ping"));
try { notify("sms", "123", "x"); } catch (e) { console.log((e as Error).message); }

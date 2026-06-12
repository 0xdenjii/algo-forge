/**
 * Title: Bridge - Notifications x Channels
 * Problem: Notification kinds (Alert, Digest) must be sendable over channels (Email, SMS,
 *   Slack). Each kind formats differently; each channel has constraints (SMS max 160 chars).
 * Approach: The Notification abstraction owns a Channel implementor; formatting lives in
 *   the abstraction, transport and constraints in the implementor.
 * Complexity: O(len) per send.
 */
interface Channel { send(to: string, subject: string, body: string): string }

class EmailChannel implements Channel {
  send(to: string, subject: string, body: string): string { return `EMAIL to=${to} subject="${subject}"\n${body}`; }
}
class SmsChannel implements Channel {
  send(to: string, subject: string, body: string): string {
    const text = `${subject}: ${body}`.replace(/\s+/g, " ");
    return `SMS to=${to} "${text.length > 160 ? text.slice(0, 157) + "..." : text}"`;
  }
}
class SlackChannel implements Channel {
  send(to: string, subject: string, body: string): string {
    return `SLACK #${to} *${subject}*\n> ${body.split("\n").join("\n> ")}`;
  }
}

abstract class Notification {
  constructor(protected channel: Channel) {}
  abstract notify(to: string): string;
}
class Alert extends Notification {
  constructor(ch: Channel, private message: string, private severity: "low" | "high") { super(ch); }
  notify(to: string): string { return this.channel.send(to, `[${this.severity.toUpperCase()}] Alert`, this.message); }
}
class Digest extends Notification {
  constructor(ch: Channel, private items: string[]) { super(ch); }
  notify(to: string): string {
    return this.channel.send(to, `Daily digest (${this.items.length})`, this.items.map((i) => `- ${i}`).join("\n"));
  }
}

console.log(new Alert(new SmsChannel(), "CPU at 97% on web-3", "high").notify("+15550100"));
console.log(new Digest(new SlackChannel(), ["3 deploys", "0 incidents"]).notify("ops"));
console.log(new Digest(new EmailChannel(), ["PR #12 merged"]).notify("dev@example.com"));

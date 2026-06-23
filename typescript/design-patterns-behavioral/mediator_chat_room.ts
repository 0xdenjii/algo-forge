/**
 * Title: Mediator - Chat Room
 * Problem: Users send broadcast and direct messages and can mute others.
 *   Users must not reference each other directly; all routing, muting and
 *   join/leave notices live in one place.
 * Approach: ChatRoom mediator owns membership and mute lists; User only
 *   knows the room.
 * Complexity: broadcast O(u) for u users; DM O(1); O(u + mutes) space.
 */
class User {
  readonly inbox: string[] = [];
  room: ChatRoom | null = null;
  constructor(readonly name: string) {}
  say(text: string) { this.requireRoom().broadcast(this, text); }
  dm(to: string, text: string) { this.requireRoom().direct(this, to, text); }
  mute(name: string) { this.requireRoom().mute(this, name); }
  receive(msg: string) { this.inbox.push(msg); }
  private requireRoom(): ChatRoom { if (!this.room) throw new Error(`${this.name} not in a room`); return this.room; }
}

class ChatRoom {
  private users = new Map<string, User>();
  private muted = new Map<string, Set<string>>(); // listener -> muted senders

  join(u: User) {
    if (this.users.has(u.name)) throw new Error(`name ${u.name} taken`);
    this.system(`${u.name} joined`);
    this.users.set(u.name, u);
    u.room = this;
  }
  leave(u: User) { this.users.delete(u.name); this.muted.delete(u.name); u.room = null; this.system(`${u.name} left`); }
  mute(by: User, target: string) {
    if (!this.muted.has(by.name)) this.muted.set(by.name, new Set());
    this.muted.get(by.name)!.add(target);
  }
  broadcast(from: User, text: string) {
    for (const u of this.users.values()) {
      if (u !== from && !this.muted.get(u.name)?.has(from.name)) u.receive(`${from.name}: ${text}`);
    }
  }
  direct(from: User, to: string, text: string) {
    const target = this.users.get(to);
    if (!target) { from.receive(`[system] ${to} is not here`); return; }
    if (!this.muted.get(to)?.has(from.name)) target.receive(`(dm) ${from.name}: ${text}`);
  }
  private system(text: string) { for (const u of this.users.values()) u.receive(`[system] ${text}`); }
}

const room = new ChatRoom();
const [ana, bob, cy] = ["ana", "bob", "cy"].map((n) => new User(n));
[ana, bob, cy].forEach((u) => room.join(u));
cy.mute("bob");
bob.say("hello all");
ana.dm("cy", "psst");
ana.dm("dan", "hi?");
room.leave(bob);
console.log({ ana: ana.inbox, bob: bob.inbox, cy: cy.inbox });

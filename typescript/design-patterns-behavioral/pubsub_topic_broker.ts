/**
 * Title: Publish/Subscribe - Topic Broker with Wildcards
 * Problem: Publishers send to dot-separated topics ("orders.eu.created").
 *   Subscribers use patterns where `*` matches one segment and `#` matches
 *   zero or more trailing segments (AMQP-style). Publishers and subscribers
 *   never know each other.
 * Approach: Trie of topic segments; matching walks literal, `*` and `#`
 *   children. Subscriptions return a disposer.
 * Complexity: publish O(matched trie paths), subscribe O(segments).
 */
type Handler = (topic: string, payload: unknown) => void;

class TrieNode {
  children = new Map<string, TrieNode>();
  handlers = new Set<Handler>();
}

class Broker {
  private root = new TrieNode();

  subscribe(pattern: string, h: Handler): () => void {
    const segs = pattern.split(".");
    const hashAt = segs.indexOf("#");
    if (hashAt !== -1 && hashAt !== segs.length - 1) throw new Error("# only allowed as last segment");
    let n = this.root;
    for (const s of segs) {
      if (!n.children.has(s)) n.children.set(s, new TrieNode());
      n = n.children.get(s)!;
    }
    n.handlers.add(h);
    return () => { n.handlers.delete(h); };
  }

  publish(topic: string, payload: unknown): number {
    const segs = topic.split(".");
    const hit = new Set<Handler>();
    const walk = (n: TrieNode, i: number) => {
      const hash = n.children.get("#");
      if (hash) hash.handlers.forEach((h) => hit.add(h)); // # matches the rest (incl. empty)
      if (i === segs.length) { n.handlers.forEach((h) => hit.add(h)); return; }
      const lit = n.children.get(segs[i]);
      if (lit) walk(lit, i + 1);
      const star = n.children.get("*");
      if (star) walk(star, i + 1);
    };
    walk(this.root, 0);
    hit.forEach((h) => h(topic, payload));
    return hit.size;
  }
}

const broker = new Broker();
const got: string[] = [];
broker.subscribe("orders.*.created", (t) => got.push(`A ${t}`));
broker.subscribe("orders.#", (t) => got.push(`B ${t}`));
const off = broker.subscribe("orders.eu.created", (t) => got.push(`C ${t}`));
broker.publish("orders.eu.created", { id: 1 }); // A, B, C
broker.publish("orders.us.shipped", { id: 2 }); // B
off();
broker.publish("orders.eu.created", { id: 3 }); // A, B
broker.publish("users.new", {});                // none
console.log(got);

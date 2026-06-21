/**
 * Title: Chain of Responsibility - Support Ticket Escalation
 * Problem: Tickets go to a bot first, then tier-1 agents, then engineers,
 *   then a manager. Each level handles tickets it is able to resolve and
 *   escalates the rest; unhandled tickets are reported.
 * Approach: Handlers as functions returning a result or undefined; a chain
 *   runner tries them in order (functional CoR, idiomatic TS).
 * Complexity: O(h) per ticket for h handlers; O(h) space.
 */
type Priority = "low" | "medium" | "high" | "critical";
interface Ticket { id: number; topic: string; priority: Priority }
type Handler = (t: Ticket) => string | undefined;

const bot: Handler = (t) => (t.topic === "password" ? `bot reset password for #${t.id}` : undefined);
const tier1: Handler = (t) => (t.priority === "low" || t.priority === "medium" ? `tier1 solved #${t.id}` : undefined);
const engineer: Handler = (t) => (t.priority === "high" && t.topic !== "billing" ? `engineer fixed #${t.id}` : undefined);
const manager: Handler = (t) => (t.priority !== "critical" ? `manager handled #${t.id}` : undefined);

function chain(...handlers: Handler[]): (t: Ticket) => string {
  return (t) => {
    for (const h of handlers) {
      const r = h(t);
      if (r !== undefined) return r;
    }
    return `UNHANDLED #${t.id}: page on-call`;
  };
}

const support = chain(bot, tier1, engineer, manager);
const tickets: Ticket[] = [
  { id: 1, topic: "password", priority: "high" },
  { id: 2, topic: "ui", priority: "low" },
  { id: 3, topic: "crash", priority: "high" },
  { id: 4, topic: "billing", priority: "high" },
  { id: 5, topic: "outage", priority: "critical" },
];
tickets.forEach((t) => console.log(support(t)));

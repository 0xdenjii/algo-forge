/**
 * Title: Type-safe Step Builder (Email)
 * Problem: An Email requires from, to and subject in that order before optional
 *   body/cc and build(); skipping a required step must be a compile error.
 * Approach: each step returns a narrower interface, so the type system enforces order.
 * Complexity: O(1) per step.
 */
interface Email { from: string; to: string[]; subject: string; body: string; cc: string[] }
interface FromStep { from(addr: string): ToStep }
interface ToStep { to(...addrs: string[]): SubjectStep }
interface SubjectStep { subject(s: string): OptionalStep }
interface OptionalStep { body(b: string): OptionalStep; cc(...a: string[]): OptionalStep; build(): Email }

export function emailBuilder(): FromStep {
  const e: Email = { from: "", to: [], subject: "", body: "", cc: [] };
  const opt: OptionalStep = {
    body(b) { e.body = b; return opt; },
    cc(...a) { e.cc.push(...a); return opt; },
    build: () => ({ ...e, to: [...e.to], cc: [...e.cc] }),
  };
  const subjectStep: SubjectStep = { subject: (s) => { e.subject = s; return opt; } };
  const toStep: ToStep = {
    to: (...addrs) => {
      if (!addrs.length) throw new Error("need at least one recipient");
      e.to = addrs;
      return subjectStep;
    },
  };
  return { from: (addr) => { e.from = addr; return toStep; } };
}

console.log(emailBuilder().from("a@x.io").to("b@x.io").subject("Hi").body("Hello").cc("c@x.io").build());
// emailBuilder().subject("x")  -> compile error: Property 'subject' does not exist on type 'FromStep'

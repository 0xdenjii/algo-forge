/**
 * Title: Visitor - Shapes via Discriminated Unions and Exhaustive match
 * Problem: Compute area, perimeter and SVG output for circles, rectangles
 *   and triangles. Adding a new shape must cause compile errors wherever an
 *   operation forgot to handle it.
 * Approach: TypeScript-idiomatic visitor: a tagged union plus a generic
 *   match() taking a handler object keyed by tag (mapped type => exhaustive).
 * Complexity: O(1) per shape per operation.
 */
type Shape =
  | { kind: "circle"; r: number }
  | { kind: "rect"; w: number; h: number }
  | { kind: "triangle"; a: number; b: number; c: number };

type Handlers<R> = { [K in Shape["kind"]]: (s: Extract<Shape, { kind: K }>) => R };

function match<R>(s: Shape, h: Handlers<R>): R {
  return (h[s.kind] as (s: Shape) => R)(s);
}

const area = (s: Shape) => match(s, {
  circle: ({ r }) => Math.PI * r * r,
  rect: ({ w, h }) => w * h,
  triangle: ({ a, b, c }) => { const p = (a + b + c) / 2; return Math.sqrt(p * (p - a) * (p - b) * (p - c)); },
});

const perimeter = (s: Shape) => match(s, {
  circle: ({ r }) => 2 * Math.PI * r,
  rect: ({ w, h }) => 2 * (w + h),
  triangle: ({ a, b, c }) => a + b + c,
});

const toSvg = (s: Shape) => match(s, {
  circle: ({ r }) => `<circle r="${r}"/>`,
  rect: ({ w, h }) => `<rect width="${w}" height="${h}"/>`,
  triangle: ({ a }) => `<!-- triangle base ${a} -->`,
});

// Classic switch with a never check, for comparison.
function describe(s: Shape): string {
  switch (s.kind) {
    case "circle": return `circle r=${s.r}`;
    case "rect": return `rect ${s.w}x${s.h}`;
    case "triangle": return `triangle ${s.a},${s.b},${s.c}`;
    default: { const _exhaustive: never = s; return _exhaustive; }
  }
}

const shapes: Shape[] = [{ kind: "circle", r: 1 }, { kind: "rect", w: 2, h: 3 }, { kind: "triangle", a: 3, b: 4, c: 5 }];
for (const s of shapes) {
  console.log(describe(s), area(s).toFixed(2), perimeter(s).toFixed(2), toSvg(s));
}

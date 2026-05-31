/**
 * Title: Simple Factory (Shapes)
 * Problem: Create Circle/Rectangle/Triangle from a descriptor and compute
 *   areas without clients knowing concrete classes.
 * Approach: one factory function switching on a discriminated union; exhaustive check via never.
 * Complexity: O(1) per creation.
 */
interface Shape { area(): number; name: string }
type ShapeSpec =
  | { kind: "circle"; r: number }
  | { kind: "rect"; w: number; h: number }
  | { kind: "triangle"; b: number; h: number };

export function createShape(spec: ShapeSpec): Shape {
  switch (spec.kind) {
    case "circle": return { name: "circle", area: () => Math.PI * spec.r ** 2 };
    case "rect": return { name: "rect", area: () => spec.w * spec.h };
    case "triangle": return { name: "triangle", area: () => (spec.b * spec.h) / 2 };
    default: { const never: never = spec; throw new Error(`unknown ${JSON.stringify(never)}`); }
  }
}

for (const s of [createShape({ kind: "circle", r: 1 }), createShape({ kind: "rect", w: 2, h: 3 })])
  console.log(s.name, s.area().toFixed(2));

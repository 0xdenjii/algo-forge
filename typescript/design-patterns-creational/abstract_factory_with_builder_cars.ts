/**
 * Title: Abstract Factory + Builder (Vehicle Manufacturing)
 * Problem: Assemble cars where a regional parts factory (EU/US) supplies matching
 *   parts (engine, safety kit, speedometer units) and a builder assembles the Car.
 * Approach: combine patterns: factory chooses the part family, builder sequences assembly.
 * Complexity: O(p) for p parts.
 */
interface Engine { type: string; hp: number }
interface SafetyKit { items: string[] }
interface PartsFactory { engine(): Engine; safety(): SafetyKit; units: "km/h" | "mph" }
const EU: PartsFactory = { engine: () => ({ type: "diesel", hp: 150 }), safety: () => ({ items: ["warning triangle", "hi-vis vest"] }), units: "km/h" };
const US: PartsFactory = { engine: () => ({ type: "V6 gasoline", hp: 280 }), safety: () => ({ items: ["airbags"] }), units: "mph" };

interface Car { model: string; color: string; engine: Engine; safety: SafetyKit; speedoUnits: string }
class CarBuilder {
  private modelName?: string;
  private paint = "white";
  constructor(private readonly parts: PartsFactory) {}
  model(m: string) { this.modelName = m; return this; }
  color(c: string) { this.paint = c; return this; }
  build(): Car {
    if (!this.modelName) throw new Error("model required");
    return { model: this.modelName, color: this.paint, engine: this.parts.engine(),
      safety: this.parts.safety(), speedoUnits: this.parts.units };
  }
}

console.log(new CarBuilder(EU).model("Roadster").color("red").build());
console.log(new CarBuilder(US).model("Roadster").build());

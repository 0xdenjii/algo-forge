/**
 * Title: Adapter - Temperature Sensors (class vs object adapter)
 * Problem: A thermostat reads `CelsiusSensor.readCelsius()`. A vendor sensor reports
 *   Fahrenheit tenths as an integer via `getRawF10()`. Adapt it, showing both an object
 *   adapter (composition) and a class adapter (inheritance).
 * Approach: Convert ((raw / 10) - 32) * 5 / 9 inside the adapter.
 * Complexity: O(1).
 */
interface CelsiusSensor { readCelsius(): number }

class VendorFahrenheitSensor {
  constructor(private rawF10: number) {}
  getRawF10(): number { return this.rawF10; }
}

const f10ToC = (raw: number): number => ((raw / 10 - 32) * 5) / 9;

class ObjectAdapter implements CelsiusSensor {
  constructor(private s: VendorFahrenheitSensor) {}
  readCelsius(): number { return f10ToC(this.s.getRawF10()); }
}
class ClassAdapter extends VendorFahrenheitSensor implements CelsiusSensor {
  readCelsius(): number { return f10ToC(this.getRawF10()); }
}

class Thermostat {
  constructor(private sensor: CelsiusSensor, private target: number) {}
  action(): string {
    const t = this.sensor.readCelsius();
    const act = t < this.target - 0.5 ? "heat" : t > this.target + 0.5 ? "cool" : "idle";
    return `${t.toFixed(1)}C -> ${act}`;
  }
}

console.log(new Thermostat(new ObjectAdapter(new VendorFahrenheitSensor(680)), 21).action()); // 20.0C -> heat
console.log(new Thermostat(new ClassAdapter(770), 21).action()); // 25.0C -> cool

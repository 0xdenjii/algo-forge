/**
 * Title: Command - Smart Home Remote with Macros
 * Problem: A programmable remote binds slots to device actions. Support
 *   macro commands (several commands as one) and an undo button.
 * Approach: Command objects wrap receivers; MacroCommand composes commands
 *   and undoes them in reverse order.
 * Complexity: O(k) for a macro of k commands; O(slots) space.
 */
class Light { on = false; brightness = 0; }
class Thermostat { temp = 20; }

interface Command { readonly label: string; execute(): void; undo(): void }

const lightOn = (l: Light): Command => {
  let prev = { on: l.on, b: l.brightness };
  return {
    label: "light on",
    execute() { prev = { on: l.on, b: l.brightness }; l.on = true; l.brightness = 100; },
    undo() { l.on = prev.on; l.brightness = prev.b; },
  };
};

const setTemp = (t: Thermostat, value: number): Command => {
  let prev = t.temp;
  return { label: `temp ${value}`, execute() { prev = t.temp; t.temp = value; }, undo() { t.temp = prev; } };
};

class MacroCommand implements Command {
  constructor(readonly label: string, private cmds: Command[]) {}
  execute(): void { this.cmds.forEach((c) => c.execute()); }
  undo(): void { [...this.cmds].reverse().forEach((c) => c.undo()); }
}

class Remote {
  private slots = new Map<number, Command>();
  private history: Command[] = [];
  bind(slot: number, c: Command): void { this.slots.set(slot, c); }
  press(slot: number): void {
    const c = this.slots.get(slot);
    if (!c) throw new Error(`slot ${slot} is empty`);
    c.execute();
    this.history.push(c);
  }
  undo(): void { this.history.pop()?.undo(); }
}

const light = new Light(), thermo = new Thermostat(), remote = new Remote();
remote.bind(1, lightOn(light));
remote.bind(2, new MacroCommand("movie night", [lightOn(light), setTemp(thermo, 23)]));
remote.press(2);
console.log(light, thermo);  // on/100, 23
remote.undo();
console.log(light, thermo);  // off/0, 20

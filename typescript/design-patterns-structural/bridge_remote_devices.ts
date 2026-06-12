/**
 * Title: Bridge - Remote Controls and Devices
 * Problem: Remotes (basic, advanced with mute) control devices (TV, Radio), each device
 *   having its own volume range. Let both hierarchies evolve independently.
 * Approach: RemoteControl (abstraction) delegates to a Device (implementor) interface.
 * Complexity: O(1) per operation.
 */
interface Device {
  readonly name: string;
  isOn(): boolean;
  power(on: boolean): void;
  getVolume(): number;
  setVolume(v: number): void;
}

abstract class BaseDevice implements Device {
  private on = false;
  private volume = 10;
  constructor(readonly name: string, private readonly maxVolume: number) {}
  isOn(): boolean { return this.on; }
  power(on: boolean): void { this.on = on; }
  getVolume(): number { return this.volume; }
  setVolume(v: number): void { this.volume = Math.max(0, Math.min(this.maxVolume, v)); }
}
class Tv extends BaseDevice { constructor() { super("TV", 100); } }
class Radio extends BaseDevice { constructor() { super("Radio", 30); } }

class RemoteControl {
  constructor(protected device: Device) {}
  togglePower(): void { this.device.power(!this.device.isOn()); }
  volumeUp(): void { this.device.setVolume(this.device.getVolume() + 10); }
  volumeDown(): void { this.device.setVolume(this.device.getVolume() - 10); }
  status(): string {
    return `${this.device.name}: ${this.device.isOn() ? "on" : "off"}, vol=${this.device.getVolume()}`;
  }
}
class AdvancedRemote extends RemoteControl {
  mute(): void { this.device.setVolume(0); }
}

const r1 = new RemoteControl(new Tv());
r1.togglePower(); r1.volumeUp(); r1.volumeUp();
console.log(r1.status()); // TV: on, vol=30
const r2 = new AdvancedRemote(new Radio());
r2.togglePower(); r2.volumeUp(); r2.volumeUp(); r2.volumeUp();
console.log(r2.status()); // Radio: on, vol=30 (clamped)
r2.mute();
console.log(r2.status()); // Radio: on, vol=0

/**
 * Title: Facade - Home Theater
 * Problem: Watching a movie requires orchestrating amplifier, projector, streaming player,
 *   lights and screen in the right order. Give clients a single `watchMovie`/`endMovie`.
 * Approach: HomeTheaterFacade composes the subsystems and encodes the sequence; the
 *   subsystems remain accessible for advanced use.
 * Complexity: O(1) per call (fixed number of subsystem steps).
 */
const log: string[] = [];

class Amplifier {
  on(): void { log.push("amp on"); }
  setSurround(): void { log.push("amp surround 5.1"); }
  setVolume(v: number): void { log.push(`amp volume ${v}`); }
  off(): void { log.push("amp off"); }
}
class Projector {
  on(): void { log.push("projector on"); }
  wideScreen(): void { log.push("projector 16:9"); }
  off(): void { log.push("projector off"); }
}
class StreamingPlayer {
  on(): void { log.push("player on"); }
  play(title: string): void { log.push(`player playing "${title}"`); }
  stop(): void { log.push("player stop"); }
  off(): void { log.push("player off"); }
}
class Lights { dim(pct: number): void { log.push(`lights ${pct}%`); } }
class Screen {
  down(): void { log.push("screen down"); }
  up(): void { log.push("screen up"); }
}

class HomeTheaterFacade {
  constructor(
    private amp = new Amplifier(),
    private projector = new Projector(),
    private player = new StreamingPlayer(),
    private lights = new Lights(),
    private screen = new Screen(),
  ) {}
  watchMovie(title: string): void {
    this.lights.dim(10);
    this.screen.down();
    this.projector.on();
    this.projector.wideScreen();
    this.amp.on();
    this.amp.setSurround();
    this.amp.setVolume(5);
    this.player.on();
    this.player.play(title);
  }
  endMovie(): void {
    this.player.stop();
    this.player.off();
    this.amp.off();
    this.projector.off();
    this.screen.up();
    this.lights.dim(100);
  }
}

const theater = new HomeTheaterFacade();
theater.watchMovie("Arrival");
theater.endMovie();
console.log(log.join("\n"));

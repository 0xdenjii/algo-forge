/**
 * Title: State - TCP Connection (GoF classic)
 * Problem: A TCPConnection responds to open/send/close/ack differently
 *   depending on its state (Closed, Listen, Established). Illegal operations
 *   in a state must raise errors rather than silently corrupt the connection.
 * Approach: Abstract state base class with default "illegal" behaviour;
 *   concrete states override only the operations they support.
 * Complexity: O(1) per operation; O(buffered bytes) space.
 */
abstract class TCPState {
  abstract readonly name: string;
  open(_c: TCPConnection): void { this.illegal("open"); }
  listen(_c: TCPConnection): void { this.illegal("listen"); }
  synAck(_c: TCPConnection): void { this.illegal("synAck"); }
  send(_c: TCPConnection, _data: string): void { this.illegal("send"); }
  close(_c: TCPConnection): void { this.illegal("close"); }
  protected illegal(op: string): never { throw new Error(`${op} not allowed in ${this.name}`); }
}

class Closed extends TCPState {
  readonly name = "CLOSED";
  open(c: TCPConnection) { c.change(new SynSent()); }
  listen(c: TCPConnection) { c.change(new Listen()); }
  close() { /* idempotent */ }
}
class Listen extends TCPState {
  readonly name = "LISTEN";
  synAck(c: TCPConnection) { c.change(new Established()); }
  close(c: TCPConnection) { c.change(new Closed()); }
}
class SynSent extends TCPState {
  readonly name = "SYN_SENT";
  synAck(c: TCPConnection) { c.change(new Established()); }
  close(c: TCPConnection) { c.change(new Closed()); }
}
class Established extends TCPState {
  readonly name = "ESTABLISHED";
  send(c: TCPConnection, data: string) { c.sent.push(data); }
  close(c: TCPConnection) { c.change(new Closed()); }
}

class TCPConnection {
  private state: TCPState = new Closed();
  readonly sent: string[] = [];
  readonly log: string[] = [];
  change(s: TCPState) { this.log.push(`${this.state.name} -> ${s.name}`); this.state = s; }
  get stateName() { return this.state.name; }
  open() { this.state.open(this); }
  listen() { this.state.listen(this); }
  synAck() { this.state.synAck(this); }
  send(d: string) { this.state.send(this, d); }
  close() { this.state.close(this); }
}

const conn = new TCPConnection();
try { conn.send("early"); } catch (e) { console.log((e as Error).message); }
conn.open(); conn.synAck(); conn.send("hello"); conn.send("world"); conn.close();
console.log(conn.log, conn.sent, conn.stateName);

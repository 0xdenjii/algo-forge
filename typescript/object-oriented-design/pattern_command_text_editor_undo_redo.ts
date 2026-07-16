/**
 * Design Pattern - Command (text editor with undo/redo)
 * Editor supports insert, delete and replace at a cursor position, plus undo and redo.
 * A new edit after an undo discards the redo history.
 * Approach: each edit is a Command object with execute()/undo(); two stacks hold history.
 * Time: O(text length) per command (string slicing), O(1) stack ops. Space: O(history).
 */
class Document {
  text = "";
}

interface Command {
  execute(doc: Document): void;
  undo(doc: Document): void;
}

class InsertCommand implements Command {
  constructor(private readonly pos: number, private readonly s: string) {}
  execute(doc: Document): void {
    doc.text = doc.text.slice(0, this.pos) + this.s + doc.text.slice(this.pos);
  }
  undo(doc: Document): void {
    doc.text = doc.text.slice(0, this.pos) + doc.text.slice(this.pos + this.s.length);
  }
}

class DeleteCommand implements Command {
  private removed = "";
  constructor(private readonly pos: number, private readonly len: number) {}
  execute(doc: Document): void {
    this.removed = doc.text.slice(this.pos, this.pos + this.len);
    doc.text = doc.text.slice(0, this.pos) + doc.text.slice(this.pos + this.len);
  }
  undo(doc: Document): void {
    doc.text = doc.text.slice(0, this.pos) + this.removed + doc.text.slice(this.pos);
  }
}

/** Composite command: replace = delete + insert as one undoable unit */
class MacroCommand implements Command {
  constructor(private readonly parts: Command[]) {}
  execute(doc: Document): void {
    this.parts.forEach((c) => c.execute(doc));
  }
  undo(doc: Document): void {
    [...this.parts].reverse().forEach((c) => c.undo(doc));
  }
}

export class Editor {
  private readonly doc = new Document();
  private readonly undoStack: Command[] = [];
  private readonly redoStack: Command[] = [];

  private run(cmd: Command): this {
    cmd.execute(this.doc);
    this.undoStack.push(cmd);
    this.redoStack.length = 0;
    return this;
  }
  insert(pos: number, s: string): this {
    return this.run(new InsertCommand(pos, s));
  }
  delete(pos: number, len: number): this {
    return this.run(new DeleteCommand(pos, len));
  }
  replace(pos: number, len: number, s: string): this {
    return this.run(new MacroCommand([new DeleteCommand(pos, len), new InsertCommand(pos, s)]));
  }
  undo(): this {
    const c = this.undoStack.pop();
    if (c) {
      c.undo(this.doc);
      this.redoStack.push(c);
    }
    return this;
  }
  redo(): this {
    const c = this.redoStack.pop();
    if (c) {
      c.execute(this.doc);
      this.undoStack.push(c);
    }
    return this;
  }
  get text(): string {
    return this.doc.text;
  }
}

const ed = new Editor().insert(0, "hello world").replace(6, 5, "there");
console.log(ed.text); // hello there
console.log(ed.undo().text, "|", ed.undo().text, "|", ed.redo().text); // hello world |  | hello world
console.log(ed.delete(0, 6).text, "|", ed.redo().text); // world | world   (redo cleared by new edit)

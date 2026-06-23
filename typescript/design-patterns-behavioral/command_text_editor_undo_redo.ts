/**
 * Title: Command - Text Editor with Undo/Redo
 * Problem: Support insert and delete on a document with unlimited undo and
 *   redo. Running a new command after undo clears the redo history.
 * Approach: Each Command implements execute/undo; two stacks hold history.
 * Complexity: O(L) per op for string slicing (L = doc length); O(h) history.
 */
class TextDocument { text = ""; }

interface Command { execute(): void; undo(): void }

class InsertCommand implements Command {
  constructor(private doc: TextDocument, private pos: number, private s: string) {}
  execute(): void { const t = this.doc.text; this.doc.text = t.slice(0, this.pos) + this.s + t.slice(this.pos); }
  undo(): void { const t = this.doc.text; this.doc.text = t.slice(0, this.pos) + t.slice(this.pos + this.s.length); }
}

class DeleteCommand implements Command {
  private removed = "";
  constructor(private doc: TextDocument, private pos: number, private len: number) {}
  execute(): void {
    const t = this.doc.text;
    this.removed = t.slice(this.pos, this.pos + this.len);
    this.doc.text = t.slice(0, this.pos) + t.slice(this.pos + this.len);
  }
  undo(): void { const t = this.doc.text; this.doc.text = t.slice(0, this.pos) + this.removed + t.slice(this.pos); }
}

class Editor {
  readonly doc = new TextDocument();
  private undoStack: Command[] = [];
  private redoStack: Command[] = [];
  run(cmd: Command): void { cmd.execute(); this.undoStack.push(cmd); this.redoStack = []; }
  undo(): void { const c = this.undoStack.pop(); if (c) { c.undo(); this.redoStack.push(c); } }
  redo(): void { const c = this.redoStack.pop(); if (c) { c.execute(); this.undoStack.push(c); } }
}

const ed = new Editor();
ed.run(new InsertCommand(ed.doc, 0, "Hello"));
ed.run(new InsertCommand(ed.doc, 5, " World"));
ed.run(new DeleteCommand(ed.doc, 0, 6));
console.log(ed.doc.text);              // "World"
ed.undo(); console.log(ed.doc.text);   // "Hello World"
ed.undo(); console.log(ed.doc.text);   // "Hello"
ed.redo(); console.log(ed.doc.text);   // "Hello World"

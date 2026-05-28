/**
 * Title: Factory Method (Document Editor)
 * Problem: An Application framework opens new documents; a TextApp creates
 *   text documents and a SpreadsheetApp creates sheets, while open/track logic is shared.
 * Approach: Application.newDocument() calls the overridable createDocument() hook.
 * Complexity: O(1) per document; O(n) space for n open docs.
 */
interface Doc { title: string; render(): string }
abstract class Application {
  readonly docs: Doc[] = [];
  protected abstract createDocument(title: string): Doc;
  newDocument(title: string): Doc {
    const d = this.createDocument(title);
    this.docs.push(d);
    return d;
  }
}
class TextApp extends Application {
  protected createDocument(title: string): Doc { return { title, render: () => `# ${title}\n(text)` }; }
}
class SpreadsheetApp extends Application {
  protected createDocument(title: string): Doc { return { title, render: () => `| ${title} | A | B |` }; }
}

console.log(new TextApp().newDocument("Notes").render());
console.log(new SpreadsheetApp().newDocument("Budget").render());

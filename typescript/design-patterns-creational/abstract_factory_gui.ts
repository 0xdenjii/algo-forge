/**
 * Title: Abstract Factory (Cross-platform GUI)
 * Problem: Render Button + Checkbox families for Windows or macOS; products from
 *   different families must never be mixed.
 * Approach: GUIFactory interface with one concrete factory per family.
 * Complexity: O(1) per widget.
 */
interface Button { paint(): string }
interface Checkbox { paint(): string }
interface GUIFactory { createButton(): Button; createCheckbox(): Checkbox }

const WinFactory: GUIFactory = {
  createButton: () => ({ paint: () => "[Win Button]" }),
  createCheckbox: () => ({ paint: () => "[Win Checkbox]" }),
};
const MacFactory: GUIFactory = {
  createButton: () => ({ paint: () => "(Mac Button)" }),
  createCheckbox: () => ({ paint: () => "(Mac Checkbox)" }),
};

function renderForm(f: GUIFactory): string {
  return [f.createButton().paint(), f.createCheckbox().paint()].join(" ");
}
const os: string = "mac";
console.log(renderForm(os === "mac" ? MacFactory : WinFactory));

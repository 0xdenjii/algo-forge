/**
 * Title: Abstract Factory (UI Theme Kits)
 * Problem: Produce a matched set of styled components (button, card, input markup)
 *   for Light or Dark themes, selected at runtime.
 * Approach: factory-of-factories built from a palette; `satisfies` checks the registry.
 * Complexity: O(1) per component.
 */
interface Palette { bg: string; fg: string; accent: string; border: string }
interface ThemeFactory { button(label: string): string; card(content: string): string; input(name: string): string }

function themeFactory(p: Palette): ThemeFactory {
  return {
    button: (l) => `<button style="background:${p.accent};color:${p.bg}">${l}</button>`,
    card: (c) => `<div style="background:${p.bg};color:${p.fg};border:1px solid ${p.border}">${c}</div>`,
    input: (n) => `<input name="${n}" style="background:${p.bg};color:${p.fg};border-color:${p.border}">`,
  };
}
const themes = {
  light: themeFactory({ bg: "#fff", fg: "#111", accent: "#0a66c2", border: "#ddd" }),
  dark: themeFactory({ bg: "#121212", fg: "#eee", accent: "#4ea1ff", border: "#333" }),
} satisfies Record<string, ThemeFactory>;

function loginForm(t: ThemeFactory): string { return t.card(t.input("user") + t.button("Login")); }
console.log(loginForm(themes.dark));

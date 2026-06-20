/**
 * Title: Proxy - Protection Proxy (role-based access control)
 * Problem: A DocumentStore exposes read/write/delete. Only some roles may perform each
 *   operation. Enforce permissions without touching the store, and keep an audit trail.
 * Approach: ProtectedStore implements the same interface, checks a permission table for the
 *   current user, records an audit entry, then delegates or throws.
 * Complexity: O(1) per check.
 */
type Role = "viewer" | "editor" | "admin";
type Action = "read" | "write" | "delete";
interface User { name: string; role: Role }

interface DocumentStore {
  read(id: string): string;
  write(id: string, content: string): void;
  delete(id: string): void;
}

class InMemoryStore implements DocumentStore {
  private docs = new Map<string, string>([["readme", "hello"]]);
  read(id: string): string {
    const d = this.docs.get(id);
    if (d === undefined) throw new Error(`no document ${id}`);
    return d;
  }
  write(id: string, content: string): void { this.docs.set(id, content); }
  delete(id: string): void { this.docs.delete(id); }
}

const PERMISSIONS: Record<Role, ReadonlySet<Action>> = {
  viewer: new Set(["read"]),
  editor: new Set(["read", "write"]),
  admin: new Set(["read", "write", "delete"]),
};

class AccessDenied extends Error {}

class ProtectedStore implements DocumentStore {
  readonly audit: string[] = [];
  constructor(private inner: DocumentStore, private user: User) {}
  private check(action: Action, id: string): void {
    const ok = PERMISSIONS[this.user.role].has(action);
    this.audit.push(`${this.user.name}:${action}:${id}:${ok ? "allow" : "deny"}`);
    if (!ok) throw new AccessDenied(`${this.user.role} may not ${action}`);
  }
  read(id: string): string { this.check("read", id); return this.inner.read(id); }
  write(id: string, content: string): void { this.check("write", id); this.inner.write(id, content); }
  delete(id: string): void { this.check("delete", id); this.inner.delete(id); }
}

const store = new InMemoryStore();
const asViewer = new ProtectedStore(store, { name: "vic", role: "viewer" });
const asEditor = new ProtectedStore(store, { name: "eve", role: "editor" });
console.log(asViewer.read("readme"));
try { asViewer.write("readme", "hacked"); } catch (e) { console.log("denied:", (e as Error).message); }
asEditor.write("readme", "updated");
console.log(asViewer.read("readme"));
try { asEditor.delete("readme"); } catch (e) { console.log("denied:", (e as Error).message); }
console.log(asViewer.audit, asEditor.audit);

/**
 * Title: Abstract Factory (Database Drivers)
 * Problem: Data layer needs Connection + QueryBuilder that agree on SQL dialect
 *   (Postgres uses $1 placeholders and "ident", MySQL uses ? and `ident`).
 * Approach: DbFactory produces a consistent family of dialect objects.
 * Complexity: O(p) to build a query with p params.
 */
interface QueryBuilder { select(table: string, where: string[]): string }
interface Connection { describe(): string }
interface DbFactory { connection(dsn: string): Connection; queryBuilder(): QueryBuilder }

const postgres: DbFactory = {
  connection: (dsn) => ({ describe: () => `pg connection to ${dsn}` }),
  queryBuilder: () => ({
    select: (t, w) => `SELECT * FROM "${t}"` +
      (w.length ? " WHERE " + w.map((c, i) => `"${c}" = $${i + 1}`).join(" AND ") : ""),
  }),
};
const BT = "`";
const mysql: DbFactory = {
  connection: (dsn) => ({ describe: () => `mysql connection to ${dsn}` }),
  queryBuilder: () => ({
    select: (t, w) => `SELECT * FROM ${BT}${t}${BT}` +
      (w.length ? " WHERE " + w.map((c) => `${BT}${c}${BT} = ?`).join(" AND ") : ""),
  }),
};

for (const f of [postgres, mysql]) {
  console.log(f.connection("db:5432").describe());
  console.log(f.queryBuilder().select("users", ["id", "email"]));
}

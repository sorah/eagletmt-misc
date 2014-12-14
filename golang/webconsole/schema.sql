CREATE TABLE executions (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  command TEXT NOT NULL,
  status INTEGER NOT NULL DEFAULT -1,
  output TEXT NOT NULL DEFAULT ""
);

CREATE TYPE table_status AS ENUM ('open', 'closed');

CREATE TABLE IF NOT EXISTS tables (
  id UUID PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  status table_status NOT NULL,
  settings JSONB NOT NULL
);

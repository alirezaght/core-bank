CREATE TYPE transaction_type AS ENUM ('WITHDRAW', 'DEPOSIT', 'LOCK', 'UNLOCK');
CREATE TYPE transaction_reason AS ENUM ('TRANSFER', 'REVERT', 'LOCK', 'UNLOCK', 'WITHDRAW', 'DEPOSIT');

CREATE TABLE t_transaction (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  account VARCHAR(100) NOT NULL,
  type transaction_type NOT NULL,
  seq BIGINT NOT NULL DEFAULT 0,
  amount NUMERIC NOT NULL DEFAULT 0 CHECK (amount >= 0),
  reason transaction_reason NOT NULL,
  comment TEXT,
  description TEXT,
  balance NUMERIC NOT NULL DEFAULT 0 CHECK (balance >= 0),
  blocked NUMERIC NOT NULL DEFAULT 0 CHECK (blocked >= 0),
  created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CHECK (balance >= blocked),
  UNIQUE (account, seq)
)

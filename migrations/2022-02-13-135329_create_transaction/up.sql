CREATE TYPE transaction_type AS ENUM ('withdraw', 'deposit', 'lock', 'unlock');
CREATE TYPE transaction_reason AS ENUM ('transfer', 'revert', 'lock', 'unlock', 'withdraw', 'deposit');

CREATE TABLE t_account (
  address VARCHAR(100),
  detail TEXT,
  seq BIGINT NOT NULL DEFAULT 0,
  withdraw BOOLEAN NOT NULL DEFAULT TRUE,
  deposit BOOLEAN NOT NULL DEFAULT TRUE,
  comment TEXT,
  created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (address, seq)
);


CREATE TABLE t_transaction (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  account VARCHAR(100) NOT NULL,
  account_seq BIGINT NOT NULL DEFAULT 0,
  type transaction_type NOT NULL,
  seq BIGINT NOT NULL DEFAULT 0,
  amount NUMERIC NOT NULL DEFAULT 0 CHECK (amount >= 0),
  reason transaction_reason NOT NULL,
  comment TEXT,
  description TEXT,
  balance NUMERIC NOT NULL DEFAULT 0 CHECK (balance >= 0),
  blocked NUMERIC NOT NULL DEFAULT 0 CHECK (blocked >= 0),
  factor VARCHAR(100),
  created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CHECK (balance >= blocked),
  UNIQUE (account, seq),
  CONSTRAINT fk_account FOREIGN KEY (account, account_seq) REFERENCES t_account(address, seq) ON DELETE CASCADE
);

CREATE INDEX transaction_account_index ON t_transaction(account);

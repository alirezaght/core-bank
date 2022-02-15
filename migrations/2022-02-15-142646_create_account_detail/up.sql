CREATE TABLE t_account (
  address VARCHAR(100) PRIMARY KEY,
  detail TEXT,
  seq BIGINT NOT NULL DEFAULT 0,
  withdraw BOOLEAN NOT NULL DEFAULT TRUE,
  deposit BOOLEAN NOT NULL DEFAULT TRUE,
  comment TEXT,
  UNIQUE (address, seq)
)


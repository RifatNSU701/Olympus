ALTER TABLE users
  DROP CONSTRAINT IF EXISTS users_email_length_check;

ALTER TABLE users
  ADD CONSTRAINT users_email_length_check
  CHECK (char_length(email) <= 254);

CREATE UNIQUE INDEX IF NOT EXISTS users_email_lower_uidx
  ON users (lower(email));

ALTER TABLE comments ADD COLUMN user_id uuid REFERENCES users(id);
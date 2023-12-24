CREATE TABLE posts (
    id      uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid REFERENCES users(id),
    title   TEXT,
    content TEXT
);
CREATE TABLE comments (
    id        uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id   uuid REFERENCES posts(id),
    parent_id uuid REFERENCES comments(id),
    content   TEXT
);
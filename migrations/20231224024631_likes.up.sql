-- Add up migration script here
CREATE TABLE post_likes (
    id      uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid REFERENCES users(id),
    post_id uuid REFERENCES posts(id)
);

CREATE INDEX idx_post_likes_post_id ON post_likes (post_id);
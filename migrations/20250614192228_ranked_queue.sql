CREATE TABLE ranked_queue (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES "users"(id),
    mode INTEGER NOT NULL CHECK (mode >= 0 AND mode <= 3),
    created_at TIMESTAMP NOT NULL,
    UNIQUE(user_id)  -- Un joueur ne peut être qu'une fois dans la queue
);

-- Index pour accélérer la recherche de joueurs en attente
CREATE INDEX idx_ranked_queue_mode ON ranked_queue(mode);
CREATE INDEX idx_ranked_queue_created_at ON ranked_queue(created_at); 
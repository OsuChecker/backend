CREATE TABLE ranked_match (
    id SERIAL PRIMARY KEY,
    player1_id INTEGER NOT NULL REFERENCES "users"(id),
    player2_id INTEGER REFERENCES "users"(id),
    status VARCHAR(20) NOT NULL CHECK (status IN ('waiting_player', 'playing', 'completed')),
    match_type VARCHAR(20) NOT NULL CHECK (match_type IN ('five_minutes')),
    mode INTEGER NOT NULL CHECK (mode >= 0 AND mode <= 3),
    best_of INTEGER NOT NULL,
    player1_points INTEGER NOT NULL DEFAULT 0,
    player2_points INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Les maps bannies par les joueurs
CREATE TABLE ranked_match_banned_maps (
    id SERIAL PRIMARY KEY,
    match_id INTEGER NOT NULL REFERENCES ranked_match(id),
    beatmap_id INTEGER NOT NULL REFERENCES beatmap(id),
    banned_by INTEGER NOT NULL REFERENCES "users"(id),
    created_at TIMESTAMP NOT NULL,
    UNIQUE(match_id, beatmap_id)
);

CREATE TABLE ranked_match_round (
    id SERIAL PRIMARY KEY,
    match_id INTEGER NOT NULL REFERENCES ranked_match(id),
    round_number INTEGER NOT NULL,
    beatmap_id INTEGER NOT NULL REFERENCES beatmap(id),
    -- Un round peut être:
    -- preparing: 30s pour DL la map
    -- playing: 5min de jeu
    -- completed: Round terminé
    status VARCHAR(20) NOT NULL CHECK (status IN ('preparing', 'playing', 'completed')),
    -- Indique si les joueurs sont prêts à commencer
    player1_ready BOOLEAN NOT NULL DEFAULT false,
    player2_ready BOOLEAN NOT NULL DEFAULT false,
    -- Les meilleurs scores des joueurs pendant les 5min
    player1_best_score_id INTEGER REFERENCES score(id),
    player2_best_score_id INTEGER REFERENCES score(id),
    -- Le gagnant du round
    winner_id INTEGER REFERENCES "users"(id),
    preparation_start TIMESTAMP, -- Début des 30s de DL
    play_start TIMESTAMP,        -- Début des 5min de jeu
    ended_at TIMESTAMP,          -- Fin du round
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    UNIQUE(match_id, round_number)
);

-- Historique de tous les scores soumis pendant un round
CREATE TABLE ranked_match_round_scores (
    id SERIAL PRIMARY KEY,
    round_id INTEGER NOT NULL REFERENCES ranked_match_round(id),
    player_id INTEGER NOT NULL REFERENCES "users"(id),
    score_id INTEGER NOT NULL REFERENCES score(id),
    created_at TIMESTAMP NOT NULL,
    UNIQUE(round_id, score_id)
); 
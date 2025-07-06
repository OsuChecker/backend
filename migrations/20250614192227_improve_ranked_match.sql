-- Ajout de contraintes et améliorations pour le ranked match
ALTER TABLE ranked_match
    ADD COLUMN preparation_duration INTEGER NOT NULL DEFAULT 30, -- 30 secondes par défaut
    ADD COLUMN play_duration INTEGER NOT NULL DEFAULT 300; -- 5 minutes par défaut

-- Ajout d'un index pour accélérer les recherches par status
CREATE INDEX idx_ranked_match_status ON ranked_match(status);

-- Ajout d'un index pour les rounds
CREATE INDEX idx_ranked_match_round_match_id ON ranked_match_round(match_id);
CREATE INDEX idx_ranked_match_round_status ON ranked_match_round(status);

-- Ajout d'un index pour les scores
CREATE INDEX idx_ranked_match_round_scores_round_id ON ranked_match_round_scores(round_id);
CREATE INDEX idx_ranked_match_round_scores_player_id ON ranked_match_round_scores(player_id);

-- Ajout d'un index pour les maps bannies
CREATE INDEX idx_ranked_match_banned_maps_match_id ON ranked_match_banned_maps(match_id); 
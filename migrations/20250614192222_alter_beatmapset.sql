-- Add migration script here
-- Rendre le champ creator_id nullable dans la table beatmapset
ALTER TABLE beatmapset ALTER COLUMN creator_id DROP NOT NULL; 
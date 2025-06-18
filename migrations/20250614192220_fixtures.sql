-- Fixtures for users table
INSERT INTO users (username, email, password_hash, country, avatar_url, is_verified) VALUES
('cookiezi', 'cookiezi@example.com', '$2a$12$rAnd0mH4sH.r4nd0mh4sh12', 'JP', 'https://example.com/avatars/cookiezi.jpg', true),
('mrekk', 'mrekk@example.com', '$2a$12$rAnd0mH4sH.r4nd0mh4sh34', 'AU', 'https://example.com/avatars/mrekk.jpg', true),
('btmc', 'btmc@example.com', '$2a$12$rAnd0mH4sH.r4nd0mh4sh56', 'US', 'https://example.com/avatars/btmc.jpg', true),
('whitecat', 'whitecat@example.com', '$2a$12$rAnd0mH4sH.r4nd0mh4sh78', 'DE', 'https://example.com/avatars/whitecat.jpg', true),
('vaxei', 'vaxei@example.com', '$2a$12$rAnd0mH4sH.r4nd0mh4sh90', 'US', 'https://example.com/avatars/vaxei.jpg', true);

-- Fixtures for beatmapset table
INSERT INTO beatmapset (artist, title, creator_id, source, tags, status, has_video, has_storyboard, cover_url) VALUES
('Imperial Circus Dead Decadence', 'Yomi yori Kikoyu, Koukoku no Tou to Honoo no Shoujo', 1, 'Fantasy', ARRAY['metal', 'japanese', 'stream', 'hard'], 'ranked', false, true, 'https://example.com/covers/yomi_yori.jpg'),
('xi', 'Freedom Dive', 2, 'SOUND VOLTEX III GRAVITY WARS', ARRAY['stream', 'technical', 'speed'], 'ranked', false, false, 'https://example.com/covers/freedom_dive.jpg'),
('DragonForce', 'Through the Fire and Flames', 3, '', ARRAY['metal', 'long', 'stream', 'guitar'], 'ranked', true, false, 'https://example.com/covers/ttfaf.jpg'),
('Camellia', 'GHOST', 4, 'osu! featured artist', ARRAY['electronic', 'speedcore', 'difficult'], 'ranked', false, true, 'https://example.com/covers/ghost.jpg'),
('Kano', 'Stella-rium', 5, 'Houkago no Pleiades', ARRAY['j-pop', 'anime', 'jump'], 'ranked', false, false, 'https://example.com/covers/stellarium.jpg');

-- Fixtures for beatmap table
INSERT INTO beatmap (beatmapset_id, version, difficulty_rating, count_circles, count_sliders, count_spinners, max_combo, drain_time, total_time, bpm, cs, ar, od, hp, mode, status, hit_length, file_md5, file_path) VALUES
(1, 'Kyouaku', 8.28, 3500, 1200, 3, 4700, 480, 540, 220.00, 4.2, 9.8, 9.0, 6.0, 0, 'ranked', 480, 'a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6', '/beatmaps/yomi_yori_kyouaku.osu'),
(1, 'Kurushimi', 7.65, 3200, 1100, 2, 4300, 480, 540, 220.00, 4.0, 9.6, 8.8, 6.0, 0, 'ranked', 480, 'q1w2e3r4t5y6u7i8o9p0a1s2d3f4g5h', '/beatmaps/yomi_yori_kurushimi.osu'),
(2, 'Four Dimensions', 7.92, 2200, 800, 4, 3000, 240, 260, 222.22, 4.0, 9.6, 9.0, 7.0, 0, 'ranked', 240, 'z1x2c3v4b5n6m7q8w9e0r1t2y3u4i5', '/beatmaps/freedom_dive_4d.osu'),
(2, 'COSMIC', 6.54, 2000, 750, 3, 2750, 240, 260, 222.22, 4.2, 9.2, 8.0, 7.0, 0, 'ranked', 240, 'o6p7a8s9d0f1g2h3j4k5l6z7x8c9v0', '/beatmaps/freedom_dive_cosmic.osu'),
(3, 'Flames', 7.26, 2800, 1300, 5, 4100, 440, 460, 200.00, 3.8, 9.5, 8.5, 6.0, 0, 'ranked', 440, 'b1n2m3a4s5d6f7g8h9j0k1l2z3x4c5', '/beatmaps/ttfaf_flames.osu'),
(3, 'Inferno', 6.84, 2600, 1200, 4, 3800, 440, 460, 200.00, 4.0, 9.3, 8.2, 6.5, 0, 'ranked', 440, 'v6b7n8m9q0w1e2r3t4y5u6i7o8p9a0', '/beatmaps/ttfaf_inferno.osu'),
(4, 'Nightmare', 8.10, 2400, 1000, 2, 3400, 210, 230, 240.00, 3.9, 10.0, 9.2, 5.5, 0, 'ranked', 210, 's1d2f3g4h5j6k7l8z9x0c1v2b3n4m5', '/beatmaps/ghost_nightmare.osu'),
(5, 'Pleiades', 5.82, 1800, 600, 3, 2400, 180, 200, 180.00, 4.0, 9.0, 8.0, 6.0, 0, 'ranked', 180, 'q6w7e8r9t0y1u2i3o4p5a6s7d8f9g0', '/beatmaps/stellarium_pleiades.osu'),
(5, 'Constellation', 4.95, 1600, 550, 2, 2150, 180, 200, 180.00, 4.2, 8.5, 7.5, 6.5, 0, 'ranked', 180, 'h1j2k3l4z5x6c7v8b9n0m1q2w3e4r5', '/beatmaps/stellarium_constellation.osu');


INSERT INTO score (user_id, beatmap_id, score, max_combo, perfect, statistics, mods, accuracy, rank, replay_available) VALUES
(1, 1, 700000, 4700, true, '{"count_300": 1000, "count_100": 0, "count_50": 0, "count_miss": 0, "count_katu": 0, "count_geki": 0}', 0, 1.00, 'XH', true),
(1, 1, 900000, 4300, true, '{"count_300": 900, "count_100": 0, "count_50": 0, "count_miss": 0, "count_katu": 0, "count_geki": 0}', 0, 1.00, 'XH', true),
(2, 1, 1000000, 3900, true, '{"count_300": 800, "count_100": 0, "count_50": 0, "count_miss": 0, "count_katu": 0, "count_geki": 0}', 0, 1.00, 'XH', true),
(4, 1, 700000, 3500, true, '{"count_300": 700, "count_100": 0, "count_50": 0, "count_miss": 0, "count_katu": 0, "count_geki": 0}', 0, 1.00, 'XH', true),
(3, 1, 600000, 3100, true, '{"count_300": 600, "count_100": 0, "count_50": 0, "count_miss": 0, "count_katu": 0, "count_geki": 0}', 0, 1.00, 'XH', true);





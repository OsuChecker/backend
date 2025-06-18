-- Add migration script here

-- Table utilisateur
create table if not exists users (
    id serial primary key,
    username varchar(32) not null unique,
    email varchar(255) not null unique,
    password_hash varchar(255) not null,
    country varchar(2) not null,
    avatar_url varchar(255),
    cover_url varchar(255),
    is_verified boolean not null default false,
    last_visit timestamp,
    created_at timestamp default now(),
    updated_at timestamp default now(),
    constraint valid_username check (username ~ '^[a-zA-Z0-9_-]{3,32}$'),
    constraint valid_email check (email ~ '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$')
);

-- Table beatmapset
create table if not exists beatmapset (
    id serial primary key,
    artist varchar(255) not null,
    artist_unicode varchar(255),
    title varchar(255) not null,
    title_unicode varchar(255),
    creator_id integer not null references users(id),
    source varchar(255),
    tags text[],
    status varchar(20) not null default 'pending',
    has_video boolean not null default false,
    has_storyboard boolean not null default false,
    is_explicit boolean not null default false,
    is_featured boolean not null default false,
    cover_url varchar(255),
    preview_url varchar(255),
    osu_file_url varchar(255),
    created_at timestamp default now(),
    updated_at timestamp default now(),
    constraint valid_status check (status in ('pending', 'ranked', 'qualified', 'loved', 'graveyard'))
);

-- Table beatmap
create table if not exists beatmap (
    id serial primary key,
    beatmapset_id integer not null references beatmapset(id) on delete cascade,
    version varchar(255) not null,
    difficulty_rating decimal(4,2) not null,
    count_circles integer not null,
    count_sliders integer not null,
    count_spinners integer not null,
    max_combo integer not null,
    drain_time integer not null,
    total_time integer not null,
    bpm decimal(10,2) not null,
    cs decimal(3,1) not null,
    ar decimal(3,1) not null,
    od decimal(3,1) not null,
    hp decimal(3,1) not null,
    mode integer not null default 0,
    status varchar(20) not null default 'pending',
    hit_length integer not null,
    file_md5 varchar(32) not null unique,
    file_path varchar(255) not null,
    created_at timestamp default now(),
    updated_at timestamp default now(),
    constraint valid_difficulty_rating check (difficulty_rating >= 0),
    constraint valid_mode check (mode in (0, 1, 2, 3)),
    constraint valid_status check (status in ('pending', 'ranked', 'qualified', 'loved', 'graveyard'))
);

-- Indexes pour la table user
create index if not exists idx_user_username on users(username);
create index if not exists idx_user_email on users(email);
create index if not exists idx_user_country on users(country);
create index if not exists idx_user_created_at on users(created_at);

-- Indexes pour la table beatmapset
create index if not exists idx_beatmapset_artist on beatmapset(artist);
create index if not exists idx_beatmapset_title on beatmapset(title);
create index if not exists idx_beatmapset_creator_id on beatmapset(creator_id);
create index if not exists idx_beatmapset_status on beatmapset(status);
create index if not exists idx_beatmapset_created_at on beatmapset(created_at);

-- Indexes pour la table beatmap
create index if not exists idx_beatmap_beatmapset_id on beatmap(beatmapset_id);
create index if not exists idx_beatmap_version on beatmap(version);
create index if not exists idx_beatmap_difficulty_rating on beatmap(difficulty_rating);
create index if not exists idx_beatmap_mode on beatmap(mode);
create index if not exists idx_beatmap_status on beatmap(status);
create index if not exists idx_beatmap_file_md5 on beatmap(file_md5); 
-- Add migration script here


create table if not exists score (
    id serial primary key,
    user_id integer not null references "user"(id),
    beatmap_id integer not null references beatmap(id),
    score integer not null,
    max_combo integer not null,
    perfect boolean not null default false,
    statistics jsonb not null default '{
        "count_300": 0,
        "count_100": 0,
        "count_50": 0,
        "count_miss": 0,
        "count_katu": 0,
        "count_geki": 0
    }',
    mods integer not null default 0,
    accuracy decimal(6,3) not null,
    rank varchar(2) not null,
    replay_available boolean not null default false,
    created_at timestamp default now(),
    updated_at timestamp default now(),
    constraint fk_user foreign key (user_id) references "user"(id) on delete cascade,
    constraint valid_rank check (rank in ('XH', 'X', 'SH', 'S', 'A', 'B', 'C', 'D')),
    constraint valid_accuracy check (accuracy >= 0 and accuracy <= 100)
);

create table if not exists rating_type (
    id serial primary key,
    name varchar(20) not null unique,
    description text,
    is_active boolean not null default true,
    created_at timestamp default now(),
    updated_at timestamp default now(),
    constraint valid_name check (name ~ '^[a-z_]+$')
);

-- Insertion des types de rating par défaut
insert into rating_type (name, description) values
    ('pp', 'Performance Points - Le système de rating officiel d''osu!'),
    ('quaver', 'Système de rating de Quaver'),
    ('etterna', 'Système de rating d''Etterna');

create table if not exists score_rating (
    id serial primary key,
    score_id integer not null references score(id) on delete cascade,
    rating_type_id integer not null references rating_type(id),
    rating_value decimal(10,3) not null,
    max_rating decimal(10,3),
    created_at timestamp default now(),
    updated_at timestamp default now(),
    constraint valid_rating_value check (rating_value >= 0),
    constraint valid_max_rating check (max_rating is null or max_rating >= 0)
);

-- Indexes pour optimiser les recherches
create index if not exists idx_score_user_id on score(user_id);
create index if not exists idx_score_beatmap_id on score(beatmap_id);
create index if not exists idx_score_created_at on score(created_at);

-- Indexes pour rating_type
create index if not exists idx_rating_type_name on rating_type(name);
create index if not exists idx_rating_type_active on rating_type(is_active);

-- Indexes pour score_rating
create index if not exists idx_score_rating_score_id on score_rating(score_id);
create index if not exists idx_score_rating_type_id on score_rating(rating_type_id);
create index if not exists idx_score_rating_pp_value on score_rating(rating_value) where rating_type_id = 1;
alter table users add column if not exists roles jsonb not null default '["user"]';

alter table users add constraint valid_roles check (roles @> '["user"]');
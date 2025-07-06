-- Add migration script here
-- Add migration script here
alter table beatmap drop constraint valid_status;
alter table beatmap add constraint valid_status check (status in ('pending', 'ranked', 'qualified', 'loved', 'graveyard', 'tournament')); 

alter table beatmap drop constraint valid_status;
alter table beatmap add constraint valid_status check (status in ('pending', 'ranked', 'qualified', 'loved', 'graveyard', 'tournament')); 

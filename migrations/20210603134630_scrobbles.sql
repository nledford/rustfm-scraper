-- drop table if it exists
drop table if exists scrobbles;

create table scrobbles
(
    id            integer not null
        constraint scrobbles_pk
            primary key autoincrement,
    track         text    not null,
    artist        text    not null,
    album         text,
    loved         integer not null
        check (loved in (0, 1)),
    -- local timestamp will be provided in a view
    timestamp_utc integer not null
);

-- descending index for timestamps
create index scrobbles_timestamp_utc_uindex
    on scrobbles (timestamp_utc desc);

-- scrobbles_local view
-- more or less the same as the table, but provides timestamp in local time
-- as well as a true/false column for boolean values

-- example:
-- select datetime('1622728549', 'unixepoch', 'localtime');

-- drop view if it exists
drop view if exists scrobbles_local;

create view scrobbles_local as
select track,
       artist,
       album,
       case when loved = 0 then 'false' else 'true' end  loved,
       timestamp_utc,
       datetime(timestamp_utc, 'unixepoch', 'localtime') timestamp_local
from scrobbles
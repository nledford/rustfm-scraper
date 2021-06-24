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

-- index on tracks, artists, and albums
create index scrobbles_tracks on scrobbles (track);
create index scrobbles_tracks_artists on scrobbles (track, artist);
create index scrobbles_tracks_artists_albums on scrobbles (track, artist, album);
create index scrobbles_artists on scrobbles (artist);
create index scrobbles_albums on scrobbles (album);

-- scrobbles_local view

-- example:
-- select datetime('1622728549', 'unixepoch', 'localtime');

-- drop view if it exists
drop view if exists scrobbles_local;

create view scrobbles_local as
select track,
       artist,
       album,
       track_artist,
       album_artist,
       track_artist_album,
       loved,
       timestamp_utc,
       timestamp_local,
       scrobbled_today,
       track_first_scrobbled,
       CASE
           WHEN date(track_first_scrobbled) = date('now')
               THEN 'true'
           ELSE 'false'
           END track_first_scrobbled_today,
       track_album_first_scrobbled,
       CASE
           WHEN DATE(track_album_first_scrobbled) = DATE('now')
               THEN 'true'
           ELSE 'false'
           END track_album_first_scrobbled_today,
       date,
       year,
       month_num,
       CASE CAST(month_num AS integer)
           WHEN 1 THEN 'January'
           WHEN 2 THEN 'February'
           WHEN 3 THEN 'March'
           WHEN 4 THEN 'April'
           WHEN 5 THEN 'May'
           WHEN 6 THEN 'June'
           WHEN 7 THEN 'July'
           WHEN 8 THEN 'August'
           WHEN 9 THEN 'September'
           WHEN 10 THEN 'October'
           WHEN 11 THEN 'November'
           ELSE 'December'
           END month,
       day_num,
       CASE CAST(day_num AS integer)
           WHEN 0 THEN 'Sunday'
           WHEN 1 THEN 'Monday'
           WHEN 2 THEN 'Tuesday'
           WHEN 3 THEN 'Wednesday'
           WHEN 4 THEN 'Thursday'
           WHEN 5 THEN 'Friday'
           ELSE 'Saturday'
           END day,
       time,
       hour
from (
         select track,
                artist,
                album,
                track_artist,
                album_artist,
                track_artist_album,
                loved,
                timestamp_utc,
                timestamp_local,
                (SELECT MIN(datetime(timestamp_utc, 'unixepoch', 'localtime'))
                 FROM scrobbles fs
                 WHERE fs.track = sl.track
                   AND fs.artist = sl.artist)                    track_first_scrobbled,
                (SELECT MIN(datetime(timestamp_utc, 'unixepoch', 'localtime'))
                 FROM scrobbles fs
                 WHERE fs.track = sl.track
                   AND fs.artist = sl.artist
                   AND fs.album = sl.album)                      track_album_first_scrobbled,
                date(timestamp_local)                            date,
                strftime('%Y', timestamp_local)                  year,
                strftime('%m', timestamp_local)                  month_num,
                strftime('%d', timestamp_local)                  day_num,
                time(timestamp_local)                            time,
                CAST(strftime('%H', timestamp_local) AS integer) hour,
                CASE
                    WHEN date(timestamp_local) = date('now')
                        THEN 'true'
                    ELSE 'false'
                    END                                          scrobbled_today
         from (
                  select track,
                         artist,
                         album,
                         (track || ' - ' || artist)                        track_artist,
                         (album || ' - ' || artist)                        album_artist,
                         (track || ' - ' || artist || ' - ' || album)      track_artist_album,
                         case when loved = 0 then 'false' else 'true' end  loved,
                         timestamp_utc,
                         datetime(timestamp_utc, 'unixepoch', 'localtime') timestamp_local
                  from scrobbles
              ) sl
     ) s

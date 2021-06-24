DROP TABLE IF EXISTS all_dates_tbl;
DROP VIEW IF EXISTS all_dates;

-- Will simply hold a static list of dates
CREATE TABLE all_dates_tbl
(
    id   integer primary key,
    date datetime not null
);

-- Ensure each date can only be inserted once
create unique index if not exists ux_all_dates_date
    on all_dates_tbl (date);

-- Generates a list of all dates from the first scrobble to the current day
-- This will allow us to account for days that do not have scrobbles when calculating statistics
insert into all_dates_tbl (date)
WITH RECURSIVE dates(date) AS (
    VALUES ('2003-01-01')
    UNION ALL
    SELECT date(date, '+1 day')
    FROM dates
    WHERE date < ('2099-12-31')
)
SELECT date
from dates;

-- Create view that contains additional formatted columns
CREATE VIEW IF NOT EXISTS all_dates AS
SELECT ad.date,
       strftime('%Y', date) year,
       strftime('%m', date) month_num,
       strftime('%d', date) day_num,
       date = date('now')   is_today
FROM all_dates_tbl ad;

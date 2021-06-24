DROP VIEW IF EXISTS all_dates;

-- Generates a list of all dates from the first scrobble to the current day
-- This will allow us to account for days that do not have scrobbles when calculating statistics
CREATE VIEW all_dates AS
WITH RECURSIVE dates(date) AS (
    SELECT MIN(date)
    FROM scrobbles_local
    UNION ALL
    SELECT date(date, '+1 day')
        FROM dates
        WHERE date < date('now')
        )
SELECT date,
    strftime('%Y', date) year,
    strftime('%m', date) month_num,
    strftime('%d', date) day_num,
    date = date('now') is_today
from dates;
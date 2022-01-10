SELECT strftime('%H:%M', time_local),
(SELECT AVG(temp_external) FROM temperatures WHERE time_local BETWEEN datetime(t.time_local, '-5 minutes') AND datetime(t.time_local)) as mavg_external,
(SELECT AVG(temp_internal) FROM temperatures WHERE time_local BETWEEN datetime(t.time_local, '-5 minutes') AND datetime(t.time_local)) as mavg_internal
FROM temperatures t
WHERE time_utc > datetime('now', '-24 hours') AND strftime('%M', time_local) % 5 = 0 ORDER BY 1;

SELECT
IFNULL(AVG(temp_internal), 0),
IFNULL(AVG(temp_external), 0)
FROM temperatures WHERE time_local BETWEEN datetime('2020-01-10 16:15:57', '-5 minutes') AND datetime('2020-01-10 16:15:57')
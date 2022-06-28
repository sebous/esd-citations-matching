-- SQLite
SELECT esdcases.*, esdcaseinfos.*
FROM esdcases
LEFT JOIN esdcaseinfos ON esdcases.id = esdcaseinfos.case_id
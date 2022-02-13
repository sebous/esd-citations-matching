-- SQLite
SELECT esdcases.*, esdrelatedcases.*
FROM esdcases
LEFT JOIN esdrelatedcases ON esdcases.id = esdrelatedcases.parent_case_id
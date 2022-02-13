-- SQLite
SELECT esdcases.id, esdcases.code
FROM esdcases
UNION
SELECT esdcases.id, esdrelatedcases.code
FROM esdrelatedcases
INNER JOIN esdcases ON esdrelatedcases.parent_case_id = esdcases.id
ORDER BY esdcases.id
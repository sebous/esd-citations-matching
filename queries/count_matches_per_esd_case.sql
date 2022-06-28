-- SQLite
SELECT counted.count_of_matches,
  esdcases.*
FROM (
    SELECT esdcases.id,
      COUNT(matches.id) as count_of_matches
    FROM esdcases
      LEFT JOIN matches ON matches.matched_case_id = esdcases.id
    GROUP BY esdcases.id
    HAVING count_of_matches > 0
    ORDER BY count_of_matches DESC
  ) counted
  LEFT JOIN esdcases ON esdcases.id = counted.id
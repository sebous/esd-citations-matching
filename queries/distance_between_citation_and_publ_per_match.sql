-- SQLite
SELECT m.id as match_id,
  JULIANDAY(s.date) - JULIANDAY(e.date) as distance_between_publication_citation_in_days
FROM matches m
  INNER JOIN sourcecases s ON s.id = m.source_case_id
  INNER JOIN esdcases e ON e.id = m.matched_case_id
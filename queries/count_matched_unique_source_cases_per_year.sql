-- SQLite
SELECT strftime('%Y', dates.d) as year,
  COUNT(cases.id) as source_cases_count
FROM (
    WITH RECURSIVE dates(date) AS (
      VALUES('1993-01-01')
      UNION ALL
      SELECT date(date, '+1 year')
      FROM dates
      WHERE date < date('now', 'start of year')
    )
    SELECT date as d
    FROM dates
  ) as dates
  LEFT JOIN (
    SELECT sourcecases.date as d,
      sourcecases.id
    FROM sourcecases
      INNER JOIN matches ON sourcecases.id = matches.source_case_id
    WHERE d IS NOT NULL
    GROUP BY sourcecases.id
  ) cases ON cases.d >= dates.d
  AND cases.d <= date(dates.d, '+1 year', '-1 day')
GROUP BY year
HAVING source_cases_count > 0;
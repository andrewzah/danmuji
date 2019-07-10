WITH filter as (
  SELECT m.*
  FROM messages m
  LEFT JOIN channels c
  ON c.channel_id = m.channel_id
  LEFT JOIN users u
  ON u.user_id = m.user_id
  WHERE c.enabled is NOT false
  AND u.opt_out IS NOT true
),
summ AS
(
  SELECT
    user_id, SUM(hangeul_count) as sum_hangeul_count,
    SUM(non_hangeul_count) as sum_non_hangeul_count,
    SUM(raw_count) as sum_raw_count,
    COUNT(*) as sum_messages
  FROM filter
  GROUP BY user_id
), map AS (
   SELECT summ.user_id,
      summ.sum_messages,
      sum_hangeul_count::integer as sum_hangeul_count,
      summ.sum_non_hangeul_count::integer as sum_non_hangeul_count,
      sum_raw_count::integer as sum_raw_count,
      sum_hangeul_count::float / sum_raw_count::float as ratio
   FROM summ
)
SELECT * FROM map ORDER BY ratio DESC, sum_raw_count DESC;

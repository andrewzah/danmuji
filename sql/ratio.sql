WITH cte_a AS
(
  SELECT
    user_id, SUM(hangeul_count) as sum_hangeul_count,
    SUM(non_hangeul_count) as sum_non_hangeul_count,
    SUM(raw_count) as sum_raw_count,
    COUNT(*) as sum_messages
  FROM messages
  GROUP BY user_id
), cte_b AS (
   SELECT cte_a.user_id,
      cte_a.sum_messages,
      sum_hangeul_count::integer as sum_hangeul_count,
      cte_a.sum_non_hangeul_count::integer as sum_non_hangeul_count,
      sum_raw_count::integer as sum_raw_count,
      sum_hangeul_count::float / sum_raw_count::float as ratio
   FROM cte_a
)
SELECT * FROM cte_b ORDER BY ratio DESC, sum_raw_count DESC;

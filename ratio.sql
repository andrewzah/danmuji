WITH cte_a AS
(
  SELECT
    user_id, SUM(hangeul_count) as hangeul_count,
    SUM(non_hangeul_count) as non_hangeul_count,
    SUM(raw_count) as raw_count,
    COUNT(user_id) as total_messages
  FROM messages
  GROUP BY user_id
), cte_b AS (
   SELECT cte_a.*,
      hangeul_count::float / raw_count::float as ratio
   FROM cte_a
)
SELECT * FROM cte_b ORDER BY ratio, total_messages DESC;

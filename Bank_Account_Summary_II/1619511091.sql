# Write your MySQL query statement below
SELECT
  Users.name,
  SUM(amount) AS balance
FROM Users
INNER JOIN Transactions
  USING (account)
GROUP BY 1
HAVING balance > 10000;


SELECT name AS names
  FROM Staff
 WHERE S.place = (SELECT id
                    FROM Countries
                   WHERE continent = 'Europe');
SELECT name
  FROM Staff;

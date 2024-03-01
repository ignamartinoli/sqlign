  SELECT name AS names 
    FROM Staff 
   WHERE S . place = ( SELECT id FROM Countries WHERE continent = ' Europe ' ) 
ORDER BY name ;
SELECT name 
  FROM Staff ;

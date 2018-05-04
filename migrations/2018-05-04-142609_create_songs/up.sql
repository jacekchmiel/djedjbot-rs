CREATE TABLE songs (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  tempo SMALLINT
  -- other data will come along later
);

--INSERT INTO songs (title, tempo) VALUES
--    ('Test 1', 100),
--    ('Test 2 notempo', NULL);
-- migrate:up
CREATE TABLE IF NOT EXISTS media_datas (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL
);


--migrate:down
DROP TABLE IF EXISTS media_datas;

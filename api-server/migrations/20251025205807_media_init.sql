-- migrate:up
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS media_datas (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    path TEXT NOT NULL
);

--migrate:down
DROP TABLE IF EXISTS media_datas;
DROP EXTENSION IF EXISTS "uuid-ossp";

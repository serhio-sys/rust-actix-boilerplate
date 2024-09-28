CREATE TABLE IF NOT EXISTS sessions
(
    user_id INTEGER NOT NULL,
    uuid    UUID    NOT NULL,
    CONSTRAINT auths_pkey PRIMARY KEY (user_id, uuid)
);


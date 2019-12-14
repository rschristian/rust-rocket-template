CREATE TABLE users
(
    id                   SERIAL  PRIMARY KEY,
    first_name           TEXT    NOT NULL,
    last_name            TEXT    NOT NULL,
    email                TEXT    NOT NULL,
    hashed_password      TEXT    NOT NULL,
--     activated            BOOLEAN NOT NULL DEFAULT FALSE,
--     authentication_token TEXT,
--     expiry_datetime      TIMESTAMP WITH TIME ZONE,
    CONSTRAINT unique_email UNIQUE (email)
);

ALTER SEQUENCE users_id_seq RESTART WITH 1;

INSERT INTO users (first_name, last_name, email, hashed_password)
VALUES ('smoke', 'test', 'smoketest@example.com', '$rscrypt$0$DggB$AFR6Zk+AY16xtD9KAor9tw==$k7V3U+gadVDLCWwS41qwd09pt0x/WFDtidUj/S8MhJ4=$');
create table posts
(
    id        TEXT                          not null,
    pid       integer,
    title     TEXT,
    link      TEXT,
    published integer default 'unixepoch()' not null,
    country   TEXT,
    attacker  TEXT    default unknown       not null
);
create unique index id_index on posts (id);

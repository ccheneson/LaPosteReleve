-- Your SQL goes here

CREATE TABLE activities (
    id              SERIAL,
    date            DATE NOT NULL,
    statement       TEXT NOT NULL,
    amount          FLOAT4 NOT NULL,
    PRIMARY KEY ( date, statement, amount)
);



CREATE TABLE balances (
    id              SERIAL,
    date            DATE NOT NULL,
    amount          FLOAT4 NOT NULL,
    PRIMARY KEY ( date, amount)
);


CREATE TABLE tags (
    id              INTEGER NOT NULL,
    tag             TEXT NOT NULL
);



CREATE TABLE tags_patterns (
    id              INTEGER NOT NULL,
    tags_pattern      TEXT NOT NULL
);


CREATE TABLE tags_pattern_to_tags (
    id                SERIAL,
    tags_pattern_id   INTEGER NOT NULL,
    tags_id   INTEGER NOT NULL,
    PRIMARY KEY ( tags_pattern_id, tags_id) 
);


CREATE TABLE activity_tags (
    id              SERIAL,
    activity_id       INTEGER NOT NULL,
    tags_pattern_id   INTEGER NOT NULL,
    PRIMARY KEY ( activity_id, tags_pattern_id) 
);



INSERT INTO tags (id, tag)
VALUES 
(1, 'EDF'),
(2, 'FREEMOBILE'), 
(3, 'LOYER'),
(4, 'PARIS'),                
(5, 'RETRAIT'),
(6, 'VIREMENT_BANCAIRE');



INSERT INTO tags_patterns (id, tags_pattern)
VALUES 
(1, 'EDF'),                
(2, 'VIREMENT'),
(3, 'RETRAIT'),
(4, 'LOYER'),
(5, 'FREE MOBILE');




INSERT INTO tags_pattern_to_tags (tags_pattern_id, tags_id)
VALUES 
(1, 1),
(2, 6),
(3, 5),
(4, 3), (4, 4),
(5, 2), (5, 4);

SET search_path TO productoffour;

CREATE TABLE oddonlyresults (
    sqrt BIGINT NOT NULL,
    sigma2 VARCHAR(75) NULL,
    CONSTRAINT oddonlyresults_pk PRIMARY KEY (sqrt)
);

CREATE TABLE pairs (
    sqrt BIGINT NOT NULL,
    sequenceStart BIGINT NOT NULL,
    increment INT NULL,
    CONSTRAINT pairs_pk PRIMARY KEY (sqrt, sequenceStart)
);

CREATE TABLE factors (
    sqrt BIGINT NOT NULL,
    factor BIGINT NOT NULL,
    count INT DEFAULT 0,
    CONSTRAINT factors_pk PRIMARY KEY (sqrt, factor)
);
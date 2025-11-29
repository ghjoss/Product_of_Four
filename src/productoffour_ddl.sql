-- productoffour.allresults definition

-- Drop table

-- DROP TABLE productoffour.allresults;

CREATE TABLE productoffour.allresults (
	sqrt int8 NOT NULL,
	sigma2 varchar(75) NULL,
	CONSTRAINT allresults_pk PRIMARY KEY (sqrt)
);


-- productoffour.factors definition

-- Drop table

-- DROP TABLE productoffour.factors;

CREATE TABLE productoffour.factors (
	sqrt int8 NOT NULL,
	factor int8 NOT NULL,
	count int4 NOT NULL,
	CONSTRAINT factors_pk PRIMARY KEY (sqrt, factor)
);


-- productoffour.oddonlyresults definition

-- Drop table

-- DROP TABLE productoffour.oddonlyresults;

CREATE TABLE productoffour.oddonlyresults (
	sqrt int8 NOT NULL,
	sigma2 varchar(75) NULL,
	sigma2_of_sequence_start varchar(75) NULL,
	CONSTRAINT oddonlyresults_pk PRIMARY KEY (sqrt)
);


-- productoffour.pairs definition

-- Drop table

-- DROP TABLE productoffour.pairs;

CREATE TABLE productoffour.pairs (
	sqrt int8 NOT NULL,
	sequencestart int8 DEFAULT 0 NOT NULL,
	"increment" int4 NULL,
	CONSTRAINT pairs_pk PRIMARY KEY (sqrt, sequencestart)
);


-- productoffour.pairs2 definition

-- Drop table

-- DROP TABLE productoffour.pairs2;

CREATE TABLE productoffour.pairs2 (
	sqrt int8 NOT NULL,
	start_incr1 int8 DEFAULT 0 NOT NULL,
	start_incr2 int8 NULL,
	CONSTRAINT pairs_pk2 PRIMARY KEY (sqrt, start_incr1)
);

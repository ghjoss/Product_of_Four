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
CREATE UNIQUE INDEX idx_factors_sqrt_factor_count 
ON factors (sqrt, factor, count);


-- productoffour.oddonlyresults definition

-- Drop table

-- DROP TABLE productoffour.oddonlyresults;

CREATE TABLE productoffour.oddonlyresults (
	sqrt int8 NOT NULL,
	sigma2 varchar(75) NULL,
	sigma2_of_sequence_start varchar(75) NULL,
	CONSTRAINT oddonlyresults_pk PRIMARY KEY (sqrt)
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

-- For the reverse symmetric pairs and the WHERE filter
CREATE UNIQUE INDEX idx_pairs2_reverse_lookup 
ON pairs2 (start_incr2, start_incr1, sqrt);

CREATE UNIQUE INDEX idx_pairs2_lookup
ON pairs2 (start_incr1, start_incr2);


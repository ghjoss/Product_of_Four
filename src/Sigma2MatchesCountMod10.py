#!/usr/bin/env python3
"""
query the three tables in the Pof4 database tables to create a set of spreadsheets
Each sheet is approximately 100,000 rows.
"""

import os
from pathlib import Path
        


import psycopg
from psycopg.rows import dict_row
        
# CONFIG (via env vars or hard-code)
try:
    RECORDS_PER_SHEET    = int(os.getenv("RECORDS_PER_SHEET","100000"))
except ValueError:
    RECORDS_PER_SHEET    = 100000

DB_HOST              = os.getenv("PGHOST", "localhost")
DB_PORT              = int(os.getenv("PGPORT", "5432"))
DB_NAME              = os.getenv("PGDATABASE", "Pof4")
DB_USER              = os.getenv("PGUSER", "postgres")
DB_PASS              = os.getenv("PGPASSWORD", "gr3at ch1cken gh0st shaver")

COL_SQRT             = os.getenv("COL_SQRT","sqrt")

PAIRS_TABLE          = os.getenv("PAIRS_TABLE", "pairs2")
# COL_SQRT
COL_START_INCR1      = os.getenv("BASE_COLUMN", "start_incr1")
COL_START_INCR2      = os.getenv("INCREMENT_COLUMN", "start_incr2")

FACTORS_TABLE        = os.getenv("FACTORS_TABLE","factors")
# COL_SQRT
COL_FACTOR           = os.getenv("COL_FACTOR","factor")
COL_COUNT            = os.getenv("COL_COUNT","count")

ODDONLYRESULTS_TABLE = os.getenv("ODDONLYRESULTS_TABLE","oddonlyresults")
# COL_SQRT
COL_SIGMA2           = os.getenv("COL_SIGMA2","sigma2")
COL_SIGMA2_MOD10     = os.getenv("COL_SIGMA2_MOD10","sigma2mod10")
COL_SIGMA2_MOD100    = os.getenv("COL_SIGMA2_MOD100","sigma2mod100")

# SQL to get distinct sqrt and counts (the "second select")
#  Merge pairs2, oddonlyresults and factors.
#  
#  To save space, when pairs2 data is generated rows with start_incr1=N and start_incr2=K are stored but
#  rows with start_incr1=K and start_incr2=N are not stored. To correct this all rows in pairs2 are
#   combined with a UNION ALL with rows in pair2. In the second half of the union, rows wiht K=N are not 
#   included. Thus for pairs2 row: sqrt=605, start_incr1=4, start_incr2=19 and pairs2 row: sqrt=605, start_incr1=11, 
#   start_incr2=11, a third row will be created in the union: sqrt=605, start_incr1=19, start_incr2=4.
#   This common table expression (CTE) is called CombinedPairs. CombinedPairs is then used to create CTE
#   SquareRootCounts consisting of rows with matching sqrt (squareRoot) values. Lastly CTE 
#   AggregatedFactors is created by using the AGGREGATE keyword to combine all factors with
#   matching square roots into a single row.
#   
#   A query then joins the three CTEs and the oddOnlyResults table.
#  
#   Sample output:
#   "squareroot","base","increment","σ₂([base=increment]²)","count","σ₂ mod 100","σ₂ mod 10","factors of sqrt   "
#   5,1,1,1,1,1,1,"5^1"
#   20,2,2,21,1,21,1,"2^2 x 5^1"
#   45,3,3,91,1,91,1,"3^2 x 5^1"
#   80,4,4,341,1,41,1,"^4  x  5^1"
#   125,5,5,651,1,51,1,"5^3"
#   180,6,6,1,911,1,11,1,"2^2 x 3^2 x 5^1"
SELECT_SQL = f"""
WITH CombinedPairs AS (
    -- Defines the full set of pairs (N,K) where N <= K, and the reverse pairs (K,N)
    SELECT p1.{COL_SQRT} AS squareRoot, p1.{COL_START_INCR1} AS base, p1.{COL_START_INCR2} AS increment
    FROM {PAIRS_TABLE} p1 
    UNION ALL
    SELECT p2.{COL_SQRT}, p2.{COL_START_INCR2}, p2.{COL_START_INCR1} FROM {PAIRS_TABLE} p2
    WHERE p2.{COL_START_INCR1} <> p2.{COL_START_INCR2} 
),
SquareRootCounts AS (
    -- Calculates the count of pairs for each squareRoot
    SELECT squareRoot, COUNT(squareRoot) AS ct
    FROM CombinedPairs
    GROUP BY squareRoot
),
AggregatedFactors AS (
    -- Aggregates the prime factors into a single string for each sqrt
    SELECT
        f.{COL_SQRT},
        STRING_AGG(
            CAST(f.{COL_FACTOR} AS TEXT) || '^' || CAST(f.{COL_COUNT} AS TEXT), 
            '  x  ' 
            ORDER BY f.{COL_FACTOR}
        ) AS factor_list
    FROM {FACTORS_TABLE} f
    GROUP BY f.{COL_SQRT}
)
-- FINAL QUERY: Selects from the three CTEs and the oddonlyresults table
SELECT
    p.squareRoot,
    p.base, 
    p.increment, 
    o.{COL_SIGMA2} AS sigma2, 
    grp.ct AS "count",
    o.sigma2mod100 AS "s2Mod100",
    o.sigma2mod10 AS "s2Mod10",
    f.factor_list AS "factorization"
FROM CombinedPairs p 
JOIN {ODDONLYRESULTS_TABLE} o ON p.squareRoot = o.{COL_SQRT}
JOIN SquareRootCounts grp ON p.squareRoot = grp.squareRoot
LEFT JOIN AggregatedFactors f ON f.{COL_SQRT} = p.squareRoot
ORDER BY p.squareRoot, p.base"""

# write_header()
#  bump the current_file header and append to the file name template. Open this
#  file for write access in the passed directory and write a sheet header line.
#  passed parameters:
#       current_file:   a numeric file number which will be incremented to form a unique file name
#       output_dir:     The directory where the file will be saved
#  returns:
#       current_file    The incremented file number
#       f               The open file handle
def write_header(current_file,output_dir):
    current_file += 1
    outfile = output_dir / f"Sigma2MatchesCountMod10_{current_file}.csv"
    f = open(outfile,"w",encoding="utf-8")
    print('"square root","base","increment","σ₂([base=increment]²)","count","σ₂() mod 100","σ₂() mod 10","factors of square root"',file=f)
    return current_file, f

# connect()
# Connect to the postgresSQL server and create a cursor.
# passed parameters:
#       none
# returns:
#       conn:           The connection object to the server
#       cur:            A cursor for iterating the returned query rows
def connect():
    conn = psycopg.connect(host=DB_HOST, port=DB_PORT, dbname=DB_NAME, user=DB_USER, password=DB_PASS)
    cur = conn.cursor(row_factory=dict_row)
    return conn, cur


def main():
    # determine the output directory. In this case it will be the parent directory of the program.
    # if the program directory is root (/), the output_dir will be the root directory.
    # 
    output_dir = Path(__file__).resolve().parent
    
    # if the sheets are to be placed in the program's directory, comment out the following two lines.
    output_dir_parent = output_dir.parent
    output_dir = output_dir_parent if output_dir_parent != output_dir else Path(output_dir_parent.root)
    
    conn, cur = connect()
    print("Query Starting...")
    cur.execute(SELECT_SQL)
    print("...end")
    
    print("Fetch starting...")
    rows = cur.fetchall()
    print("...complete")
    
    current_file = 0
    current_file,f = write_header(current_file,output_dir)
    records_processed = 1 # the header is record 1
    last_square_root = -1
    
    with conn.transaction():
        processed = 0
        for r in rows:
            # get the column data for row r
            squareRoot = r.get("squareroot")
            base = r.get("base")
            increment = r.get("increment")
            sigma2 = r.get("sigma2")
            count = r.get("count")
            sigma2mod100 = r.get("s2Mod100")
            sigma2mod10 = r.get("s2Mod10")
            factors = r.get("factorization")

            records_processed += 1

            # if the RECORDS_PER_SHEET count has been reached and if the current record is 
            # not part of a group of rows having the same squareRoot,
            # then close the current file and start a new one with a new header line and 
            # the current record as the second row.
            if records_processed >= RECORDS_PER_SHEET and last_square_root != squareRoot:
                f.flush()
                f.close()
                current_file,f = write_header(current_file,output_dir)
                records_processed = 2 # reset to header + current row 

            # write a comma separated list            
            print(f'{squareRoot},{base},{increment},{sigma2},{count},{sigma2mod100},{sigma2mod10},"{factors}"',file=f)

            processed += 1
            if processed < 100 or processed % 10000 == 0:
                print(f'{processed} rows fetched')
                print(f'{squareRoot},{base},{increment},{sigma2},{count},{sigma2mod100},{sigma2mod10},"{factors}"')
                f.flush()
            last_square_root = squareRoot

    cur.close()
    conn.close()
    f.close()
    print(f"{processed+1} rows written to {current_file} files")

if __name__ == "__main__":
    main()


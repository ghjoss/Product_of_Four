#!/usr/bin/env python3
"""
transfer_counts.py
Query pairs2 for distinct sqrt values and their counts, compute a value, and insert into oddonlyresults.
"""

import os
import sys
import math
        

# Try psycopg (psycopg3) then psycopg2
import psycopg
from psycopg.rows import dict_row
        
# CONFIG (via env vars or hard-code)
DB_HOST = os.getenv("PGHOST", "localhost")
DB_PORT = int(os.getenv("PGPORT", 5432))
DB_NAME = os.getenv("PGDATABASE", "Pof4")
DB_USER = os.getenv("PGUSER", "postgres")
DB_PASS = os.getenv("PGPASSWORD", "gr3at ch1cken gh0st shaver")

SRC_TABLE = os.getenv("SRC_TABLE", "pairs2")
SRC_COLUMN = os.getenv("SRC_COLUMN", "sqrt")
base = os.getenv("BASE_COLUMN", "start_incr1")
increment = os.getenv("INCREMENT_COLUMN", "start_incr2")
DST_TABLE = os.getenv("DST_TABLE", "oddonlyresults")
# assume oddonlyresults has columns: sqrt, calculated_value
DST_COLUMNS = os.getenv("DST_COLUMNS", "sqrt, sigma2")  # comma-separated if needed

# SQL to get distinct sqrt and counts (the "second select")
SELECT_SQL = f"""
SELECT {SRC_COLUMN} AS sqrt_value,{base} AS base_value
FROM {SRC_TABLE}
WHERE {base} = {increment}
ORDER BY sqrt ASC;
"""

# Insert SQL placeholder (parameterized)
# This expects oddonlyresults to have two columns: sqrt, calculated_value
INSERT_SQL = f"""
INSERT INTO {DST_TABLE} (sqrt, sigma2)
VALUES (%s, %s)
ON CONFLICT (sqrt) DO NOTHING
"""

def sigma_2_of_square(n: int) -> int:
    """
    Compute sigma_2(n^2) = sum_{d|n^2} d^2.
    Uses divisor generation from prime factorization for efficiency.
    """
    if n == 0:
        return 0  # define sigma_2(0) as 0 here
    n_abs = abs(n)
    # trial division prime factorization
    factors = {}
    x = n_abs
    # factor out 2s
    while x % 2 == 0:
        factors[2] = factors.get(2, 0) + 1
        x //= 2
    f = 3
    limit = int(math.isqrt(x)) + 1
    while f <= limit and x > 1:
        while x % f == 0:
            factors[f] = factors.get(f, 0) + 1
            x //= f
            limit = int(math.isqrt(x)) + 1
        f += 2
    if x > 1:
        factors[x] = factors.get(x, 0) + 1

    # For n^2, each prime exponent doubles: e -> 2e.
    # Formula for sigma_k for prime p with exponent a: (p^{k(a+1)} - 1) / (p^k - 1)
    # here k = 2, a = 2e
    result = 1
    for p, e in factors.items():
        a = 2 * e
        numerator = pow(p, 2 * (a + 1)) - 1  # p^{2(a+1)} - 1
        denominator = pow(p, 2) - 1         # p^2 - 1
        term = numerator // denominator
        result *= term
    return result

def connect():
    conn = psycopg.connect(host=DB_HOST, port=DB_PORT, dbname=DB_NAME, user=DB_USER, password=DB_PASS)
    cur = conn.cursor(row_factory=dict_row)
    return conn, cur

def main():
    conn, cur = connect()
    cur.execute(SELECT_SQL)
    rows = cur.fetchall()
    # Use a transaction for inserts
    with conn.transaction():
        inserted = 0
        for r in rows:
            sqrt_value = r.get("sqrt_value")
            base_value = r.get("base_value")
            calc = sigma_2_of_square(base_value)
            # Use DB param style for psycopg3 is %s as well for execute with list/tuple
            cur.execute(INSERT_SQL, (sqrt_value, calc))
            inserted += 1
            if inserted % 100 == 0:
                print(f"{inserted} rows processed")

        
    print(f"Inserted {inserted} rows into {DST_TABLE}")
    cur.close()
    conn.close()

if __name__ == "__main__":
    main()

The product of four integers in an arithmetic progression of four integers when 
added to the difference value raised to the fourth power is always a squared integer
value. The function f(n,k) is defined as n * n+k * n+2k * n+3k + k⁴, where n is the
start of the progression and k is the difference (or increment).
As an example, for the f(3,4) = 3 * 7 * 11 * 15  (difference = 4):

     3 * 7 * 11 * 15 + 4⁴ = 3465 + 256 = 3721 = 61²

See the On-line Encyclopedia of Integer Sequences (OEIS) article #A062938 
(https://oeis.org/A062938). This sequence, generalized for all differences (not only inc=1), 
is used in the code below to evaluate the square roots without having to use the sqrt() function.
In general for initial value n and increment (difference) k, √f(n,k) = (n*k + (n+k)²).

Thus √f(3,4) (61 in the above example) can be computed by the
formula: (3*4) * (3+4)² = 12 + 7² = 12 + 49 = 61

By associativity, f(n,k) = f(k,n), because both have the same square root. Thus,
for n ≠ k, there are at least two n,k :: k,n pairs that have the same root. There
can be more. When n = k, there is at least one result f(n,n). However, there can
be other results. So, when n = k there is always an odd number of (n,k) pairs where
√f(n,k) generates the same number. This program loops through 1 to s values, computing 
√f(s,s) and then searches for any other pairs of f(n,k),f(k,n) that generate the same 
value: √f(s,s). 

Also, see the On-line Encyclopedia of Integer Sequences (OEIS) article #A062938 (https://oeis.org/A062938)
This sequence, generalized for all differences (not only inc=1), is used in the
code below to evaluate the square roots without having to use the sqrt() function. 

main()
process the JSON file Product_of_Four.json which has program control parameters. Display those
parameters and then connect to the mysql database Pof4. The credentials for this connect are
in the JSON file value "connection_string".

If the JSON file indicates that the database tables are to be scratched and created anew
(i.e. "drop_and_create_tables" = true), then redefine the tables pairs and oddOnlyResults.
The actual table names are defined in the JSON entries "pairs_table" and "odd_only_results_table".

Iterate from the JSON start value ("start_of_loop") through the end value ("end_of_loop") calculating
√f(s,s) where s is the iteration value. Calculate the sigma_2(√f(s,s)) modulo 100. Insert records
in the oddOnlyResults and pairs tables.

Call get_pairs() to look for any other n,k values that generate the same result of √f(s,s).

Args: program has no input arguments.
Result: Ok or error.

Notes:
 
 Three different ways to calculate f(n,k):
 1:
 n * (n+k) * (n + 2*k) * (n+3*k) + k⁴
  = n * (n+2*k) * (n+k)*(n+3*k) + k⁴
  = (n²+2*k*n) * (n² + 4*k*n + 3*k²) + k⁴
  = (n²*n² + 2*k*n*n²) + (4*k*n*n² + 4*k*n*2*k*n) + (3*k² * n² + 3*k² * 2*k*n) + k⁴
  = n⁴	 + 2*k*n³	 + 4*k*n³	 + 8*k²*n²	      +  3*k²*n² + 6*k³*n		   + k⁴
  = n⁴     + 6*k*n³				 + 11*k²*n²                  + 6*k³*n		   + k⁴
 
 2: (showing how √f(n,k) was derived )
  (n*k + (n+k)²)²                          <== (√f(n,k))²
  = (n*k + n² + 2*k*n + k²)²
  = (n*k + n² + 2*k*n + k²) * (n*k + n² + 2*k*n + k²)
  = (n*k*n*k + n²*n*k + 2*k*n*n*k + k²*n*k) + (n*k*n² + n²*n² + 2*k*n*n² + k²*n²) + (n*k*2*k*n + n²*2*k*n + 2*k*n*2*k*n + k²*2*k*n) + (n*k*k² + n²*k² + 2*k*n*k²) + (k²*k²)
  = (k²*n² + k*n³ + 2*k²*n² + k³*n) + (k*n³ + n⁴ + 2*k*n³ + k²*n²) + (2*k²*n² + 2*k*n³ + 4*k²*n² + 2*k³*n) + (k³*n + k²*n² + 2*k³*n + k⁴)
   = n⁴ + (k*n³ + k*n³ + 2*k*n³ + 2*k*n³) + (k²*n² + 2*k²*n² + k²*n² + 2*k²*n² + 4*k²*n² + k²*n²) + (k³*n + 2*k³*n + k³*n + 2*k³*n) + k⁴
   = n⁴     + 6*k*n³               + 11*k²*n²                  + 6*k³*n         + k⁴

  3:
   (n² + 3*k*n + k²)²
   = (n² + 3*k*n + k²) * (n² + 3*k*n + k²)
   = n⁴ + 3*k*n³ + n²*k² + 3*k*n³ + 9*k²*n² + 3*k³*n + k²+n²  + 3*k³*n            + k⁴
   = n⁴ + (3*k*n³ + 3*k*n³) +        (n²*k² + 9*k²*n² +n²*k²) + (3*k³*n + 3*k³*n) + k⁴
   = n⁴    + 6*k*n³                 + 11*k²*n²                + 6*k³*n            + k⁴ 

use sqlx::{PgPool};
use std::error::Error;
use std::env::{self};
use std::fs;
use std::str::FromStr;
use std::process;
use tokio;
use tokio::io::{self, stdin, AsyncBufReadExt, BufReader};
use serde_json;
use bigdecimal::BigDecimal;
use num_bigint::{BigUint,ToBigInt};
use num_traits::{One};
  
///	The product of four integers in an arithmetic progression of four integers when 
///	added to the difference value raised to the fourth power is always a squared integer
///	value. The function f(n,k) is defined as n * n+k * n+2k * n+3k + k⁴, where n is the
/// start of the progression and k is the difference (or increment).
/// As an example, for the f(3,4) = 3 * 7 * 11 * 15  (difference = 4):
///
///					3 * 7 * 11 * 15 + 4⁴ = 3465 + 256 = 3721 = 61²
/// 
/// See the On-line Encyclopedia of Integer Sequences (OEIS) article #A062938 
/// (https://oeis.org/A062938). This sequence, generalized for all differences (not only inc=1), 
/// is used in the code below to evaluate the square roots without having to use the sqrt() function.
/// In general for innow returns (string, last_digit)itial value n and increment (difference) k, √f(n,k) = (n*k + (n+k)²).
/// 
/// Thus √f(3,4) (61 in the above example) can be computed by the
/// formula: (3*4) + (3+4)² = 12 + 7² = 12 + 49 = 61
///
/// By associativity, f(n,k) = f(k,n), because both have the same square root. Thus,
/// for n ≠ k, there are at least two n,k :: k,n pairs that have the same root. There
/// can be more. When n = k, there is at least one result f(n,n). However, there can
/// be other results. So, when n = k there is always an odd number of (n,k) pairs where
/// √f(n,k) generates the same number. This program loops through 1 to s values, computing 
/// √f(s,s) and then searches for any other pairs of f(n,k),f(k,n) that generate the same 
/// value: √f(s,s). 
/// 
/// Also, see the On-line Encyclopedia of Integer Sequences (OEIS) article #A062938 (https://oeis.org/A062938)
///	This sequence, generalized for all differences (not only inc=1), is used in the
///	code below to evaluate the square roots without having to use the sqrt() function. 
///

/// main()
/// process the JSON file Product_of_Four.json which has program control parameters. Display those
/// parameters and then connect to the postgre database Pof4. The credentials for this connect are
/// in the JSON file value "connection_string".
/// 
/// If the JSON file indicates that the database tables are to be scratched and created anew
/// (i.e. "drop_and_create_tables" = true), then redefine the tables pairs and oddOnlyResults.
/// The actual table names are defined in the JSON entries "pairs_table" and "odd_only_results_table".
/// 
/// Iterate from the JSON start value ("start_of_loop") through the end value ("end_of_loop") calculating
/// √f(s,s) where s is the iteration value. Calculate the sigma_2(√f(s,s)) modulo 100. Insert records
/// in the oddOnlyResults and pairs tables.
/// 
/// Call get_pairs() to look for any other n,k values that generate the same result of √f(s,s).
/// 
/// Args: program has no input arguments.
/// Result: Ok or error.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // JSON file definition and reading
    let current_dir = env::current_dir()?;
    let file_name = "Product_Of_Four.json";
    let json_file_path = current_dir.join(file_name);
    println!("{}",json_file_path.display());
    let contents = fs::read_to_string(json_file_path)?;
 
    // JSON parsing
    // convert string into a serde_json::Value
    let v: serde_json::Value = serde_json::from_str(&contents)?;

    let start_of_loop = v["start"].as_i64().unwrap();
    let end_of_loop = v["end"].as_i64().unwrap();

    let odd_only_results_table: &str = v["oddOnlyResults_table"].as_str().unwrap();
    let pairs_table: &str = v["pairs_table"].as_str().unwrap();
    let drop_and_create_tables = v["drop_and_create_tables"].as_bool().unwrap();
    let db_port: &str = v["db_port"].as_str().unwrap();
    let db_ip_address: &str = v["db_ip_address"].as_str().unwrap();
    let db_name: &str = v["db_name"].as_str().unwrap();
    let install_dual_pairs = v["install_dual_pairs"].as_bool().unwrap();
    let pairs_table_columns: Vec<&str> = v["pairs_table_columns"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();

    let read_credentials_at_runtime = v["read_credentials_at_runtime"].as_bool().unwrap();
    let mut user = String::from("");
    let mut pwd = String::from("");

    if read_credentials_at_runtime {
        let stdin = stdin();
        let reader = io::BufReader::new(stdin);
        let mut lines = reader.lines();
        println!("Enter the database user id:");
        while let Some(line) = lines.next_line().await.unwrap() {
            if line.is_empty() {
                eprintln!("user id was not specified");
                process::exit(1);
            }
            user = line.clone();
            break;
        }
        println!("Enter the database password:");
        while let Some(line) = lines.next_line().await.unwrap() {
            if line.is_empty() {
                eprintln!("Password was not specified.");
                process::exit(1);
            }
            pwd = line.clone();
            break;
        }
    
    }
    else {
        let temp: &str = v["username"].as_str().unwrap();
        user = temp.to_string();
        let temp: &str = v["password"].as_str().unwrap();
        pwd = temp.to_string();
    }

    println!("Parameters:");
    println!("
    \tip_address:port : {}:{}\n \
    \tdatabase name: {}\n \
    \toddonlyresults table name: {} \
    \tpairs table name: {}\n\n",
    db_ip_address,db_port,db_name,odd_only_results_table,pairs_table);

// Read in table of primes
    let file_name = "_100KPrimes.txt";
    let primes_file_path = current_dir.join(file_name);
    println!("Reading {}",primes_file_path.display());
    let primes: Vec<u64> = match parse_file_to_vector(file_name).await {
        Ok(mut p) => {
            p.sort();
            p
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            // Decide if we should exit here or return an error
            return Err(Box::new(e) as Box<dyn Error>); 
        }
    };

    let connection_string = format!("postgres://{}:{}@{}:{}/{}",user,pwd,db_ip_address,db_port,db_name);
    // let pool = sqlx::postgres::PgPool::connect(connection_string).await?;
    let pool = PgPool::connect(&connection_string).await?;

    if drop_and_create_tables {
        println!("Table DROP and CREATE");
        let qry_str = format!("DROP TABLE IF EXISTS {}",pairs_table);
        let expect_str = format!("Failed to drop table '{}'",pairs_table);
        sqlx::query(&qry_str)
            .execute(&pool)
            .await
            .expect(&expect_str);

        let qry_str = format!("DROP TABLE IF EXISTS {}",odd_only_results_table);
        let expect_str = format!("Failed to drop table '{}'",odd_only_results_table);
        sqlx::query(&qry_str)
            .execute(&pool)
            .await
            .expect(&expect_str);

        let qry_str = format!(
            "CREATE TABLE {} ( \
                    {} BIGINT NOT NULL, \
                    {} BIGINT NOT NULL, \
                    {} BIGINT DEFAULT NULL, \
                    CONSTRAINT pairs_pk PRIMARY KEY ({},{}) \
            )",
            pairs_table,pairs_table_columns[0],pairs_table_columns[1],pairs_table_columns[2],
            pairs_table_columns[0],pairs_table_columns[1]
        );
        let expect_str = format!("Failed to create table '{}'",pairs_table);
        sqlx::query(&qry_str)
            .execute(&pool)
            .await
            .expect(&expect_str);

        let qry_str = format!(
            "CREATE TABLE {} ( \
                    sqrt BIGINT NOT NULL, \
                    sigma2 NUMERIC DEFAULT NULL, \
                    sigma2Mod10 SMALLINT GENERATED ALWAYS AS (mod(sigma2,10)) STORED, \
                    sigma2Mod100 SMALLINT GENERATED ALWAYS AS (mod(sigma2,100)) STORED, \
                    CONSTRAINT oddonlyresults_pk PRIMARY KEY (sqrt) \
            )",
            odd_only_results_table);
        let expect_str = format!("Failed to create table '{}'",odd_only_results_table);
        sqlx::query(&qry_str)
            .execute(&pool)
            .await
            .expect(&expect_str);
    }

    let mut transaction = pool.begin().await?;
    let mut trans_ct = 0;
    // s:u64 represents the equal start_of_sequence and difference (increment)
    for s_i64 in start_of_loop..end_of_loop {
        // sigma_2 returns u128 to avoid overflow
        let s:u64 = s_i64 as u64;
        let sigma2_num:u128 = sigma_2_dirichlet(s, &primes).await?;
        // New: Add .expect() to handle the Result
        let bd_sigma2 = BigDecimal::from_str(&sigma2_num.to_string()).expect("Failed to parse BigDecimal"); // compute sqrt safely in i128 (sqrt = (s*s) + (s+s)*(s+s) == 5*s*s)
        let s_i128 = s_i64 as i128;
        let sqrt_i128 = 5i128 * s_i128 * s_i128;
        if sqrt_i128 > i128::from(i64::MAX) {
            eprintln!("sqrt overflow for s={} (value {}) - skipping", s, sqrt_i128);
            continue;
        }
        let sqrt = sqrt_i128 as i64;
        println!("{sqrt} ==> {s}:{s} (sigma_2 : {sigma2_num})");

        trans_ct += 1;
        if trans_ct % 500 == 0 {
            transaction.commit().await?;
            transaction = pool.begin().await?;
        }
        let insert_str = format!(
            "INSERT INTO {} (sqrt, sigma2) VALUES ($1,$2) \
             ON CONFLICT (sqrt) DO NOTHING",
            odd_only_results_table);
        let expect_str = format!("Insert failed for {}",odd_only_results_table);
        sqlx::query(&insert_str)
            .bind(sqrt)
            .bind(bd_sigma2)
            .execute(&mut *transaction)
            .await
            .expect(&expect_str);

        let insert_str = format!(
            "INSERT INTO {} ({}, {}, {}) VALUES($1,$2,$3) \
             ON CONFLICT ({}, {}) DO NOTHING",
            pairs_table,
            pairs_table_columns[0],
            pairs_table_columns[1],
            pairs_table_columns[2],
            pairs_table_columns[0],
            pairs_table_columns[1]);
        let expect_str = format!("Failed insert for {} table",pairs_table);
        sqlx::query(&insert_str)
            .bind(sqrt)
            .bind(s_i64)
            .bind(s_i64)
            .execute(&mut *transaction)
            .await
            .expect(&expect_str);

        // find all pairs (sequenceStart "s"/increment "i") 
        // where (s*i) + (s+i)(s+i) generates the current sqrt value.
        //let sigma2_num = sigma2_last_digit as u64;
         
        get_pairs(s_i64, sqrt, &pool, sigma2_num, &pairs_table, &pairs_table_columns, install_dual_pairs).await?;

    if s%250 == 0 {
            println!("{s}");
        };
    }

    // commit any remaining work and finish main
    transaction.commit().await?;
    Ok(())
}

/// parse_file_to_vector: used to read the file of comma-separated
/// prime numbers.
/// arg: filename: the name of the file to read
/// result: a vector of the integers (prime numbers) in the file.
async fn parse_file_to_vector(filename: &str) -> Result<Vec<u64>, io::Error> {
    let mut numbers: Vec<u64> = Vec::new();

    // Open the file asynchronously
    let file = tokio::fs::File::open(filename).await?;
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        // Split the line by commas and parse each value as an integer
        for num_str in line.split(',') {
            if let Ok(num) = num_str.trim().parse::<u64>() {
                numbers.push(num); // Add the parsed number to the vector
            }
        }
    }

    Ok(numbers)
}
/// prime_factors:
/// determine the prime factors of a specified number
/// args:
/// num: the number to factor
/// prime_set: a hash set of the first 100,000 (or so) prime numbers
/// primes: the first 100,000 (or so) prime numbers in order from lowest (2) to highest 
/// result:
/// a vector of the prime factors of num.
async fn prime_factors(num: u64, primes: &[u64]) -> Result<Vec<u64>, Box<dyn Error>> {
    let mut factors: Vec<u64> = Vec::new();
    let mut n = num;

    for &p in primes.iter() {
        if p * p > n { // Optimization: Stop checking when p exceeds the square root of the remaining n
            break;
        }

        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }

    if n > 1 {
        // The remaining value is a prime factor itself
        factors.push(n);
    }

    Ok(factors)
}
// Note: This removes the need for prime_set, simplifying the function signature and setup.

/// group_factors
/// with a vector of all of the prime factors of a number as input, group the duplicate
/// factors into a vector of tuples. e.g. for vector [2,2,5,5,5,17] group these as [(2,2),(5,3),(17,1)]
/// args: all_factors - the vector of prime factors
/// result: the vector of tuples as noted above.
async fn group_factors(all_factors:Vec<u64>) -> Result<Vec<(u64,u16)>, Box<dyn Error>> {
    let mut grouped_factors:Vec<(u64,u16)> = Vec::new();
    let mut current_factor:u64 = all_factors[0];
    let mut factor_count:u16 = 1;
    for f in 1..all_factors.len() {
        if all_factors[f] == current_factor {
            factor_count += 1;
        }
        else {
            grouped_factors.push((current_factor,factor_count));
            factor_count = 1;
            current_factor = all_factors[f];
        }
    }
    grouped_factors.push((current_factor,factor_count));
    // for (a, b) in &grouped_factors {
    //     println!("({:?}, {:?})", a, b);
    // }
    Ok(grouped_factors)
}
async fn sigma_2_dirichlet(n: u64, primes: &[u64]) -> Result<u128, Box<dyn Error>> {
    // 1. Get flat list of factors for n
    let vec_result = prime_factors(n, primes).await?;
    
    // Handle the case where n=1 or no factors are found (should not happen if n > 1)
    if vec_result.is_empty() {
        return Ok(1); // sigma_2(1) = 1
    }
    
    // 2. Group factors of n: [(p1, a1), (p2, a2), ...]
    let factor_groups = group_factors(vec_result).await?;

    let mut result = BigUint::one();

    for (p, a) in factor_groups {
        let p_big = p.to_bigint().unwrap().to_biguint().unwrap();
        let a_u32 = a as u32;

        // Exponent in n^2 is e_i = 2 * a_i
        // Exponent for the formula numerator is 2 * (e_i + 1) = 2 * (2*a + 1)
        let numerator_exponent: u32 = 2 * (2 * a_u32 + 1);
        
        // Numerator: p^(2*(2a+1)) - 1
        let numerator = p_big.pow(numerator_exponent) - BigUint::one();
        
        // Denominator: p^2 - 1
        let denominator = p_big.pow(2) - BigUint::one();
        
        // Term = Numerator / Denominator
        let term = numerator / denominator; 
        
        result *= term;
    }

    // Convert the final BigUint result back to u128 (assuming it fits)
    Ok(result.try_into().unwrap_or_else(|_| {
        // Handle overflow if the number is > u128::MAX
        eprintln!("Sigma2 result overflowed u128 for n={}. Truncating.", n);
        u128::MAX
    }))
}
fn sigma_2(num: i64) -> u128 {
    // use u128 to avoid overflow on intermediate squares and sums; use saturating_add
    // to protect against unlikely u128 overflow, and also return the last digit to avoid
    // parsing huge strings back into numeric types.
    let num128 = num as u128;
    let num_2: u128 = num128 * num128;
    let mut sum_of_squares: u128 = 0;
    for d in 1..(num128 + 1) {
        let d_u128 = d as u128;
        if num_2 % d_u128 == 0 {
            sum_of_squares = sum_of_squares.saturating_add(d_u128 * d_u128);
            let flr = num_2 / d_u128;
            //println!("num_2:{num_2}  d:{d}  flr:{flr}");
            if d_u128 != flr {
                sum_of_squares = sum_of_squares.saturating_add(flr * flr);
            }
        }
    }
    sum_of_squares
}

/// async get_pairs_old()
/// Loop through all values (n2) less then the passed number n (see args). Within that loop
/// loop in reverse through all values starting at k2=n*2.25. for all (n2*k2) + (n2+k2)² that
/// match the passed sqrt, insert two rows into the pairs table. One for sequenceStart=n2, increment=k2
/// and one for sequenceStart=k2, increment=n2.
/// args:
///   n:i64         the previously processed value where sequenceStart = increment = n
///   sqrt:i64      √f(n,n), the square root i.e. (n*n) + (n*n)² 
///   pool:&sqlx::PgPool
///                 The sqlx connection for INSERTS
///   sigma2:&String sigma2. For sigma2 values ending in 3 or 9, get_pairs() will stop
///                 searching for f(n,k) pairs that generate the passed sqrt.
///   pairs_table:&String
///                 The table name of the pairs table in the connected database. Passed into
///                 main() via the JSON parameters file.
///   pairs_table_columns:&Vec<&str>
/// results:        Returns no value
/// 
///   
async fn get_pairs_old(n: i64, sqrt: i64, pool: &sqlx::PgPool, sigma2_num: u128, pairs_table: &str, 
    pairs_table_columns: &Vec<&str>,install_dual_pairs: bool) {
// Note: the 2.25 multiplier for the upper value was determined by trial and error. It has not
//       been proved that this number is correct for all n, but has been tested up to n=65505.
// Note2: See comments at the end of the program where there is an SQL query to validate
//        this number.
    let mut upper_k2:i64 = (2.24*n as f32) as i64;
    let lower_k2:i64 = n+1;
    println!("getting pairs...");
    let mut found_count:u128 = 1;  // the row inserted before this subroutine was called
    for n2 in 1..n {
        for k2 in (lower_k2..upper_k2).rev() {
            if sqrt == (n2*k2) + (n2+k2)*(n2+k2) {
                println!("...{}:{} & {}:{}",n2,k2,k2,n2);
                let qry_str = format!(
                    "INSERT INTO {} ({}, {}, {}) VALUES($1,$2,$3) \
                    ON CONFLICT ({},{}) DO NOTHING",
                    pairs_table,
                    pairs_table_columns[0],
                    pairs_table_columns[1],
                    pairs_table_columns[2],
                    pairs_table_columns[0],
                    pairs_table_columns[1]);
                let expect_str = format!("Failed insert for {} table in get_pairs() function.",pairs_table);
                sqlx::query(&qry_str)
                    .bind(sqrt)
                    .bind(n2)
                    .bind(k2)
                    .execute(pool)
                    .await
                    .expect(&expect_str);
                if install_dual_pairs {
                    sqlx::query(&qry_str)
                        .bind(sqrt)
                        .bind(k2)
                        .bind(n2)
                    .execute(pool)
                    .await
                    .expect(&expect_str);
                }
                upper_k2 = k2-1; // no other upper values >= k2 will work with the increasing n2 values
                found_count += 2;
                break;                              
            }
            if sigma2_num == 3 || sigma2_num  == 9 {
                if found_count == sigma2_num {
                    break;
                }
            }
        }
    }
}

/// A helper function to check if a number is a perfect square and return its root.
fn is_perfect_square(n: i128) -> Option<i128> {
    if n < 0 {
        return None;
    }
    // Calculate the integer square root
    let root = (n as f64).sqrt().round() as i128;
    // Check if squaring the integer root returns the original number
    if root * root == n {
        Some(root)
    } else {
        None
    }
}

/// async get_pairs()
/// Solves for k2 using the quadratic formula, eliminating the nested loop.
/// This function is modified to return an error (using the '?') for clean error propagation.
async fn get_pairs(
    n: i64, 
    sqrt: i64, 
    pool: &PgPool, 
    sigma2_num: u128, 
    pairs_table: &str, 
    pairs_table_columns: &Vec<&str>,
    install_dual_pairs: bool
) -> Result<(), Box<dyn Error>> { // Added Result<(), Box<dyn Error>> for safety
    
    println!("getting pairs (optimized)...");
    
    // Convert inputs to i128 for safe calculation
    let sqrt_i128: i128 = sqrt.into();
    let n_i128: i128 = n.into();

    let mut found_count: u128 = 1; // n:n pair already inserted

    // Only iterate up to n, searching for n2
    for n2 in 1..n_i128 {
        
        // 1. Calculate the Discriminant (Value under the square root)
        // Discriminant (Delta) = 5*n2^2 + 4*sqrt
        let n2_i128 = n2.into();
        let discriminant = 5 * n2_i128 * n2_i128 + 4 * sqrt_i128;

        // 2. Check if the discriminant is a perfect square
        if let Some(delta_root) = is_perfect_square(discriminant) {
            
            // 3. Calculate k2 using the positive root of the quadratic formula
            // k2 = (-3*n2 + sqrt(Delta)) / 2
            let numerator = -3 * n2_i128 + delta_root;
            
            // k2 must be positive and divisible by 2 for an integer solution
            if numerator > 0 && numerator % 2 == 0 {
                let k2_i128: i128 = numerator / 2;
                
                // 4. Validate k2 constraints (k2 must be greater than n2)
                // The relationship f(n,k) = f(k,n) means we only need to find one.
                // Since we iterate n2=1 up to n, we are looking for k2 > n2.
                // The current code requires lower_k2 = n+1, meaning the found pairs 
                // must have k2 > n. For simplicity and correctness with the algebraic formula, 
                // we check k2 > n2.
                
                if k2_i128 > n2_i128 {
                    // We found a pair (n2, k2_i128)
                    let k2: i64 = k2_i128.try_into().unwrap_or(i64::MAX);
                    let n2_i64: i64 = n2_i128.try_into().unwrap_or(i64::MAX);

                    println!("...{}:{} & {}:{}", n2_i64, k2, k2, n2_i64);
                    
                    // --- Database Insertion ---
                    // Note: You should wrap this in a transaction and use batching 
                    // (P3 optimization) for better performance. 
                    
                    let qry_str = format!(
                        "INSERT INTO {} ({}, {}, {}) VALUES($1,$2,$3) \
                        ON CONFLICT ({},{}) DO NOTHING",
                        pairs_table,
                        pairs_table_columns[0],
                        pairs_table_columns[1],
                        pairs_table_columns[2],
                        pairs_table_columns[0],
                        pairs_table_columns[1]);
                    
                    // Insert (n2, k2)
                    sqlx::query(&qry_str)
                        .bind(sqrt)
                        .bind(n2_i64)
                        .bind(k2)
                        .execute(pool)
                        .await?; // Use '?' instead of .expect()
                        
                    if install_dual_pairs {
                        // Insert (k2, n2)
                        sqlx::query(&qry_str)
                            .bind(sqrt)
                            .bind(k2)
                            .bind(n2_i64)
                            .execute(pool)
                            .await?; // Use '?' instead of .expect()
                    }
                    
                    found_count += 2;
                    
                    // 5. Early Exit Logic
                    if sigma2_num == 3 || sigma2_num == 9 {
                        if found_count >= sigma2_num {
                            // Stop searching for this specific sqrt if the target count is reached
                            return Ok(());
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}
/*
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

Note for how the muliplier in get_pairs was validated. 
  After creating rows in the pairs table for startSequence/increment equal pairs from
  1-65505, the SQL query below was used to calculate the ratio of the highest sequenceStart
  to the second highest sequenceStart. The highest ratio between those two columns was 
  2.2354262487966627: 

-- having built a table of pairs values calculate highest (max) sequenceStart and next highest
-- sequence start. Then divide the max by next highest. This will give a ratio that should show
-- the muliplier to use in the productoffour get_pairs() function.
		WITH RankedData AS (
			SELECT
				sqrt,
				sequenceStart,
				RANK() OVER (PARTITION BY sqrt ORDER BY sequenceStart DESC) AS rank_within_group
			FROM pairs
		)
		select rd2.sqrt, max_1, max_2, quotient,right(sigma2,1) 
		from 
			(select   RankedData.sqrt,
			MAX(CASE WHEN rank_within_group = 1 THEN sequenceStart END) AS max_1,
			MAX(CASE WHEN rank_within_group = 2 THEN sequenceStart END) AS max_2,
			MAX(CASE WHEN rank_within_group = 1 THEN cast(sequenceStart as numeric) END) 
				/ MAX(CASE WHEN rank_within_group = 2 THEN cast(sequenceStart as numeric) END) AS quotient
			FROM RankedData
			WHERE rank_within_group <= 2
			GROUP BY RankedData.sqrt) as rd2
		join oddonlyresults o 
		on rd2.sqrt = o.sqrt
		group by rd2.sqrt,max_1,max_2,quotient,sigma2
		having quotient notnull 
		ORDER BY quotient desc
		;
*/


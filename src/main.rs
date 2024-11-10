use sqlx::MySqlPool;
use std::error::Error;
use std::env;
use std::fs;
use serde_json;

  
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
/// In general for initial value n and increment (difference) k, √f(n,k) = (n*k + (n+k)²).
/// 
/// Thus √f(3,4) (61 in the above example) can be computed by the
/// formula: (3*4) * (3+4)² = 12 + 7² = 12 + 49 = 61
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
/// parameters and then connect to the mysql database Pof4. The credentials for this connect are
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
#[async_std::main]
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

    let drop_and_create_tables: bool = v["drop_and_create_tables"].as_bool().unwrap();
    let start_of_loop: u64 = v["start"].as_u64().unwrap();
    let end_of_loop: u64 = v["end"].as_u64().unwrap();
    let odd_only_results_table: &str = v["oddOnlyResults_table"].as_str().unwrap();
    let pairs_table: &str = v["pairs_table"].as_str().unwrap();
    let connection_string: &str = v["connection_string"].as_str().unwrap();

    println!("Parameters:");
    println!("\tdrop_and_create_tables: {}\n \
    \tstart_of_loop: {}\n \
    \tend_of_loop: {}\n \
    \tconnection_string: {}",
    drop_and_create_tables,start_of_loop,end_of_loop,connection_string);

    println!("\n \
    \tpairs table name: {}\n \
    \toddOnlyResults table name: {}",
    pairs_table,odd_only_results_table);

    let pool = MySqlPool::connect(connection_string).await?;

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
                    sqrt BIGINT UNSIGNED NOT NULL, \
                    sequenceStart BIGINT UNSIGNED NOT NULL, \
                    increment BIGINT UNSIGNED DEFAULT NULL, \
                    PRIMARY KEY (sqrt,sequenceStart) \
            )",
            pairs_table);
        let expect_str = format!("Failed to create table '{}'",pairs_table);
        sqlx::query(&qry_str)
            .execute(&pool)
            .await
            .expect(&expect_str);

        let qry_str = format!(
            "CREATE TABLE {} ( \
                    sqrt BIGINT UNSIGNED NOT NULL, \
                    sigma2mod100 INT UNSIGNED DEFAULT NULL, \
                    PRIMARY KEY (sqrt) \
            )",
            odd_only_results_table);
        let expect_str = format!("Failed to create table '{}'",odd_only_results_table);
        sqlx::query(&qry_str)
            .execute(&pool)
            .await
            .expect(&expect_str);
    }

    // s:u64 represents the equal start_of_sequence and difference (increment)
    for s in start_of_loop..end_of_loop {
        let sigma2_mod_100:u64 = sigma_2(s);
        let sqrt = (s*s) + (s+s)*(s+s);
        println!("{sqrt} ==> {s}:{s} (sigma_2 mod 100: {sigma2_mod_100})");

        let qry_str = format!(
            "INSERT INTO {} (sqrt, sigma2mod100) VALUES (?,?) \
             ON DUPLICATE KEY UPDATE sigma2mod100 = sigma2mod100",
            odd_only_results_table);
        let expect_str = format!("Insert failed for {}",odd_only_results_table);
        sqlx::query(&qry_str)
            .bind(sqrt)
            .bind(sigma2_mod_100)
            .execute(&pool)
            .await
            .expect(&expect_str);

        let qry_str = format!(
            "INSERT INTO {} (sqrt, sequenceStart, increment) VALUES(?,?,?) \
             ON DUPLICATE KEY UPDATE increment = increment",
            pairs_table);
        let expect_str = format!("Failed insert for {} table",pairs_table);
        sqlx::query(&qry_str)
            .bind(sqrt)
            .bind(s)
            .bind(s)
            .execute(&pool)
            .await
            .expect(&expect_str);

        // find all pairs (sequenceStart "s"/increment "i") 
        // where (s*i) + (s+i)(s+i) generates the current sqrt value.
         
        get_pairs(s, sqrt, &pool, sigma2_mod_100, &pairs_table).await;

        if s%250 == 0 {
            println!("{s}");
        };
    }
    Ok(())
}

/// max value for num as a 64 bit integer is 65500 (or so). After that
/// sigma2%100 will generate incorrect results (overflow)
fn sigma_2(num:u64) -> u64 {
    let num_2 = num * num;
    let mut sum_of_squares:u64 = 0;
    for d in 1..num+1 {
        if num_2 % d == 0 {
            sum_of_squares += (d*d)%100;
            let flr = num_2 / d;
            //println!("num_2:{num_2}  d:{d}  flr:{flr}");
            if d != flr {
                sum_of_squares += (flr*flr)%100;
            }
        }
    }
    sum_of_squares % 100
}

/// async get_pairs()
/// Loop through all values (n2) less then the passed number n (see args). Within that loop
/// loop in reverse through all values starting at k2=n*2.25. for all (n2*k2) + (n2+k2)² that
/// match the passed sqrt, insert two rows into the pairs table. One for sequenceStart=n2, increment=k2
/// and one for sequenceStart=k2, increment=n2.
/// Note: the 2.25 multiplier for the upper value was determined by trial and error. It has not
///       been proved that this number is correct for all n, but has been tested up to n=65505.  
/// args:
///   n:u64         the previously processed value where sequenceStart = increment = n
///   sqrt:u64      √f(n,n), the square root i.e. (n*n) + (n*n)² 
///   pool:&MySqlPool
///                 The sqlx connection for INSERTS
///   sigma2mod10:u64 sigma2mod100 modulo 10. For sigma2mod10 == 3 or 9, get_pairs() will stop
///                 searching for f(n,k) pairs that generate the passed sqrt.
///   pairs_table:&String
///                 The table name of the pairs table in the connected database. Passed into
///                 main() via the JSON parameters file.
/// results:        Returns no value
///   
async fn get_pairs(n: u64, sqrt: u64, pool: &MySqlPool, sigma2mod10: u64, pairs_table: &str) {
    let mut upper_k2:u64 = (2.25*n as f32) as u64;
    let lower_k2:u64 = n+1;
    let mut found_count:u64 = 1;  // the row inserted before this subroutine was called
    for n2 in 1..n {
        for k2 in (lower_k2..upper_k2).rev() {
            if sqrt == (n2*k2) + (n2+k2)*(n2+k2) {
                let qry_str = format!(
                    "INSERT INTO {} (sqrt, sequenceStart, increment) VALUES(?,?,?) \
                    ON DUPLICATE KEY UPDATE increment = increment",
                    pairs_table);
                let expect_str = format!("Failed insert for {} table in get_pairs() function.",pairs_table);
                sqlx::query(&qry_str)
                    .bind(sqrt)
                    .bind(n2)
                    .bind(k2)
                    .execute(pool)
                    .await
                    .expect(&expect_str);
                sqlx::query(&qry_str)
                    .bind(sqrt)
                    .bind(k2)
                    .bind(n2)
                    .execute(pool)
                    .await
                    .expect(&expect_str);
                upper_k2 = k2-1; // no other upper values >= k2 will work with the increasing n2 values
                found_count += 2;
                break;                              
            }

            if sigma2mod10 == 3 || sigma2mod10 == 9 {
                if found_count == sigma2mod10 {
                    break;
                }
            }
        }
    }
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
*/

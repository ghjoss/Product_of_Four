use sqlx::{PgPool};
use std::error::Error;
use std::env::{self};
use std::fs;
use tokio;
use tokio::io::{self,stdin, AsyncBufReadExt};
use std::process;
use serde_json;

//use futures::stream::StreamExt;

  
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
                    sqrt BIGINT NOT NULL, \
                    sequenceStart BIGINT NOT NULL, \
                    increment INT UNSIGNED DEFAULT NULL, \
                    CONSTRAINT pairs_pk PRIMARY KEY (sqrt,sequenceStart) \
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
                    sigma2 VARCHAR(75) DEFAULT NULL, \
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
    for s in start_of_loop..end_of_loop {
        let sigma2_str = sigma_2(s);
        let sqrt = (s*s) + (s+s)*(s+s);
        println!("{sqrt} ==> {s}:{s} (sigma_2 : {sigma2_str})");

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
            .bind(sigma2_str.clone())
            .execute(&pool)
            .await
            .expect(&expect_str);

        let insert_str = format!(
            "INSERT INTO {} (sqrt, sequenceStart, increment) VALUES($1,$2,$3) \
             ON CONFLICT (sqrt, sequenceStart) DO NOTHING",
            pairs_table);
        let expect_str = format!("Failed insert for {} table",pairs_table);
        sqlx::query(&insert_str)
            .bind(sqrt)
            .bind(s)
            .bind(s)
            .execute(&pool)
            .await
            .expect(&expect_str);

        // find all pairs (sequenceStart "s"/increment "i") 
        // where (s*i) + (s+i)(s+i) generates the current sqrt value.
        let sigma2_num = sigma2_str.parse::<u64>().unwrap() % 10;
         
        get_pairs(s, sqrt, &pool, sigma2_num, &pairs_table).await;

        if s%250 == 0 {
            println!("{s}");
        };
    }
    transaction.commit().await?;
    Ok(())
}

//// max value for num as a 64 bit integer is 65500 (or so). After that
/// sigma2%100 will generate incorrect results (overflow)
/// Note: The reason u64 is not returned is that sqlx:: does not support u64
///     variables (9 Nov 20)
fn sigma_2(num:i64) -> String {
    let num64 = num as u64;
    let num_2: u64 = num64 * num64;
    let mut sum_of_squares:u64 = 0;
    for d in 1..(num64 + 1) {
        if num_2 % d == 0 {
            sum_of_squares += d*d;
            let flr = num_2 / d;
            //println!("num_2:{num_2}  d:{d}  flr:{flr}");
            if d != flr {
                sum_of_squares += flr*flr;
            }
        }
    }
    sum_of_squares.to_string()
}

/// async get_pairs()
/// Loop through all values (n2) less then the passed number n (see args). Within that loop
/// loop in reverse through all values starting at k2=n*2.25. for all (n2*k2) + (n2+k2)² that
/// match the passed sqrt, insert two rows into the pairs table. One for sequenceStart=n2, increment=k2
/// and one for sequenceStart=k2, increment=n2.
/// Note: the 2.25 multiplier for the upper value was determined by trial and error. It has not
///       been proved that this number is correct for all n, but has been tested up to n=65505.  
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
/// results:        Returns no value
///   
async fn get_pairs(n: i64, sqrt: i64, pool: &sqlx::PgPool, sigma2_num: u64, pairs_table: &str) {
    let mut upper_k2:i64 = (2.25*n as f32) as i64;
    let lower_k2:i64 = n+1;
    println!("getting pairs...");
    let mut found_count:u64 = 1;  // the row inserted before this subroutine was called
    for n2 in 1..n {
        for k2 in (lower_k2..upper_k2).rev() {
            if sqrt == (n2*k2) + (n2+k2)*(n2+k2) {
                println!("...{}:{} & {}:{}",n2,k2,k2,n2);
                let qry_str = format!(
                    "INSERT INTO {} (sqrt, sequenceStart, increment) VALUES($1,$2,$3) \
                    ON CONFLICT (sqrt, sequenceStart) DO NOTHING",
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
            if sigma2_num == 3 || sigma2_num  == 9 {
                if found_count == sigma2_num {
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

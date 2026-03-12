use axum_etl::model;
use axum_etl::etl;

use anyhow::Result;


fn main()->Result<()>{

    let transactions = etl::read_transactions_from_csv("artifacts/data.csv")?;
    println!("Loaded {} transactions", transactions.len());

    let transactions = etl::normalise_users(transactions);
    for row in transactions.iter(){
        print!("{:#?}",row);
    }
    Ok(())
}
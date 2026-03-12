use std::{path::Path};
use anyhow::{Ok, Result};
use csv::{self,ReaderBuilder};
use std::fs::File;

use crate::{model::{RawTransaction,Transaction}, pipeline::{self, MapTransform, Pipeline}};

pub fn read_transactions_from_csv<P:AsRef<Path>>(path:P)->Result<Vec<Transaction>>{
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut transactions = Vec::new();

    for result in reader.deserialize::<RawTransaction>(){
        let raw_transaction_row = result?;
        let transaction_row = Transaction::from(raw_transaction_row);
        transactions.push(transaction_row)
    }
    Ok(transactions)
}

pub fn normalise_users(transactions:Vec<Transaction>)->Vec<Transaction>{
    let transform = MapTransform::new(|mut transactions:Transaction|{
        transactions.user = transactions.user.to_lowercase();
        transactions
    });
    let pipeline = Pipeline::new(transform);
    pipeline.run(transactions.into_iter()).collect()
}
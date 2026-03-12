use std::os::linux::raw;

use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,Eq,Hash)]
pub enum Category{
    Food,
    Books,
    Other(String)
}


impl From<&str> for Category{
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str(){
            "food"=>Category::Food,
            "books"=>Category::Books,
            other=>Category::Other(other.to_string())
        }
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Transaction{
    pub id:String,
    pub user:String,
    pub category:String,
    pub amount:f64,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct RawTransaction {
    pub id:String,
    pub user:String,
    pub category:String,
    pub amount:f64,
}


impl From<RawTransaction> for Transaction{
    fn from(raw_transaction: RawTransaction) -> Self {
        Transaction { 
            id: raw_transaction.id,
            user: raw_transaction.user,
            category: raw_transaction.category,
            amount: raw_transaction.amount
        }
    }
}



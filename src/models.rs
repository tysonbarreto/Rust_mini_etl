use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawUser {
    pub name: String,
    pub age: String,
    pub country: String
}

#[derive(Debug)]
pub struct CleanUser{
    pub name: String,
    pub age: u8,
    pub country: String
}


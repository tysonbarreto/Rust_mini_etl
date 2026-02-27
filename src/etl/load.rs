use crate::errors::AppResult;
use crate::models::CleanUser;
use crate::etl::traits::Loader;
use rusqlite::{Connection, params};

pub struct SqlLiteLoader {
    conn:Connection
}

impl SqlLiteLoader {
    ///Create an instrance of new SqlLiteLoader
    pub fn new(path:&str)->AppResult<Self>{
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS users(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            age INTEGER NOT NULL,
            country TEXT NOT NULL
        );")?;
        Ok(Self{conn:conn})
    }
}

impl Loader<CleanUser> for SqlLiteLoader{
    fn load(&mut self, item: &CleanUser)->AppResult<()> {
        self.conn.execute("INSERT INTO user(name,age,country) VALUES (?1,?2,?3)", params![item.name, item.age, item.country])?;
        Ok(())
    }
}
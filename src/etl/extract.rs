use crate::errors::AppResult;
use crate::models::RawUser;
use crate::etl::traits::Extractor;

use std::fs::File;
use std::path::Path;

use csv;

pub struct CsvExtractor{
    reader: csv::Reader<File>
}

impl CsvExtractor {
    pub fn from_path<P>(path:P)->AppResult<Self>
    where 
        P:AsRef<Path>
    {
        let file = File::open(path)?;
        let reader = csv::Reader::from_reader(file);
        Ok(Self{reader:reader})
    }
}   

impl Extractor<RawUser> for CsvExtractor{
    fn extract<'a>(&'a mut self)-> Box<dyn Iterator<Item = AppResult<RawUser>> + 'a> {
        let iter = self
                                    .reader
                                    .deserialize::<RawUser>()
                                    .map(|res|{
                                        match res {
                                            Ok(record)=>Ok(record),
                                            Err(error)=>Err(error.into())
                                        }
                                    })
                                    ;
        Box::new(iter)            
    }
}

//>>>>>>> TESTS <<<<<<<<

#[cfg(test)]
mod tests{
    use std::*;
    use std::fs;

    use crate::etl::extract::CsvExtractor;
    use crate::etl::traits::Extractor;

    #[test]
    fn extracts_rows_from_csv(){
        let csv_data = "name,age,country\nAlice,30,uk";
        fs::write("test_users.csv", csv_data).unwrap();

        let mut extractor = CsvExtractor::from_path("test_users.csv").unwrap();
        let mut iter = extractor.extract();

        let first = iter.next().unwrap().unwrap();

        assert_eq!(first.name,"Alice");
        assert_eq!(first.country,"uk");

        fs::remove_file("test_users.csv").unwrap();
    }
}

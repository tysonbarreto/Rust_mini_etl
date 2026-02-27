pub mod traits;
pub mod extract;
pub mod transform;
pub mod load;

use crate::models::{RawUser,CleanUser};
use crate::errors::AppResult;
use traits::{Extractor,Transformer,Loader};

pub struct Pipeline<E,T,L>
    where
        E:Extractor<RawUser>,
        T:Transformer<RawUser,CleanUser>,
        L:Loader<CleanUser>
    {
        extractor: E,
        transformer: T,
        loader: L
    }
impl <E,T,L> Pipeline<E,T,L>
    where
        E:Extractor<RawUser>,
        T:Transformer<RawUser,CleanUser>,
        L:Loader<CleanUser>
{
    pub fn new(extractor:E, transformer:T, loader:L)->Self {
        Self { extractor, transformer, loader }
    }

    pub fn run(&mut self)->AppResult<()>{
        let iter = self.extractor.extract();

        let mut total = 0;
        let mut loaded = 0;

        for item in iter{
            total +=1 ;
            let raw = match item {
                Ok(v)=>v,
                Err(err)=>{
                    println!("Error extracting records: {}",err);
                    continue;
                }
            };

            if let Some(clean) = self.transformer.transform(raw){
                self.loader.load(&clean)?;
                loaded +=1 ;

            };

        }
    println!("ETL finished. Total rows: {0}, Loaded rows: {1}", total, loaded);    
    Ok(())
    }

}





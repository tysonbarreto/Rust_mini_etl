
use crate::errors::AppResult;


///Extractor produces an iterator of items T wrapped in AppResult
pub trait Extractor<T>{
    /// The iterator returned, must have its lifetime tied with the lifetime of one implementing the trait
    /// Rust by default ties the lifetime of &self or &mut self to the output 
    fn extract<'a>(&'a mut self)-> Box<dyn Iterator<Item = AppResult<T>> + 'a>;
}

/// Transfoms the imputs to optional outputs
pub trait Transformer<I,O>{
    /// Rust by default ties the lifetime of &self or &mut self to the output 
    fn transform(&self, input:I)->Option<O>;
}

///Loader consumes the items of type T and persists them somewhere, returning ()
pub trait Loader<T>{
    /// Rust by default ties the lifetime of &self or &mut self to the output 
    fn load(&mut self, item: &T)->AppResult<()>;
}



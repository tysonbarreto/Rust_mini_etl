use std::fmt::Debug;
use std::fmt;

pub trait Transform<I,O>{
    fn transform(&self,input:I)->O;
}

#[derive(Debug)]
pub struct MapTransform<F>{
    f:F
}

impl <F> MapTransform<F>{
    pub fn new(f:F)->Self{
        Self { f:f }
    }
}


impl<F> fmt::Display for MapTransform<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MapTransform<{}>", std::any::type_name::<F>())
    }
}


impl <F,I,O> Transform<I,O> for MapTransform<F>
where
    F:Fn(I)->O
{
    fn transform(&self,input:I)->O {
        (self.f)(input) //this gives the flexibility to define the any built in method inside the closure
    }
}   


//The pipeline takes in an iterator and retuns another iterator

#[derive(Debug)]
pub struct Pipeline<T>{
    transform:T
}


impl<T> fmt::Display for Pipeline<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pipeline<{}>", std::any::type_name::<T>())
    }
}


impl <T> Pipeline<T>{
    pub fn new(transform:T)->Self{
        Self { transform }
    }
    //the lifetime by default is linked with the lifetime pipeline
    pub fn run<'a,I,It,O>(&'a self, iter:It)->impl Iterator<Item = O>+'a
    where 
        It:IntoIterator<Item = I>+'a,
        I:'a,
        O:'a,
        T:Transform<I,O>+'a,
    {//we will move the each value of the raw transactions model into the new transaction model
        iter.into_iter().map(move |item| {//this item here is essentially going to be the Transaction model
            self.transform.transform(item)})
    }
}
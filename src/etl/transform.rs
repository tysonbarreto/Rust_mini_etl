use crate::models::{RawUser, CleanUser};
use crate::etl::traits::Transformer;

pub struct UserTransformer;

impl UserTransformer{
    pub fn new()->Self{
        Self
    }    
}

impl Transformer<RawUser, CleanUser> for UserTransformer {
    fn transform(&self, input:RawUser)->Option<CleanUser> {
     //trim and validate name
     let name = input.name.trim().to_string();
     if name.is_empty(){
        println!("Skipping user with empty name: {:?}",input);
        return None;
     }
     //parse age
     let age:u8 = match input.age.trim().parse(){
        Ok(a) if a>0 && a<120 => a,
        _ => {
            println!("Skipping user with invalid age: {:?}",input);
            return None;
        }
     };

     //normalise country to uppercase
     let country = input.country.trim().to_uppercase();
     if country.is_empty(){
        println!("Skipping user with empty country: {:?}",input)
     }
     Some(CleanUser { name: name, age:age, country:country })   
    }
}

//>>>>>>>TESTS<<<<<<<<<<<


#[cfg(test)]
mod tests{
    use super::*;
    use crate::models::{RawUser};

    fn raw(name:&str, age:&str, country:&str)->RawUser{
        RawUser { name: name.to_string(), age: age.to_string(), country: country.to_string() }
    }

    #[test]
    fn transform_valid_user(){
        let t = UserTransformer;
        let input = raw(" Alice","30"," uk ");
        let out = t.transform(input).unwrap();
        assert_eq!(out.name, "Alice");
        assert_eq!(out.age, 30);
        assert_eq!(out.country, "UK");
    }

    #[test]
    fn rejects_ivalid_age(){
        let t = UserTransformer;
        let input = raw("Bob", "99", "us");
        assert!(t.transform(input).is_none());
    }

}
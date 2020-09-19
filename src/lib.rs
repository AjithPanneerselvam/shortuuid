use std::string::String;
use std::{str, u128};
use uuid::Uuid;

pub static DEFAULT_BASE_62: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

#[derive(Debug)]
pub struct ShortUuid {
    short_uuid: String,
    uuid: Uuid,
}

#[allow(dead_code, unused_variables)]
fn shorten(uuid: &Uuid, alphabets: Option<&str>) -> Result<ShortUuid, ()> {
    if *uuid == Uuid::nil() {
        ()
    }

    let uuid_str: String = uuid.to_string().split('-').collect();
    let uuid_base16 = u128::from_str_radix(&uuid_str, 16);
    let mut decimal: u128;

    match uuid_base16 {
        Ok(v) => decimal = v,
        Err(e) => return Err(()),
    }

    let alpha = alphabets.unwrap_or(DEFAULT_BASE_62);
    let radix = alpha.len() as u128;

    let mut result = Vec::<u8>::new();

    while decimal != 0 {
        result.push(alpha.as_bytes()[(decimal % radix) as usize]);
        decimal = decimal / radix;
    }

    Ok(ShortUuid {
        short_uuid: str::from_utf8(&result).unwrap().to_string(),
        uuid: *uuid,
    })
}

#[allow(dead_code, unused_variables)]
fn expand(short_uuid: String, alphabets: Option<&str>) -> Result<Uuid, ()> {
    if short_uuid.is_empty() {
        return Err(())
    }

    let alpha = alphabets.unwrap_or(DEFAULT_BASE_62); 

    let mut decimal = 0; 
    let radix = alpha.len();

    for (i, v) in short_uuid.chars().enumerate() {
        decimal += short_uuid.find(v).unwrap() * radix.pow(i as u32);  
    }

    Ok(Uuid::new_v4())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shorten() {
        let test_uuid = Uuid::new_v4();
        let result = shorten(&test_uuid, None);
        println!("result {:?}", result);
    }

    #[test] 
    fn test_expand() {
    }
}

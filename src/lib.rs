#[macro_use]
pub extern crate validators;
pub extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate base64_stream;
extern crate mime_guess;
extern crate percent_encoding;
extern crate regex;

pub mod escaping;
pub mod value;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

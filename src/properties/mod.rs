use std::fmt::{self, Formatter};

mod begin;
mod end;
mod formatted_name;
mod version;
mod source;
mod name;
mod nickname;
mod photo;
mod birthday;
mod anniversary;
mod gender;

pub use self::begin::Begin;
pub use self::end::End;
pub use self::version::Version;
pub use self::formatted_name::FormattedName;
pub use self::source::Source;
pub use self::name::Name;
pub use self::nickname::NickName;
pub use self::photo::Photo;
pub use self::birthday::Birthday;
pub use self::anniversary::Anniversary;
pub use self::gender::Gender;

pub trait Property {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>;
}

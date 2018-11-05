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
mod address;
mod telephone;
mod email;
mod impp;
mod language;
mod time_zone;
mod geo;
mod title;
mod role;
mod logo;
mod organization;
mod member;
mod relationship;

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
pub use self::address::Address;
pub use self::telephone::Telephone;
pub use self::email::Email;
pub use self::impp::IMPP;
pub use self::language::Language;
pub use self::time_zone::TimeZone;
pub use self::geo::Geo;
pub use self::title::Title;
pub use self::role::Role;
pub use self::logo::Logo;
pub use self::organization::Organization;
pub use self::member::Member;
pub use self::relationship::Relationship;

pub trait Property {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>;
}
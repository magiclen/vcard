use std::fmt::{self, Formatter};

mod address;
mod anniversary;
mod begin;
mod birthday;
mod calendar_address_uri;
mod calendar_uri;
mod category;
mod client_property_id_map;
mod email;
mod end;
mod fburl;
mod formatted_name;
mod gender;
mod geo;
mod impp;
mod key;
mod language;
mod logo;
mod member;
mod name;
mod nickname;
mod note;
mod organization;
mod photo;
mod product_id;
mod relationship;
mod revision;
mod role;
mod sound;
mod source;
mod telephone;
mod time_zone;
mod title;
mod uid;
mod url;
mod version;
mod x_property;

pub use self::{
    address::Address, anniversary::Anniversary, begin::Begin, birthday::Birthday,
    calendar_address_uri::CalendarAddressURI, calendar_uri::CalendarURI, category::Category,
    client_property_id_map::ClientPropertyIDMap, email::Email, end::End, fburl::FBURL,
    formatted_name::FormattedName, gender::Gender, geo::Geo, impp::IMPP, key::Key,
    language::Language, logo::Logo, member::Member, name::Name, nickname::NickName, note::Note,
    organization::Organization, photo::Photo, product_id::ProductID, relationship::Relationship,
    revision::Revision, role::Role, sound::Sound, source::Source, telephone::Telephone,
    time_zone::TimeZone, title::Title, uid::UID, url::URL, version::Version, x_property::XProperty,
};

pub trait Property {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>;
}

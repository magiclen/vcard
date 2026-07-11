/*!
# vCard

A pure Rust implementation of vCard 4.0 for generating and parsing vCards, based on [RFC 6350](https://www.rfc-editor.org/rfc/rfc6350) and [RFC 9554](https://www.rfc-editor.org/rfc/rfc9554).

## Generating a vCard

Create a [`VCard`] and fill its public fields, then serialize it with `to_string` or [`VCard::save_to_file`].

```rust
use vcard::{
    VCard,
    values::{NameValue, TelValue},
    Name, Tel, TypeValue,
};

let mut vcard = VCard::new("David Wang");

let mut name = Name::new(NameValue {
    family_names: vec![String::from("Wang")],
    given_names: vec![String::from("David")],
    ..NameValue::default()
});

vcard.names.push(name);

let mut telephone = Tel::new(TelValue::Text(String::from("+886-912-345-678")));

telephone.parameters.types.push(TypeValue::Cell);

vcard.telephones.push(telephone);

assert_eq!(
    "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:David Wang\r\nN:Wang;David;;;\r\nTEL;TYPE=cell:+886-912-345-678\r\nEND:VCARD\r\n",
    vcard.to_string()
);
```

## Parsing vCards

Parse a single vCard with `str::parse` or several of them with [`VCard::parse_multiple`].

```rust
use vcard::VCard;

let vcard: VCard = "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Magic Len\r\nEND:VCARD\r\n".parse().unwrap();

assert_eq!("Magic Len", vcard.formatted_names[0].value);
```
*/

mod error;
mod fold;
mod parameters;
mod parse;
mod property;
mod syntax;
pub mod values;
mod vcard;

pub use base64;
pub use chrono;
pub use chrono_tz;
pub use error::{InvalidValueError, ParseError, ParseErrorKind, ValidationError};
pub use fold::FoldingWriter;
pub use language_tags;
pub use mime;
pub use mime_guess;
pub use parameters::{
    AnyParameter, Calscale, Parameters, Phonetic, Pid, Pref, PropId, Script, TypeValue, TzParam,
};
pub use property::{
    Address, Anniversary, Birthday, CalendarAddressUri, CalendarUri, Categories, ClientPidMap,
    Created, Email, ExtensionProperty, Fburl, FormattedName, Gender, Geo, GramGender, GroupName,
    Impp, Key, Kind, Lang, Language, Logo, Member, Name, Nickname, Note, Org, Photo, ProdId,
    Pronouns, Property, PropertyValue, Related, Rev, Role, SocialProfile, Sound, Source, Tel,
    TimeZone, Title, Uid, Url, Xml,
};
pub use url;
pub use validators;
pub use vcard::VCard;

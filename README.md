vCard
====================

[![CI](https://github.com/magiclen/vcard/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/vcard/actions/workflows/ci.yml)

A pure Rust implementation of vCard 4.0 for generating and parsing vCards, based on [RFC 6350](https://www.rfc-editor.org/rfc/rfc6350) and [RFC 9554](https://www.rfc-editor.org/rfc/rfc9554).

## Features

* Generates vCard 4.0 text with correct line folding (75 octets) and value escaping.
* Parses vCard 4.0 text back into typed structures, including folded lines and extension properties.
* Supports all RFC 6350 properties and parameters, including KIND, XML and the group construct.
* Supports the RFC 9554 extensions: the CREATED, GRAMGENDER, LANGUAGE, PRONOUNS and SOCIALPROFILE properties, the new parameters like AUTHOR, DERIVED, PHONETIC and SERVICE-TYPE, the extended N and ADR components, and the RFC 6868 caret encoding for parameter values.
* Allows applications to supply external calendar handlers for BDAY and ANNIVERSARY without adding calendar dependencies.

## Generating a vCard

```rust
use vcard::{
    Tel, TypeValue, VCard,
    values::TelValue,
};

let mut vcard = VCard::new("David Wang");

let mut telephone = Tel::new(TelValue::Uri("tel:+886-912-345-678".parse().unwrap()));
telephone.parameters.types.push(TypeValue::Cell);
vcard.telephones.push(telephone);

println!("{vcard}");

/*
BEGIN:VCARD
VERSION:4.0
FN:David Wang
TEL;VALUE=uri;TYPE=cell:tel:+886-912-345-678
END:VCARD
*/
```

Every property is stored in a `Vec` field, and an empty list means that the property is absent.
A property limited to one logical instance can have several representations when they all share the same ALTID.
Every property carries its parameters in a shared `Parameters` struct and can have a group name.
Use `VCard::validate` to check required properties, ALTID cardinality, the group kind for MEMBER, and the service type for text SOCIALPROFILE values.
It also checks phonetic representations, GRAMGENDER language conflicts, LANGUAGE and USERNAME usage, and non-empty AUTHOR-NAME values.
PID identifiers must be positive, their sources must have CLIENTPIDMAP entries, and properties limited to one logical instance cannot have PID parameters.
CLIENTPIDMAP cannot have PID parameters either.
TYPE parameters are checked against the RFC 6350 property restrictions.
CALSCALE on BDAY and ANNIVERSARY requires a date or date-time value, and SORT-AS cannot have more entries than the N or ORG value has serialized components.
Known property names belong in their dedicated fields, and known parameters belong in the named `Parameters` fields.
Typed values write their own VALUE parameters, so only extension properties can keep a VALUE parameter in `Parameters::any`.
Text values and parameters cannot contain ASCII control characters other than tabs and line breaks, and line breaks are escaped when written.
The raw values of extension properties cannot contain ASCII control characters other than tabs.

MEDIATYPE values keep literal carets, including `^n` and `^'`, without RFC 6868 decoding.
Ordinary media types and token MIME parameters such as `text/plain;charset=utf-8` are supported.
MIME parameters containing double quotes, such as `text/plain;name="a,b"`, have ambiguous vCard quoting and are rejected by `VCard::validate`.
Call `validate` before writing a card that you build through its public fields.

Use `VCard::save_to_file` to write the vCard to a file, and `vcard::values::Uri::from_file` to embed a photo, a logo or a sound as a base64 data URI.

## Migrating from 0.5 to 0.6

The former `Option` property fields are now `Vec` fields so that ALTID representations can be kept together.
Replace assignments such as `vcard.birthday = Some(birthday)` with `vcard.birthdays.push(birthday)`.

| Property | Former field | Current field |
| --- | --- | --- |
| KIND | `kind` | `kinds` |
| BDAY | `birthday` | `birthdays` |
| ANNIVERSARY | `anniversary` | `anniversaries` |
| GENDER | `gender` | `genders` |
| PRODID | `product_id` | `product_ids` |
| REV | `revision` | `revisions` |
| UID | `uid` | `uids` |
| CREATED | `created` | `created` |
| LANGUAGE | `language` | `languages` |

`vcard::values::Date::from_year` now returns a `Result` like the other `Date` constructors.
Every `Date` constructor rejects a year outside `0..=9999`.

`VCard` now has a `version_parameters: Parameters` field that preserves VERSION parameters.
Add this field or use `..VCard::default()` when constructing a complete struct literal.
The VERSION value remains fixed at `4.0`, and an explicit VALUE parameter on it must be `text` and is not kept, like the default VALUE of other properties.

`Time::with_zone` now returns `Result<Time, InvalidValueError>`.
Handle its result with `?` or `unwrap()`; a truncated time without an hour cannot have a zone.
Parsing also rejects these combinations, following RFC 6350 erratum 3484.

`Uri::as_url` now returns `Option<&url::Url>`, and `Uri::into_url` returns `Result<url::Url, InvalidValueError>`.
URIs outside the WHATWG URL syntax, such as `foo://[v1.example]/`, are validated against RFC 3986 and kept as written.
Existing URL parsing and normalization remain available, including IDNA and percent encoding.
Handle the optional or fallible URL conversion when your application needs a `url::Url`.

The public `base64` re-export now uses version 0.23.

## Parsing vCards

```rust
use vcard::VCard;

let vcard: VCard = "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Magic Len\r\nEND:VCARD\r\n".parse().unwrap();

assert_eq!("Magic Len", vcard.formatted_names[0].value);
```

Use `VCard::parse_multiple` to read a `.vcf` file that contains several vCards.
Only version 4.0 is supported, and unknown properties are kept in the `extensions` field with their raw values and typed parameters.
The only properties that are dropped are BDAY and ANNIVERSARY with an unknown CALSCALE, which RFC 6350 requires to be ignored, or with conflicting CALSCALE values.
Known properties reject VALUE declarations that do not match their supported value types.
The RFC 9554 occurrence limits for GRAMGENDER, PRONOUNS and SOCIALPROFILE parameters are checked while parsing.
Other repeated single-value parameters keep their last value, while TYPE, PID and SORT-AS collect every value.
Content lines cannot contain ASCII control characters other than tabs.
URI property values accept bare commas and the `\\`, `\,` and `\;` escapes, which RFC 6350 errata 3845 and 3846 apply to URI values.
URI output still uses bare commas, escapes only backslashes, and keeps percent-encoded bytes as encoded.

## Calendar extensions

The `Date` constructors and `FromStr` use the Gregorian calendar, except `Date::from_calendar_parts`.
Other calendars are supplied by the application through the `Calendar` and `CalendarRegistry` traits; this crate does not provide any non-Gregorian calendar implementation or global registration.
Handler names must be fixed lowercase tokens other than `gregorian`.
Private calendar extensions should use an agreed `x-` name.
The registry receives lowercase names, and a returned handler must have the same name as the requested calendar.

Use `VCard::parse_with_calendars` or `VCard::parse_multiple_with_calendars` to read dates with an external registry.
The handler validates the calendar rules and can inspect property parameters, including private parameters used to distinguish dates such as leap months.
Date values, CALSCALE and the other parameters are kept together without conversion to Gregorian dates.
BDAY and ANNIVERSARY properties with an unknown or conflicting CALSCALE are ignored before parsing their values or checking their cardinality, following RFC 6350 section 5.8.
The default parsing methods therefore ignore all non-Gregorian BDAY and ANNIVERSARY properties.

Use `DateParts` and `Date::from_calendar_parts` to build a date for an external calendar:

```rust
use vcard::{Birthday, Calendar, InvalidValueError, Parameters, values::{Date, DateParts}};

fn make_calendar_birthday(
    parts: DateParts,
    mut parameters: Parameters,
    calendar: &dyn Calendar,
) -> Result<Birthday, InvalidValueError> {
    parameters.calscale = Some(calendar.name().parse()?);

    let date = Date::from_calendar_parts(parts, &parameters, calendar)?;
    let mut birthday = Birthday::new(date.into());
    birthday.parameters = parameters;

    Ok(birthday)
}
```

The crate checks the RFC date shapes and numeric bounds before calling the handler: years remain within `0..=9999`, months within `1..=12`, and days within `1..=31`.
A thirteenth month or leap-month marker cannot be written directly as an RFC 6350 DATE; it requires a separately agreed extension representation.
`Date` keeps the calendar name as a `&'static str` and stays `Copy`.
`Date::calendar_scale` returns that name, or `gregorian` for Gregorian dates.
Date equality and hashing include the calendar name and numeric parts; other calendar metadata remains in the property's parameters.

`VCard::validate` checks that each date matches its CALSCALE parameter.
Use `VCard::validate_with_calendars` to also check the current parameters with the external handlers, especially after modifying parameters that affect the calendar rules.
Missing handlers, mismatched names and rejected calendar dates report `ValidationError::InvalidParameter` for CALSCALE.
Serialization writes the usual RFC 6350 date format with the CALSCALE parameter, so reading a custom date back needs the same calendar registry.

## Crates.io

https://crates.io/crates/vcard

## Documentation

https://docs.rs/vcard

## License

[MIT](LICENSE)

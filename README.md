vCard
====================

[![CI](https://github.com/magiclen/vcard/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/vcard/actions/workflows/ci.yml)

A pure Rust implementation of vCard 4.0 for generating and parsing vCards, based on [RFC 6350](https://www.rfc-editor.org/rfc/rfc6350) and [RFC 9554](https://www.rfc-editor.org/rfc/rfc9554).

## Features

* Generates vCard 4.0 text with correct line folding (75 octets) and value escaping.
* Parses vCard 4.0 text back into typed structures, including folded lines and extension properties.
* Supports all RFC 6350 properties and parameters, including KIND, XML and the group construct.
* Supports the RFC 9554 extensions: the CREATED, GRAMGENDER, LANGUAGE, PRONOUNS and SOCIALPROFILE properties, the new parameters like AUTHOR, DERIVED, PHONETIC and SERVICE-TYPE, the extended N and ADR components, and the RFC 6868 caret encoding for parameter values.

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

A property that can appear several times is a `Vec` field, and a property that can appear at most once is an `Option` field.
Every property carries its parameters in a shared `Parameters` struct and can have a group name.

Use `VCard::save_to_file` to write the vCard to a file, and `vcard::values::Uri::from_file` to embed a photo, a logo or a sound as a base64 data URI.

## Parsing vCards

```rust
use vcard::VCard;

let vcard: VCard = "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Magic Len\r\nEND:VCARD\r\n".parse().unwrap();

assert_eq!("Magic Len", vcard.formatted_names[0].value);
```

Use `VCard::parse_multiple` to read a `.vcf` file that contains several vCards.
Only version 4.0 is supported, and unknown properties are kept in the `extensions` field so that nothing is lost.

## Crates.io

https://crates.io/crates/vcard

## Documentation

https://docs.rs/vcard

## License

[MIT](LICENSE)

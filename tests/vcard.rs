extern crate vcard;

use vcard::VCard;

#[test]
fn basic_v4p0() {
    let mut vcard = VCard::from_formatted_name_str("Magic Len").unwrap();

    vcard.revision = None;

    assert_eq!("BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Magic Len\r\nEND:VCARD\r\n", vcard.to_string());
}
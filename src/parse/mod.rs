//! The vCard text parser.

mod content_line;
mod lines;

use std::str::FromStr;

use content_line::{RawContentLine, parse_content_line};
use lines::LogicalLines;

use crate::{
    error::{ParseError, ParseErrorKind},
    parameters::{AnyParameter, Parameters},
    property::{ExtensionProperty, GroupName, Property, PropertyValue},
    values::Token,
    vcard::{VCard, for_each_property},
};

impl FromStr for VCard {
    type Err = ParseError;

    /// Parses text that contains exactly one vCard.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = LogicalLines::new(s);

        let Some(vcard) = parse_card(&mut lines)? else {
            return Err(ParseError {
                line: lines.current_line_number(),
                kind: ParseErrorKind::ExpectedBegin,
            });
        };

        if let Some((number, _)) = lines.next() {
            return Err(ParseError {
                line: number, kind: ParseErrorKind::TrailingData
            });
        }

        Ok(vcard)
    }
}

impl VCard {
    /// Parses text that contains any number of vCards, e.g. the content of a `.vcf` file with several contacts.
    pub fn parse_multiple(s: &str) -> Result<Vec<Self>, ParseError> {
        let mut lines = LogicalLines::new(s);

        let mut vcards = Vec::new();

        while let Some(vcard) = parse_card(&mut lines)? {
            vcards.push(vcard);
        }

        Ok(vcards)
    }
}

/// Parses one vCard from the line iterator, returning `None` when no lines are left.
fn parse_card(lines: &mut LogicalLines) -> Result<Option<VCard>, ParseError> {
    // The first line must be BEGIN:VCARD without a group or parameters.
    let Some((number, line)) = lines.next() else {
        return Ok(None);
    };

    if !line.eq_ignore_ascii_case("BEGIN:VCARD") {
        return Err(ParseError {
            line: number, kind: ParseErrorKind::ExpectedBegin
        });
    }

    // The VERSION property must come right after BEGIN, as RFC 6350 section 6.7.9 requires.
    let Some((number, line)) = lines.next() else {
        return Err(ParseError {
            line: lines.current_line_number(),
            kind: ParseErrorKind::ExpectedVersion,
        });
    };

    let content = parse_content_line(&line).map_err(|kind| ParseError {
        line: number,
        kind,
    })?;

    if content.group.is_some() || !content.name.eq_ignore_ascii_case("VERSION") {
        return Err(ParseError {
            line: number, kind: ParseErrorKind::ExpectedVersion
        });
    }

    if content.value != "4.0" {
        return Err(ParseError {
            line: number,
            kind: ParseErrorKind::UnsupportedVersion(content.value.to_string()),
        });
    }

    let mut vcard = VCard::default();

    loop {
        let Some((number, line)) = lines.next() else {
            return Err(ParseError {
                line: lines.current_line_number(),
                kind: ParseErrorKind::MissingEnd,
            });
        };

        let content = parse_content_line(&line).map_err(|kind| ParseError {
            line: number,
            kind,
        })?;

        if content.name.eq_ignore_ascii_case("END") {
            if content.group.is_some()
                || !content.params.is_empty()
                || !content.value.eq_ignore_ascii_case("VCARD")
            {
                return Err(ParseError {
                    line: number, kind: ParseErrorKind::InvalidLine
                });
            }

            if vcard.formatted_names.is_empty() {
                return Err(ParseError {
                    line: number,
                    kind: ParseErrorKind::MissingFormattedName,
                });
            }

            return Ok(Some(vcard));
        }

        if content.name.eq_ignore_ascii_case("BEGIN") {
            return Err(ParseError {
                line: number, kind: ParseErrorKind::InvalidLine
            });
        }

        if content.name.eq_ignore_ascii_case("VERSION") {
            return Err(ParseError {
                line: number,
                kind: ParseErrorKind::DuplicateProperty(String::from("VERSION")),
            });
        }

        add_property(&mut vcard, number, content)?;
    }
}

/// Parses one property line and stores it into the right field of the vCard.
fn add_property(
    vcard: &mut VCard,
    number: usize,
    content: RawContentLine,
) -> Result<(), ParseError> {
    let RawContentLine {
        group,
        name,
        params,
        value,
    } = content;

    // The group was already validated as a token by the content line splitter.
    let group =
        match group {
            Some(group) => Some(GroupName::from_str(group).map_err(|_| ParseError {
                line: number,
                kind: ParseErrorKind::InvalidGroupName,
            })?),
            None => None,
        };

    let (parameters, value_type) = Parameters::parse(params).map_err(|kind| ParseError {
        line: number,
        kind,
    })?;

    let value_type = value_type.as_deref();

    let upper_name = name.to_ascii_uppercase();

    // Parses the raw value, where the target type is inferred from the field the result goes into.
    macro_rules! parse_value {
        ($property_name:literal) => {
            match PropertyValue::parse_value(value, value_type) {
                Ok(value) => value,
                Err(_) => {
                    return Err(ParseError {
                        line: number,
                        kind: ParseErrorKind::InvalidValue {
                            property: String::from($property_name),
                        },
                    });
                },
            }
        };
    }

    macro_rules! dispatch_arm {
        (many, $field:ident, $property_name:literal) => {
            vcard.$field.push(Property {
                group,
                parameters,
                value: parse_value!($property_name),
            })
        };
        (one, $field:ident, $property_name:literal) => {
            if vcard.$field.is_some() {
                return Err(ParseError {
                    line: number,
                    kind: ParseErrorKind::DuplicateProperty(String::from($property_name)),
                });
            } else {
                vcard.$field = Some(Property {
                    group,
                    parameters,
                    value: parse_value!($property_name),
                })
            }
        };
    }

    macro_rules! dispatch {
        ($(($field:ident, $property_name:literal, $card:tt)),* $(,)?) => {
            match upper_name.as_str() {
                $($property_name => dispatch_arm!($card, $field, $property_name),)*
                _ => {
                    let mut parameters = parameters;

                    // The VALUE parameter is kept as a generic parameter so that unknown properties round-trip without loss.
                    if let Some(value_type) = value_type {
                        parameters.any.push(AnyParameter {
                            name:   Token::from_str("VALUE").unwrap(),
                            values: vec![value_type.to_string()],
                        });
                    }

                    // The name was already validated as a token by the content line splitter.
                    vcard.extensions.push(ExtensionProperty {
                        group,
                        name: Token::from_str(name).unwrap(),
                        parameters,
                        value: value.to_string(),
                    });
                },
            }
        };
    }

    for_each_property!(dispatch);

    Ok(())
}

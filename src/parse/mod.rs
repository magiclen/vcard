//! The vCard text parser.

mod content_line;
mod lines;

use std::str::FromStr;

use content_line::{RawContentLine, parse_content_line};
use lines::LogicalLines;

use crate::{
    CalendarRegistry,
    calendar::lookup_calendar,
    error::{ParseError, ParseErrorKind},
    parameters::{AnyParameter, Calscale, Parameters},
    property::{ExtensionProperty, GroupName, Property, PropertyValue},
    values::{DateAndOrTimeOrText, Token},
    vcard::{VCard, for_each_property},
};

impl FromStr for VCard {
    type Err = ParseError;

    /// Parses text that contains exactly one vCard.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_with_calendar_registry(s, None)
    }
}

impl VCard {
    /// Parses one vCard with external handlers for non-Gregorian BDAY and ANNIVERSARY values.
    /// Properties with an unknown or conflicting CALSCALE are ignored before their values or cardinality are checked.
    pub fn parse_with_calendars(
        s: &str,
        calendars: &dyn CalendarRegistry,
    ) -> Result<Self, ParseError> {
        Self::parse_with_calendar_registry(s, Some(calendars))
    }

    fn parse_with_calendar_registry(
        s: &str,
        calendars: Option<&dyn CalendarRegistry>,
    ) -> Result<Self, ParseError> {
        let mut lines = LogicalLines::new(s);

        let Some(vcard) = parse_card(&mut lines, calendars)? else {
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

    /// Parses text that contains any number of vCards, e.g. the content of a `.vcf` file with several contacts.
    pub fn parse_multiple(s: &str) -> Result<Vec<Self>, ParseError> {
        Self::parse_multiple_with_calendar_registry(s, None)
    }

    /// Parses any number of vCards with external handlers for non-Gregorian dates.
    /// Properties with an unknown or conflicting CALSCALE are ignored, as in [`VCard::parse_with_calendars`].
    pub fn parse_multiple_with_calendars(
        s: &str,
        calendars: &dyn CalendarRegistry,
    ) -> Result<Vec<Self>, ParseError> {
        Self::parse_multiple_with_calendar_registry(s, Some(calendars))
    }

    fn parse_multiple_with_calendar_registry(
        s: &str,
        calendars: Option<&dyn CalendarRegistry>,
    ) -> Result<Vec<Self>, ParseError> {
        let mut lines = LogicalLines::new(s);

        let mut vcards = Vec::new();

        while let Some(vcard) = parse_card(&mut lines, calendars)? {
            vcards.push(vcard);
        }

        Ok(vcards)
    }
}

/// Parses one vCard from the line iterator, returning `None` when no lines are left.
fn parse_card(
    lines: &mut LogicalLines,
    calendars: Option<&dyn CalendarRegistry>,
) -> Result<Option<VCard>, ParseError> {
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

    let (version_parameters, value_type, _) = Parameters::parse(content.params, "VERSION")
        .map_err(|kind| ParseError {
            line: number,
            kind,
        })?;

    // The only allowed VALUE is the default text type, which is dropped like the default VALUE of other properties.
    if value_type.is_some_and(|value_type| value_type != "text") {
        return Err(ParseError {
            line: number,
            kind: ParseErrorKind::InvalidValue {
                property: String::from("VERSION")
            },
        });
    }

    let mut vcard = VCard {
        version_parameters,
        ..VCard::default()
    };

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

        add_property(&mut vcard, number, content, calendars)?;
    }
}

/// Parses one property line and stores it into the right field of the vCard.
fn add_property(
    vcard: &mut VCard,
    number: usize,
    content: RawContentLine,
    calendars: Option<&dyn CalendarRegistry>,
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

    let upper_name = name.to_ascii_uppercase();

    let (parameters, value_type, conflicting_calscale) = Parameters::parse(params, &upper_name)
        .map_err(|kind| ParseError {
            line: number,
            kind,
        })?;

    let value_type = value_type.as_deref();

    let is_date = matches!(upper_name.as_str(), "BDAY" | "ANNIVERSARY");

    // Conflicting CALSCALE values cannot be understood either, so the property is ignored like an unknown CALSCALE.
    if is_date && conflicting_calscale {
        return Ok(());
    }

    let calendar = if is_date && let Some(Calscale::Extension(scale)) = &parameters.calscale {
        let calendar = lookup_calendar(calendars, scale.as_str()).map_err(|_| ParseError {
            line: number,
            kind: ParseErrorKind::InvalidParameter(String::from("CALSCALE")),
        })?;

        let Some(calendar) = calendar else {
            return Ok(());
        };

        Some(calendar)
    } else {
        None
    };

    let parse_date_value = || match calendar {
        Some(calendar) => {
            DateAndOrTimeOrText::parse_with_calendar(value, value_type, &parameters, calendar)
        },
        None => <DateAndOrTimeOrText as PropertyValue>::parse_value(value, value_type),
    };

    macro_rules! value_result {
        (birthdays) => {
            parse_date_value()
        };
        (anniversaries) => {
            parse_date_value()
        };
        ($field:ident) => {
            PropertyValue::parse_value(value, value_type)
        };
    }

    // Parses the raw value, where the target type is inferred from the field the result goes into.
    macro_rules! parse_value {
        ($field:ident, $property_name:literal) => {
            match value_result!($field) {
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
        (many, $field:ident, $property_name:literal) => {{
            let value = parse_value!($field, $property_name);

            vcard.$field.push(Property {
                group,
                parameters,
                value,
            })
        }};
        (one, $field:ident, $property_name:literal) => {{
            if let Some(first) = vcard.$field.first()
                && (parameters.altid.is_none() || first.parameters.altid != parameters.altid)
            {
                return Err(ParseError {
                    line: number,
                    kind: ParseErrorKind::DuplicateProperty(String::from($property_name)),
                });
            }

            let value = parse_value!($field, $property_name);

            vcard.$field.push(Property {
                group,
                parameters,
                value,
            })
        }};
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

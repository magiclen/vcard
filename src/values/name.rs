//! Structured name and address values, extended by RFC 9554.

use std::{
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
};

use crate::{
    error::InvalidValueError,
    syntax::{split_unescaped, unescape_text, write_escaped_text},
};

/// Writes one component which is a comma-separated list of escaped text items.
fn write_component(f: &mut Formatter, list: &[String]) -> fmt::Result {
    for (i, item) in list.iter().enumerate() {
        if i > 0 {
            f.write_char(',')?;
        }

        write_escaped_text(f, item, true)?;
    }

    Ok(())
}

/// Parses one component into a list of unescaped text items.
fn parse_component(s: Option<&str>) -> Vec<String> {
    match s {
        Some(s) if !s.is_empty() => split_unescaped(s, b',').map(unescape_text).collect(),
        _ => Vec::new(),
    }
}

/// Writes semicolon-separated components, keeping at least `min` components and dropping unused trailing ones.
fn write_components(f: &mut Formatter, components: &[&[String]], min: usize) -> fmt::Result {
    let count =
        components.iter().rposition(|list| !list.is_empty()).map_or(min, |last| min.max(last + 1));

    for (i, list) in components[..count].iter().enumerate() {
        if i > 0 {
            f.write_char(';')?;
        }

        write_component(f, list)?;
    }

    Ok(())
}

/// The structured value of the N property.
///
/// RFC 6350 defines the first five components and RFC 9554 adds the last two.
/// Each component can hold multiple values.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NameValue {
    /// The family names, also known as surnames.
    pub family_names:       Vec<String>,
    /// The given names.
    pub given_names:        Vec<String>,
    /// The additional names, e.g. middle names.
    pub additional_names:   Vec<String>,
    /// The honorific prefixes, e.g. `Mr.` or `Dr.`.
    pub honorific_prefixes: Vec<String>,
    /// The honorific suffixes, e.g. `Jr.` or `M.D.`.
    pub honorific_suffixes: Vec<String>,
    /// The secondary surnames, added by RFC 9554.
    pub surname2:           Vec<String>,
    /// The generation markers or qualifiers, e.g. `Jr.` or `III`, added by RFC 9554.
    pub generation:         Vec<String>,
}

impl NameValue {
    /// Creates an empty name value.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns all components in the order defined by RFC 6350 and RFC 9554.
    pub(crate) fn components(&self) -> [&[String]; 7] {
        [
            &self.family_names,
            &self.given_names,
            &self.additional_names,
            &self.honorific_prefixes,
            &self.honorific_suffixes,
            &self.surname2,
            &self.generation,
        ]
    }
}

impl Display for NameValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // The two RFC 9554 components are written only when they are used, for backward compatibility.
        write_components(f, &self.components(), 5)
    }
}

impl FromStr for NameValue {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = split_unescaped(s, b';');

        // Components after the seventh one are ignored, as required for unknown components.
        Ok(Self {
            family_names:       parse_component(parts.next()),
            given_names:        parse_component(parts.next()),
            additional_names:   parse_component(parts.next()),
            honorific_prefixes: parse_component(parts.next()),
            honorific_suffixes: parse_component(parts.next()),
            surname2:           parse_component(parts.next()),
            generation:         parse_component(parts.next()),
        })
    }
}

/// The structured value of the ADR property.
///
/// RFC 6350 defines the first seven components and RFC 9554 adds the remaining eleven.
/// Each component can hold multiple values.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AddressValue {
    /// The post office boxes.
    pub post_office_boxes:  Vec<String>,
    /// The extended addresses, e.g. apartment or suite numbers.
    pub extended_addresses: Vec<String>,
    /// The street addresses.
    pub street_addresses:   Vec<String>,
    /// The localities, e.g. cities.
    pub localities:         Vec<String>,
    /// The regions, e.g. states or provinces.
    pub regions:            Vec<String>,
    /// The postal codes.
    pub postal_codes:       Vec<String>,
    /// The country names.
    pub countries:          Vec<String>,
    /// The rooms, suite numbers or identifiers, added by RFC 9554.
    pub rooms:              Vec<String>,
    /// The extension designations such as apartment numbers, added by RFC 9554.
    pub apartments:         Vec<String>,
    /// The building floors or levels, added by RFC 9554.
    pub floors:             Vec<String>,
    /// The street numbers, added by RFC 9554.
    pub street_numbers:     Vec<String>,
    /// The street names, added by RFC 9554.
    pub street_names:       Vec<String>,
    /// The buildings, towers or condominiums, added by RFC 9554.
    pub buildings:          Vec<String>,
    /// The block names or numbers, added by RFC 9554.
    pub blocks:             Vec<String>,
    /// The subdistricts, added by RFC 9554.
    pub subdistricts:       Vec<String>,
    /// The districts, added by RFC 9554.
    pub districts:          Vec<String>,
    /// The landmarks or prominent features, added by RFC 9554.
    pub landmarks:          Vec<String>,
    /// The cardinal directions or quadrants, added by RFC 9554.
    pub directions:         Vec<String>,
}

impl AddressValue {
    /// Creates an empty address value.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns all components in the order defined by RFC 6350 and RFC 9554.
    pub(crate) fn components(&self) -> [&[String]; 18] {
        [
            &self.post_office_boxes,
            &self.extended_addresses,
            &self.street_addresses,
            &self.localities,
            &self.regions,
            &self.postal_codes,
            &self.countries,
            &self.rooms,
            &self.apartments,
            &self.floors,
            &self.street_numbers,
            &self.street_names,
            &self.buildings,
            &self.blocks,
            &self.subdistricts,
            &self.districts,
            &self.landmarks,
            &self.directions,
        ]
    }
}

impl Display for AddressValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // The RFC 9554 components are written only when they are used, for backward compatibility.
        write_components(f, &self.components(), 7)
    }
}

impl FromStr for AddressValue {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = split_unescaped(s, b';');

        // Components after the eighteenth one are ignored, as required for unknown components.
        Ok(Self {
            post_office_boxes:  parse_component(parts.next()),
            extended_addresses: parse_component(parts.next()),
            street_addresses:   parse_component(parts.next()),
            localities:         parse_component(parts.next()),
            regions:            parse_component(parts.next()),
            postal_codes:       parse_component(parts.next()),
            countries:          parse_component(parts.next()),
            rooms:              parse_component(parts.next()),
            apartments:         parse_component(parts.next()),
            floors:             parse_component(parts.next()),
            street_numbers:     parse_component(parts.next()),
            street_names:       parse_component(parts.next()),
            buildings:          parse_component(parts.next()),
            blocks:             parse_component(parts.next()),
            subdistricts:       parse_component(parts.next()),
            districts:          parse_component(parts.next()),
            landmarks:          parse_component(parts.next()),
            directions:         parse_component(parts.next()),
        })
    }
}

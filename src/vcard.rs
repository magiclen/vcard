//! The `VCard` struct and its serialization.

use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter, Write as _},
    fs::File,
    io::{self, BufWriter, Write as _},
    path::Path,
};

use crate::{
    CalendarRegistry,
    calendar::lookup_calendar,
    error::ValidationError,
    fold::FoldingWriter,
    parameters::{Parameters, Phonetic, is_known_parameter},
    property::{
        Address, Anniversary, Birthday, CalendarAddressUri, CalendarUri, Categories, ClientPidMap,
        Created, Email, ExtensionProperty, Fburl, FormattedName, Gender, Geo, GramGender, Impp,
        Key, Kind, Lang, Language, Logo, Member, Name, Nickname, Note, Org, Photo, ProdId,
        Pronouns, Property, Related, Rev, Role, SocialProfile, Sound, Source, Tel, TimeZone, Title,
        Uid, Url, Xml, write_property,
    },
    values::{
        AddressValue, DateAndOrTime, DateAndOrTimeOrText, KindValue, LanguageTag, NameValue,
        TextOrUri,
    },
};

/// Calls another macro with the whole property table.
///
/// Every entry is `(field, "NAME", parse_limit)`, where `one` allows one logical property and `many` has no parser limit.
/// This table drives the serializer, the parser dispatch and the validator, so they can never get out of sync.
macro_rules! for_each_property {
    ($callback:ident) => {
        $callback! {
            (sources, "SOURCE", many),
            (kinds, "KIND", one),
            (xmls, "XML", many),
            (formatted_names, "FN", many),
            (names, "N", one),
            (nicknames, "NICKNAME", many),
            (photos, "PHOTO", many),
            (birthdays, "BDAY", one),
            (anniversaries, "ANNIVERSARY", one),
            (genders, "GENDER", one),
            (addresses, "ADR", many),
            (telephones, "TEL", many),
            (emails, "EMAIL", many),
            (impps, "IMPP", many),
            (langs, "LANG", many),
            (time_zones, "TZ", many),
            (geos, "GEO", many),
            (titles, "TITLE", many),
            (roles, "ROLE", many),
            (logos, "LOGO", many),
            (organizations, "ORG", many),
            (members, "MEMBER", many),
            (relations, "RELATED", many),
            (categories, "CATEGORIES", many),
            (notes, "NOTE", many),
            (product_ids, "PRODID", one),
            (revisions, "REV", one),
            (sounds, "SOUND", many),
            (uids, "UID", one),
            (client_pid_maps, "CLIENTPIDMAP", many),
            (urls, "URL", many),
            (keys, "KEY", many),
            (fburls, "FBURL", many),
            (calendar_address_uris, "CALADRURI", many),
            (calendar_uris, "CALURI", many),
            (created, "CREATED", one),
            (gram_genders, "GRAMGENDER", many),
            (languages, "LANGUAGE", one),
            (pronouns, "PRONOUNS", many),
            (social_profiles, "SOCIALPROFILE", many),
        }
    };
}

pub(crate) use for_each_property;

/// A vCard as defined by RFC 6350 and extended by RFC 9554.
///
/// Every property is stored in a `Vec` field where an empty list means the property is absent.
/// A property limited to one logical instance can have several representations when they all share the same ALTID.
/// BEGIN, END and the VERSION value are handled automatically, while VERSION parameters are kept separately.
///
/// # Examples
///
/// ```rust
/// use vcard::VCard;
///
/// let vcard = VCard::new("Magic Len");
///
/// assert_eq!(
///     "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Magic Len\r\nEND:VCARD\r\n",
///     vcard.to_string()
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VCard {
    /// The parameters of the automatic VERSION property.
    pub version_parameters:    Parameters,
    /// The SOURCE properties.
    pub sources:               Vec<Source>,
    /// The KIND properties, which share the same ALTID when there are several of them.
    pub kinds:                 Vec<Kind>,
    /// The XML properties.
    pub xmls:                  Vec<Xml>,
    /// The FN properties, where at least one is required for a valid vCard.
    pub formatted_names:       Vec<FormattedName>,
    /// The N properties, which share the same ALTID when there are several of them.
    pub names:                 Vec<Name>,
    /// The NICKNAME properties.
    pub nicknames:             Vec<Nickname>,
    /// The PHOTO properties.
    pub photos:                Vec<Photo>,
    /// The BDAY properties, which share the same ALTID when there are several of them.
    pub birthdays:             Vec<Birthday>,
    /// The ANNIVERSARY properties, which share the same ALTID when there are several of them.
    pub anniversaries:         Vec<Anniversary>,
    /// The GENDER properties, which share the same ALTID when there are several of them.
    pub genders:               Vec<Gender>,
    /// The ADR properties.
    pub addresses:             Vec<Address>,
    /// The TEL properties.
    pub telephones:            Vec<Tel>,
    /// The EMAIL properties.
    pub emails:                Vec<Email>,
    /// The IMPP properties.
    pub impps:                 Vec<Impp>,
    /// The LANG properties.
    pub langs:                 Vec<Lang>,
    /// The TZ properties.
    pub time_zones:            Vec<TimeZone>,
    /// The GEO properties.
    pub geos:                  Vec<Geo>,
    /// The TITLE properties.
    pub titles:                Vec<Title>,
    /// The ROLE properties.
    pub roles:                 Vec<Role>,
    /// The LOGO properties.
    pub logos:                 Vec<Logo>,
    /// The ORG properties.
    pub organizations:         Vec<Org>,
    /// The MEMBER properties, which can only be used when the KIND property is set to `group`.
    pub members:               Vec<Member>,
    /// The RELATED properties.
    pub relations:             Vec<Related>,
    /// The CATEGORIES properties.
    pub categories:            Vec<Categories>,
    /// The NOTE properties.
    pub notes:                 Vec<Note>,
    /// The PRODID properties, which share the same ALTID when there are several of them.
    pub product_ids:           Vec<ProdId>,
    /// The REV properties, which share the same ALTID when there are several of them.
    pub revisions:             Vec<Rev>,
    /// The SOUND properties.
    pub sounds:                Vec<Sound>,
    /// The UID properties, which share the same ALTID when there are several of them.
    pub uids:                  Vec<Uid>,
    /// The CLIENTPIDMAP properties.
    pub client_pid_maps:       Vec<ClientPidMap>,
    /// The URL properties.
    pub urls:                  Vec<Url>,
    /// The KEY properties.
    pub keys:                  Vec<Key>,
    /// The FBURL properties.
    pub fburls:                Vec<Fburl>,
    /// The CALADRURI properties.
    pub calendar_address_uris: Vec<CalendarAddressUri>,
    /// The CALURI properties.
    pub calendar_uris:         Vec<CalendarUri>,
    /// The CREATED properties defined by RFC 9554, which share the same ALTID when there are several of them.
    pub created:               Vec<Created>,
    /// The GRAMGENDER properties defined by RFC 9554.
    pub gram_genders:          Vec<GramGender>,
    /// The LANGUAGE properties defined by RFC 9554, which share the same ALTID when there are several of them.
    pub languages:             Vec<Language>,
    /// The PRONOUNS properties defined by RFC 9554.
    pub pronouns:              Vec<Pronouns>,
    /// The SOCIALPROFILE properties defined by RFC 9554.
    pub social_profiles:       Vec<SocialProfile>,
    /// The x-name and IANA extension properties.
    pub extensions:            Vec<ExtensionProperty>,
}

impl VCard {
    /// Creates a vCard with a single FN property built from the given name.
    ///
    /// Unlike older versions of this crate, the REV property is no longer set automatically.
    #[inline]
    pub fn new<S: Into<String>>(formatted_name: S) -> Self {
        Self {
            formatted_names: vec![FormattedName::new(formatted_name.into())],
            ..Self::default()
        }
    }

    /// Checks the semantic rules that the type system cannot enforce.
    ///
    /// Checks required properties, ALTID cardinality, the group kind for MEMBER, and the service type for text SOCIALPROFILE values.
    /// Also checks phonetic representations, GRAMGENDER languages, LANGUAGE and USERNAME usage, and non-empty AUTHOR-NAME values.
    /// Checks PID sources, reserved extension names, line breaks in raw extension values, generic parameter conflicts, and MEDIATYPE quoting.
    /// Checks TYPE usage, the date required by CALSCALE, and SORT-AS component counts.
    /// Checks that each date has the same calendar as its CALSCALE parameter.
    /// Use [`VCard::validate_with_calendars`] to check external calendar rules again after changing property parameters.
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.validate_with_calendar_registry(None)
    }

    /// Checks this vCard and validates its non-Gregorian dates with the supplied calendar registry.
    /// This checks current property parameters, including any extension parameters used by a calendar handler.
    pub fn validate_with_calendars(
        &self,
        calendars: &dyn CalendarRegistry,
    ) -> Result<(), ValidationError> {
        self.validate_with_calendar_registry(Some(calendars))
    }

    fn validate_with_calendar_registry(
        &self,
        calendars: Option<&dyn CalendarRegistry>,
    ) -> Result<(), ValidationError> {
        if self.formatted_names.is_empty() {
            return Err(ValidationError::MissingFormattedName);
        }

        if !self.members.is_empty()
            && (self.kinds.is_empty()
                || self.kinds.iter().any(|kind| kind.value != KindValue::Group))
        {
            return Err(ValidationError::MemberWithoutGroupKind);
        }

        macro_rules! validate_field {
            (many, $field:ident, $name:literal) => {};
            (one, $field:ident, $name:literal) => {
                if !has_single_logical_property(&self.$field) {
                    return Err(ValidationError::DuplicateProperty(String::from($name)));
                }
            };
        }

        macro_rules! validate_fields {
            ($(($field:ident, $name:literal, $limit:tt)),* $(,)?) => {
                $(validate_field!($limit, $field, $name);)*
            };
        }

        for_each_property!(validate_fields);

        let mut sources = HashSet::with_capacity(self.client_pid_maps.len());

        for map in &self.client_pid_maps {
            if map.value.source_id == 0 {
                return Err(ValidationError::InvalidClientPidMap);
            }

            sources.insert(map.value.source_id);
        }

        validate_parameters(&self.version_parameters, "VERSION", false, false, &sources)?;

        macro_rules! allows_pid {
            (one) => {
                false
            };
            (many) => {
                true
            };
        }

        macro_rules! validate_property_parameters {
            ($(($field:ident, $name:literal, $limit:tt)),* $(,)?) => {
                $(for property in &self.$field {
                    validate_parameters(
                        &property.parameters,
                        $name,
                        false,
                        allows_pid!($limit) && $name != "CLIENTPIDMAP",
                        &sources,
                    )?;
                })*
            };
        }

        for_each_property!(validate_property_parameters);

        for extension in &self.extensions {
            let name = extension.name.as_str();

            if is_known_property(name) {
                return Err(ValidationError::ReservedExtensionName(name.to_string()));
            }

            if extension.value.contains(['\r', '\n']) {
                return Err(ValidationError::InvalidExtensionValue(name.to_string()));
            }

            validate_parameters(&extension.parameters, name, true, true, &sources)?;
        }

        validate_calendar_dates(&self.birthdays, "BDAY", calendars)?;
        validate_calendar_dates(&self.anniversaries, "ANNIVERSARY", calendars)?;

        for property in &self.names {
            validate_sort_as(&property.parameters, "N", property.value.component_count())?;
        }

        for property in &self.organizations {
            validate_sort_as(&property.parameters, "ORG", 1 + property.value.units.len())?;
        }

        if self.social_profiles.iter().any(|profile| {
            matches!(&profile.value, TextOrUri::Text(_))
                && profile.parameters.service_type.is_none()
        }) {
            return Err(ValidationError::SocialProfileWithoutServiceType);
        }

        validate_phonetic(&self.names, "N", NameValue::components)?;
        validate_phonetic(&self.addresses, "ADR", AddressValue::components)?;

        for (i, gender) in self.gram_genders.iter().enumerate() {
            if self.gram_genders[..i].iter().any(|other| {
                same_language(
                    gender.parameters.language.as_ref(),
                    other.parameters.language.as_ref(),
                )
            }) {
                return Err(ValidationError::DuplicateGramGenderLanguage);
            }
        }

        if self.languages.iter().any(|language| language.parameters.language.is_some()) {
            return Err(ValidationError::LanguageWithLanguageParameter);
        }

        if self.social_profiles.iter().any(|profile| {
            matches!(&profile.value, TextOrUri::Text(_)) && profile.parameters.username.is_some()
        }) {
            return Err(ValidationError::UsernameWithoutUriValue);
        }

        Ok(())
    }

    /// Serializes this vCard and writes it to a file.
    #[inline]
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        let mut file = BufWriter::new(File::create(path)?);

        write!(file, "{self}")?;

        file.flush()
    }
}

fn is_known_property(name: &str) -> bool {
    macro_rules! known_properties {
        ($(($field:ident, $name:literal, $limit:tt)),* $(,)?) => {
            ["BEGIN", "END", "VERSION", $($name),*].iter().any(|known| name.eq_ignore_ascii_case(known))
        };
    }

    for_each_property!(known_properties)
}

/// Checks the parameter rules shared by every property.
/// Only extension properties allow a VALUE parameter in `any`, because typed values write their own VALUE parameters.
fn validate_parameters(
    parameters: &Parameters,
    name: &str,
    allows_value_type: bool,
    allows_pid: bool,
    sources: &HashSet<u32>,
) -> Result<(), ValidationError> {
    let invalid = |parameter: &str| ValidationError::InvalidParameter {
        property:  name.to_string(),
        parameter: parameter.to_string(),
    };

    if parameters.author_name.as_ref().is_some_and(String::is_empty) {
        return Err(ValidationError::EmptyAuthorName(name.to_string()));
    }

    if parameters.media_type.as_ref().is_some_and(|mime| mime.as_ref().contains('"')) {
        return Err(ValidationError::UnrepresentableMediaType(name.to_string()));
    }

    if !allows_pid && !parameters.pids.is_empty() {
        return Err(ValidationError::PidNotAllowed(name.to_string()));
    }

    if !parameters.types.is_empty()
        && matches!(
            name,
            "VERSION"
                | "SOURCE"
                | "KIND"
                | "XML"
                | "N"
                | "BDAY"
                | "ANNIVERSARY"
                | "GENDER"
                | "MEMBER"
                | "PRODID"
                | "REV"
                | "UID"
                | "CLIENTPIDMAP"
        )
    {
        return Err(invalid("TYPE"));
    }

    for pid in &parameters.pids {
        if pid.id == 0 || pid.source == Some(0) {
            return Err(ValidationError::InvalidPid(name.to_string()));
        }

        if let Some(source) = pid.source
            && !sources.contains(&source)
        {
            return Err(ValidationError::MissingClientPidMap(source));
        }
    }

    let mut has_value_type = false;

    for parameter in &parameters.any {
        let upper_name = parameter.name.as_str().to_ascii_uppercase();

        match upper_name.as_str() {
            "VALUE" => {
                if !allows_value_type || has_value_type || parameter.values.len() != 1 {
                    return Err(invalid(&upper_name));
                }

                has_value_type = true;
            },
            known if is_known_parameter(known) => return Err(invalid(known)),
            _ => (),
        }
    }

    Ok(())
}

/// Checks that CALSCALE is used only with date or date-time values and matches the calendar of the date.
/// When a registry is given, non-Gregorian dates are also checked by their calendars with the current parameters.
fn validate_calendar_dates(
    properties: &[Property<DateAndOrTimeOrText>],
    name: &str,
    calendars: Option<&dyn CalendarRegistry>,
) -> Result<(), ValidationError> {
    let invalid = || ValidationError::InvalidParameter {
        property:  name.to_string(),
        parameter: String::from("CALSCALE"),
    };

    for property in properties {
        let date = match &property.value {
            DateAndOrTimeOrText::DateAndOrTime(DateAndOrTime::Date(date)) => Some(*date),
            DateAndOrTimeOrText::DateAndOrTime(DateAndOrTime::DateTime(date_time)) => {
                Some(date_time.date())
            },
            _ => None,
        };

        let Some(date) = date else {
            if property.parameters.calscale.is_some() {
                return Err(invalid());
            }

            continue;
        };

        let scale =
            property.parameters.calscale.as_ref().map_or("gregorian", |scale| scale.as_str());

        if !date.calendar_scale().eq_ignore_ascii_case(scale) {
            return Err(invalid());
        }

        if calendars.is_some() && date.calendar_scale() != "gregorian" {
            let calendar = lookup_calendar(calendars, date.calendar_scale())
                .map_err(|_| invalid())?
                .ok_or_else(invalid)?;

            if !calendar.is_valid_date(&date.parts(), &property.parameters) {
                return Err(invalid());
            }
        }
    }

    Ok(())
}

/// Checks that SORT-AS has no more values than the property has components.
fn validate_sort_as(
    parameters: &Parameters,
    name: &str,
    component_count: usize,
) -> Result<(), ValidationError> {
    if parameters.sort_as.len() > component_count {
        return Err(ValidationError::InvalidParameter {
            property:  name.to_string(),
            parameter: String::from("SORT-AS"),
        });
    }

    Ok(())
}

/// Checks whether two optional language tags are the same, where two missing tags also count as the same.
/// Language tags are compared case-insensitively.
fn same_language(first: Option<&LanguageTag>, second: Option<&LanguageTag>) -> bool {
    match (first, second) {
        (Some(first), Some(second)) => first.as_str().eq_ignore_ascii_case(second.as_str()),
        (None, None) => true,
        _ => false,
    }
}

/// Checks the PHONETIC rules of RFC 9554 section 4.6 for N or ADR properties.
fn validate_phonetic<V, const N: usize>(
    properties: &[Property<V>],
    name: &str,
    components: fn(&V) -> [&[String]; N],
) -> Result<(), ValidationError> {
    for (i, property) in properties.iter().enumerate() {
        let parameters = &property.parameters;

        let Some(phonetic) = &parameters.phonetic else {
            continue;
        };

        if *phonetic == Phonetic::Script && parameters.script.is_none() {
            return Err(ValidationError::PhoneticWithoutScript(name.to_string()));
        }

        let without_related = || ValidationError::PhoneticWithoutRelatedProperty(name.to_string());

        let altid = parameters.altid.as_ref().ok_or_else(without_related)?;
        let phonetic_components = components(&property.value);

        // A component set in the phonetic value must also be set at the same position in the related value.
        if !properties.iter().any(|related| {
            related.parameters.phonetic.is_none()
                && related.parameters.altid.as_ref() == Some(altid)
                && components(&related.value).into_iter().zip(phonetic_components).all(
                    |(original, pronounced)| {
                        pronounced.iter().all(String::is_empty)
                            || original.iter().any(|value| !value.is_empty())
                    },
                )
        }) {
            return Err(without_related());
        }

        // Phonetic properties with the same ALTID must have different LANGUAGE values, so at most one of them can omit it.
        if properties[..i].iter().any(|other| {
            other.parameters.phonetic.is_some()
                && other.parameters.altid.as_ref() == Some(altid)
                && same_language(parameters.language.as_ref(), other.parameters.language.as_ref())
        }) {
            return Err(ValidationError::DuplicatePhoneticLanguage(name.to_string()));
        }
    }

    Ok(())
}

fn has_single_logical_property<V>(properties: &[Property<V>]) -> bool {
    if properties.len() < 2 {
        return true;
    }

    let altid = properties[0].parameters.altid.as_deref();

    altid.is_some()
        && properties[1..].iter().all(|property| property.parameters.altid.as_deref() == altid)
}

impl Display for VCard {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut w = FoldingWriter::new(f);

        w.write_str("BEGIN:VCARD")?;
        w.end_line()?;

        w.write_str("VERSION")?;
        self.version_parameters.write(&mut w)?;
        w.write_str(":4.0")?;
        w.end_line()?;

        macro_rules! write_fields {
            ($(($field:ident, $name:literal, $limit:tt)),* $(,)?) => {
                $(for property in &self.$field {
                    write_property(&mut w, $name, property)?;
                })*
            };
        }

        for_each_property!(write_fields);

        for extension in &self.extensions {
            extension.write(&mut w)?;
        }

        w.write_str("END:VCARD")?;
        w.end_line()
    }
}

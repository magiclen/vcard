//! The `VCard` struct and its serialization.

use std::{
    fmt::{self, Display, Formatter, Write as _},
    fs::File,
    io::{self, BufWriter, Write as _},
    path::Path,
};

use crate::{
    error::ValidationError,
    fold::FoldingWriter,
    property::{
        Address, Anniversary, Birthday, CalendarAddressUri, CalendarUri, Categories, ClientPidMap,
        Created, Email, ExtensionProperty, Fburl, FormattedName, Gender, Geo, GramGender, Impp,
        Key, Kind, Lang, Language, Logo, Member, Name, Nickname, Note, Org, Photo, ProdId,
        Pronouns, Property, Related, Rev, Role, SocialProfile, Sound, Source, Tel, TimeZone, Title,
        Uid, Url, Xml, write_property,
    },
    values::{KindValue, TextOrUri},
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
/// The BEGIN, END and VERSION properties have no fields because they are handled automatically.
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
    pub fn validate(&self) -> Result<(), ValidationError> {
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

        if self.social_profiles.iter().any(|profile| {
            matches!(&profile.value, TextOrUri::Text(_))
                && profile.parameters.service_type.is_none()
        }) {
            return Err(ValidationError::SocialProfileWithoutServiceType);
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

        w.write_str("VERSION:4.0")?;
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

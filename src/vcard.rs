//! The `VCard` struct and its serialization.

use std::{
    fmt::{self, Display, Formatter, Write as _},
    fs, io,
    path::Path,
};

use crate::{
    error::ValidationError,
    fold::FoldingWriter,
    property::{
        Address, Anniversary, Birthday, CalendarAddressUri, CalendarUri, Categories, ClientPidMap,
        Created, Email, ExtensionProperty, Fburl, FormattedName, Gender, Geo, GramGender, Impp,
        Key, Kind, Lang, Language, Logo, Member, Name, Nickname, Note, Org, Photo, ProdId,
        Pronouns, Related, Rev, Role, SocialProfile, Sound, Source, Tel, TimeZone, Title, Uid, Url,
        Xml, write_property,
    },
    values::KindValue,
};

/// Calls another macro with the whole property table.
///
/// Every entry is `(field, "NAME", cardinality)` where the cardinality is `many` for a `Vec` field or `one` for an `Option` field.
/// This table drives both the serializer and the parser dispatch, so the two can never get out of sync.
macro_rules! for_each_property {
    ($callback:ident) => {
        $callback! {
            (sources, "SOURCE", many),
            (kind, "KIND", one),
            (xmls, "XML", many),
            (formatted_names, "FN", many),
            (names, "N", many),
            (nicknames, "NICKNAME", many),
            (photos, "PHOTO", many),
            (birthday, "BDAY", one),
            (anniversary, "ANNIVERSARY", one),
            (gender, "GENDER", one),
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
            (product_id, "PRODID", one),
            (revision, "REV", one),
            (sounds, "SOUND", many),
            (uid, "UID", one),
            (client_pid_maps, "CLIENTPIDMAP", many),
            (urls, "URL", many),
            (keys, "KEY", many),
            (fburls, "FBURL", many),
            (calendar_address_uris, "CALADRURI", many),
            (calendar_uris, "CALURI", many),
            (created, "CREATED", one),
            (gram_genders, "GRAMGENDER", many),
            (language, "LANGUAGE", one),
            (pronouns, "PRONOUNS", many),
            (social_profiles, "SOCIALPROFILE", many),
        }
    };
}

pub(crate) use for_each_property;

/// A vCard as defined by RFC 6350 and extended by RFC 9554.
///
/// A property whose cardinality allows several instances is a `Vec` field where an empty list means the property is absent.
/// A property that can appear at most once is an `Option` field.
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
    /// The KIND property.
    pub kind:                  Option<Kind>,
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
    /// The BDAY property.
    pub birthday:              Option<Birthday>,
    /// The ANNIVERSARY property.
    pub anniversary:           Option<Anniversary>,
    /// The GENDER property.
    pub gender:                Option<Gender>,
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
    /// The PRODID property.
    pub product_id:            Option<ProdId>,
    /// The REV property.
    pub revision:              Option<Rev>,
    /// The SOUND properties.
    pub sounds:                Vec<Sound>,
    /// The UID property.
    pub uid:                   Option<Uid>,
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
    /// The CREATED property defined by RFC 9554.
    pub created:               Option<Created>,
    /// The GRAMGENDER properties defined by RFC 9554.
    pub gram_genders:          Vec<GramGender>,
    /// The LANGUAGE property defined by RFC 9554.
    pub language:              Option<Language>,
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
    /// It currently checks that at least one FN property exists and that MEMBER is only used when KIND is `group`.
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.formatted_names.is_empty() {
            return Err(ValidationError::MissingFormattedName);
        }

        if !self.members.is_empty()
            && !self.kind.as_ref().is_some_and(|kind| kind.value == KindValue::Group)
        {
            return Err(ValidationError::MemberWithoutGroupKind);
        }

        Ok(())
    }

    /// Serializes this vCard and writes it to a file.
    #[inline]
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        fs::write(path, self.to_string())
    }
}

/// Writes the properties of one field, which is a `Vec` or an `Option` depending on the cardinality.
macro_rules! write_field {
    (many, $vcard:expr, $field:ident, $name:literal, $w:expr) => {
        for property in &$vcard.$field {
            write_property($w, $name, property)?;
        }
    };
    (one, $vcard:expr, $field:ident, $name:literal, $w:expr) => {
        if let Some(property) = &$vcard.$field {
            write_property($w, $name, property)?;
        }
    };
}

impl Display for VCard {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut w = FoldingWriter::new(f);

        w.write_str("BEGIN:VCARD")?;
        w.end_line()?;

        w.write_str("VERSION:4.0")?;
        w.end_line()?;

        macro_rules! write_fields {
            ($(($field:ident, $name:literal, $card:tt)),* $(,)?) => {
                $(write_field!($card, self, $field, $name, &mut w);)*
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

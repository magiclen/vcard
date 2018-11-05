use super::super::values::uri::URI;
use super::*;

use std::fmt::Display;
use std::path::Path;
use std::io::{self, ErrorKind, Read};
use std::fs::File;

use validators::{Validated, ValidatedWrapper};
use validators::base64::Base64;

use base64_stream::ToBase64Reader;
use mime_guess::Mime;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SoundValue {
    Base64(Mime, Base64),
    URI(URI),
}

impl SoundValue {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<SoundValue, io::Error> {
        Self::from_file_inner(path, None)
    }

    pub fn from_file_with_mime<P: AsRef<Path>>(path: P, mime: Mime) -> Result<SoundValue, io::Error> {
        Self::from_file_inner(path, Some(mime))
    }

    fn from_file_inner<P: AsRef<Path>>(path: P, mime: Option<Mime>) -> Result<SoundValue, io::Error> {
        let path = path.as_ref();

        let mime = match mime {
            Some(image_type) => image_type,
            None => {
                match path.extension() {
                    Some(ext) => match ext.to_str() {
                        Some(ext) => {
                            let mime = mime_guess::get_mime_type(ext);

                            let Mime(top, ..) = &mime;

                            if top != "audio" {
                                return Err(io::Error::new(ErrorKind::Other, "the media is not audio"));
                            }

                            mime
                        }
                        None => {
                            return Err(io::Error::new(ErrorKind::Other, "cannot find the media type, because of the unrecognized file extension name"));
                        }
                    },
                    None => {
                        return Err(io::Error::new(ErrorKind::Other, "cannot find the media type, because the file has no file extension name"));
                    }
                }
            }
        };

        let mut reader = ToBase64Reader::new(File::open(path)?);

        let mut base64_raw = Vec::new();

        reader.read_to_end(&mut base64_raw)?;

        let base64 = unsafe { String::from_utf8_unchecked(base64_raw) };

        let base64 = unsafe { Base64::from_string_unchecked(base64) };

        Ok(SoundValue::Base64(mime, base64))
    }
}

impl Value for SoundValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            SoundValue::Base64(typ, base64) => {
                f.write_str("data:")?;
                f.write_str(&typ.to_string())?;
                f.write_str(";base64,")?;
                f.write_str(base64.get_base64())?;
            }
            SoundValue::URI(uri) => {
                f.write_str(uri.get_full_uri())?;
            }
        }

        Ok(())
    }
}

impl Display for SoundValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for SoundValue {}

impl ValidatedWrapper for SoundValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
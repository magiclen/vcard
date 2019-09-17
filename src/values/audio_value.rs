use super::super::values::uri::URI;
use super::*;

use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use validators::base64::Base64;
use validators::{Validated, ValidatedWrapper};

use base64_stream::ToBase64Reader;
use mime::Mime;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum AudioValueInner {
    Base64(Mime, Base64),
    URI(URI),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AudioValue {
    inner: AudioValueInner,
}

#[derive(Debug)]
pub enum AudioValueError {
    FileMediaTypeCannotBeDefined,
    MediaTypeNotAudio,
    IOError(io::Error),
}

impl From<io::Error> for AudioValueError {
    #[inline]
    fn from(err: io::Error) -> Self {
        AudioValueError::IOError(err)
    }
}

impl AudioValue {
    pub fn from_base64(mime_type: Mime, base64: Base64) -> Result<AudioValue, AudioValueError> {
        if mime_type.type_() != mime::AUDIO {
            return Err(AudioValueError::MediaTypeNotAudio);
        }

        Ok(AudioValue {
            inner: AudioValueInner::Base64(mime_type, base64),
        })
    }

    pub fn from_uri(uri: URI) -> AudioValue {
        AudioValue {
            inner: AudioValueInner::URI(uri),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<AudioValue, AudioValueError> {
        Self::from_file_inner(path, None)
    }

    pub fn from_file_with_mime<P: AsRef<Path>>(
        path: P,
        mime_type: Mime,
    ) -> Result<AudioValue, AudioValueError> {
        Self::from_file_inner(path, Some(mime_type))
    }

    fn from_file_inner<P: AsRef<Path>>(
        path: P,
        mime_type: Option<Mime>,
    ) -> Result<AudioValue, AudioValueError> {
        let path = path.as_ref();

        let mime_type = match mime_type {
            Some(audio_type) => audio_type,
            None => {
                match path.extension() {
                    Some(ext) => {
                        match ext.to_str() {
                            Some(ext) => {
                                let mime_type = mime_guess::from_ext(ext).first_or_octet_stream();

                                if mime_type.type_() != mime::AUDIO {
                                    return Err(AudioValueError::MediaTypeNotAudio);
                                }

                                mime_type
                            }
                            None => {
                                return Err(AudioValueError::FileMediaTypeCannotBeDefined);
                            }
                        }
                    }
                    None => {
                        return Err(AudioValueError::FileMediaTypeCannotBeDefined);
                    }
                }
            }
        };

        let mut reader = ToBase64Reader::new(File::open(path)?);

        let mut base64_raw = Vec::new();

        reader.read_to_end(&mut base64_raw)?;

        let base64 = unsafe { String::from_utf8_unchecked(base64_raw) };

        let base64 = unsafe { Base64::from_string_unchecked(base64) };

        Ok(AudioValue {
            inner: AudioValueInner::Base64(mime_type, base64),
        })
    }
}

impl Value for AudioValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match &self.inner {
            AudioValueInner::Base64(typ, base64) => {
                f.write_str("data:")?;
                f.write_str(&typ.to_string())?;
                f.write_str(";base64,")?;
                f.write_str(base64.get_base64())?;
            }
            AudioValueInner::URI(uri) => {
                f.write_str(uri.get_full_uri())?;
            }
        }

        Ok(())
    }
}

impl Display for AudioValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for AudioValue {}

impl ValidatedWrapper for AudioValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

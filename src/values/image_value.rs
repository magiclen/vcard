use std::{
    fmt::Display,
    fs::File,
    io::{self, Read},
    path::Path,
};

use base64_stream::ToBase64Reader;
use mime::Mime;
use validators::{base64::Base64, Validated, ValidatedWrapper};

use super::{super::values::uri::URI, *};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
enum ImageValueInner {
    Base64(Mime, Base64),
    URI(URI),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImageValue {
    inner: ImageValueInner,
}

#[derive(Debug)]
pub enum ImageValueError {
    FileMediaTypeCannotBeDefined,
    MediaTypeNotImage,
    IOError(io::Error),
}

impl From<io::Error> for ImageValueError {
    #[inline]
    fn from(err: io::Error) -> Self {
        ImageValueError::IOError(err)
    }
}

impl ImageValue {
    pub fn from_base64(mime_type: Mime, base64: Base64) -> Result<ImageValue, ImageValueError> {
        if mime_type.type_() != mime::IMAGE {
            return Err(ImageValueError::MediaTypeNotImage);
        }

        Ok(ImageValue {
            inner: ImageValueInner::Base64(mime_type, base64)
        })
    }

    pub fn from_uri(uri: URI) -> ImageValue {
        ImageValue {
            inner: ImageValueInner::URI(uri)
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<ImageValue, ImageValueError> {
        Self::from_file_inner(path, None)
    }

    pub fn from_file_with_mime<P: AsRef<Path>>(
        path: P,
        mime_type: Mime,
    ) -> Result<ImageValue, ImageValueError> {
        Self::from_file_inner(path, Some(mime_type))
    }

    fn from_file_inner<P: AsRef<Path>>(
        path: P,
        mime_type: Option<Mime>,
    ) -> Result<ImageValue, ImageValueError> {
        let path = path.as_ref();

        let mime_type = match mime_type {
            Some(image_type) => image_type,
            None => match path.extension() {
                Some(ext) => match ext.to_str() {
                    Some(ext) => {
                        let mime_type = mime_guess::from_ext(ext).first_or_octet_stream();

                        if mime_type.type_() != mime::IMAGE {
                            return Err(ImageValueError::MediaTypeNotImage);
                        }

                        mime_type
                    },
                    None => {
                        return Err(ImageValueError::FileMediaTypeCannotBeDefined);
                    },
                },
                None => {
                    return Err(ImageValueError::FileMediaTypeCannotBeDefined);
                },
            },
        };

        let mut reader = ToBase64Reader::new(File::open(path)?);

        let mut base64_raw = Vec::new();

        reader.read_to_end(&mut base64_raw)?;

        let base64 = unsafe { String::from_utf8_unchecked(base64_raw) };

        let base64 = unsafe { Base64::from_string_unchecked(base64) };

        Ok(ImageValue {
            inner: ImageValueInner::Base64(mime_type, base64)
        })
    }
}

impl Value for ImageValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match &self.inner {
            ImageValueInner::Base64(typ, base64) => {
                f.write_str("data:")?;
                f.write_str(typ.as_ref())?;
                f.write_str(";base64,")?;
                f.write_str(base64.get_base64())?;
            },
            ImageValueInner::URI(uri) => {
                f.write_str(uri.get_full_uri())?;
            },
        }

        Ok(())
    }
}

impl Display for ImageValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for ImageValue {}

impl ValidatedWrapper for ImageValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

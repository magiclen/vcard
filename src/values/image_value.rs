use super::*;

use std::fmt::Display;
use std::path::Path;
use std::io::{self, ErrorKind, Read};
use std::fs::File;

use validators::{Validated, ValidatedWrapper};
use validators::base64::Base64;
use validators::http_url::HttpUrlLocalableWithProtocol;

use base64_stream::ToBase64Reader;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImageType {
    JPEG,
    PNG,
    GIF,
    SVG,
    WEBP,
    ICO,
}

impl ImageType {
    pub fn get_str(&self) -> &'static str {
        match self {
            ImageType::JPEG => "image/jpeg",
            ImageType::PNG => "image/png",
            ImageType::GIF => "image/gif",
            ImageType::SVG => "image/svg+xml",
            ImageType::WEBP => "image/webp",
            ImageType::ICO => "image/x-icon",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImageValue {
    Base64(ImageType, Base64),
    URL(HttpUrlLocalableWithProtocol),
}

impl ImageValue {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<ImageValue, io::Error> {
        Self::from_file_inner(path, None)
    }

    pub fn from_file_with_image_type<P: AsRef<Path>>(path: P, image_type: ImageType) -> Result<ImageValue, io::Error> {
        Self::from_file_inner(path, Some(image_type))
    }

    fn from_file_inner<P: AsRef<Path>>(path: P, image_type: Option<ImageType>) -> Result<ImageValue, io::Error> {
        let path = path.as_ref();

        let image_type = match image_type {
            Some(image_type) => image_type,
            None => {
                match path.extension() {
                    Some(ext) => match ext.to_str() {
                        Some(ext) => match mime_guess::get_mime_type_str(ext) {
                            Some(t) => match t {
                                "image/jpeg" => ImageType::JPEG,
                                "image/png" => ImageType::PNG,
                                "image/gif" => ImageType::GIF,
                                "image/svg+xml" => ImageType::SVG,
                                "image/webp" => ImageType::WEBP,
                                "x-icon" => ImageType::ICO,
                                _ => {
                                    return Err(io::Error::new(
                                        ErrorKind::Other,
                                        "the media type of this file is not acceptable",
                                    ));
                                }
                            },
                            None => {
                                return Err(io::Error::new(ErrorKind::Other, "cannot find the media type, because of the unrecognized file extension name"));
                            }
                        },
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

        Ok(ImageValue::Base64(image_type, base64))
    }
}

impl Value for ImageValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            ImageValue::Base64(typ, base64) => {
                f.write_str("data:")?;
                f.write_str(typ.get_str())?;
                f.write_str(";base64,")?;
                f.write_str(base64.get_base64())?;
            }
            ImageValue::URL(url) => {
                f.write_str(url.get_full_http_url())?;
            }
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
use std::fmt::Write;

use idna::domain_to_ascii;
use validators::host::Host;

pub use super::super::validators::http_ftp_url::HttpFtpUrlLocalableWithProtocol as URL;
use super::*;
use crate::PATH_PERCENT_ENCODE_SET;

impl Value for URL {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_protocol())?;

        if self.is_absolute() {
            f.write_str("://")?;
        } else {
            f.write_char(':')?;
        }

        let host = self.get_host();

        if let Host::Domain(domain) = host {
            match domain_to_ascii(domain.get_full_domain_without_port()) {
                Ok(domain_without_port) => {
                    f.write_str(&domain_without_port)?;
                },
                Err(_) => {
                    return Err(fmt::Error);
                },
            }

            if let Some(port) = domain.get_port() {
                f.write_char(':')?;
                f.write_fmt(format_args!("{}", port))?;
            }
        } else {
            f.write_str(host.get_full_host())?;
        }

        if let Some(path) = self.get_path() {
            f.write_str(
                &percent_encoding::utf8_percent_encode(path, PATH_PERCENT_ENCODE_SET).to_string(),
            )?;
        }

        if let Some(query) = self.get_query() {
            f.write_char('?')?;
            f.write_str(
                &percent_encoding::utf8_percent_encode(query, PATH_PERCENT_ENCODE_SET).to_string(),
            )?;
        }

        if let Some(fragment) = self.get_fragment() {
            f.write_char('#')?;
            f.write_str(
                &percent_encoding::utf8_percent_encode(fragment, PATH_PERCENT_ENCODE_SET)
                    .to_string(),
            )?;
        }

        Ok(())
    }
}

pub use super::super::validators::http_ftp_url::HttpFtpUrlLocalableWithProtocol as URL;
use super::*;

impl Value for URL {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_full_http_ftp_url())?;

        Ok(())
    }
}

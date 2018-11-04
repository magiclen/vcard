use std::fmt::{self, Formatter};

mod begin;
mod end;
mod formatted_name;
mod version;
mod source;

pub use self::begin::Begin;
pub use self::end::End;
pub use self::version::Version;
pub use self::formatted_name::FormattedName;
pub use self::source::Source;

pub trait Property {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>;
}

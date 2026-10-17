//! External calendar validation for dates expressed with a CALSCALE parameter.

use std::borrow::Cow;

use crate::{InvalidValueError, Parameters, syntax::is_token, values::DateParts};

/// Validates dates in a calendar supplied by the application.
///
/// The crate handles Gregorian dates itself and does not provide other calendar implementations.
/// Calendar implementations validate their own month lengths, leap years and partial dates.
pub trait Calendar {
    /// Returns the stable lowercase name used by the CALSCALE parameter.
    /// The name must be a non-empty token of ASCII lowercase letters, digits and hyphens.
    /// The name must not be `gregorian`, which is reserved for the built-in calendar.
    /// Private calendar extensions should use an `x-` name with an agreed interpretation.
    fn name(&self) -> &'static str;

    /// Checks whether the date is valid in this calendar with the given property parameters.
    /// The crate checks the RFC 6350 date shape and numeric bounds before calling this method.
    /// Implementations may inspect extension parameters that distinguish dates in their calendar.
    fn is_valid_date(&self, date: &DateParts, parameters: &Parameters) -> bool;
}

/// Looks up calendar implementations supplied by the application.
///
/// The crate queries this registry only for non-Gregorian CALSCALE values.
/// No global registration or ownership of the calendar implementations is required.
pub trait CalendarRegistry {
    /// Returns the calendar for a lowercase CALSCALE value, or `None` when it is not understood.
    /// The returned calendar must have the same name as the requested scale.
    fn get(&self, scale: &str) -> Option<&dyn Calendar>;
}

const CALENDAR_NAME_ERROR: InvalidValueError = InvalidValueError::new("calendar name");

/// Checks a calendar's canonical name and prevents replacing the built-in Gregorian calendar.
pub(crate) fn calendar_name(calendar: &dyn Calendar) -> Result<&'static str, InvalidValueError> {
    let name = calendar.name();

    if !is_token(name) || name.bytes().any(|b| b.is_ascii_uppercase()) || name == "gregorian" {
        return Err(CALENDAR_NAME_ERROR);
    }

    Ok(name)
}

/// Resolves an extension calendar and checks that its canonical name matches the requested scale.
/// Gregorian and unknown scales return `None` so that the caller can handle them separately.
pub(crate) fn lookup_calendar<'a>(
    registry: Option<&'a dyn CalendarRegistry>,
    scale: &str,
) -> Result<Option<&'a dyn Calendar>, InvalidValueError> {
    if scale.eq_ignore_ascii_case("gregorian") {
        return Ok(None);
    }

    let Some(registry) = registry else {
        return Ok(None);
    };

    let scale = if scale.bytes().any(|b| b.is_ascii_uppercase()) {
        Cow::Owned(scale.to_ascii_lowercase())
    } else {
        Cow::Borrowed(scale)
    };

    let Some(calendar) = registry.get(&scale) else {
        return Ok(None);
    };

    if !calendar_name(calendar)?.eq_ignore_ascii_case(&scale) {
        return Err(CALENDAR_NAME_ERROR);
    }

    Ok(Some(calendar))
}

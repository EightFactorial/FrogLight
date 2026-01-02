//! TODO

use core::fmt::{Debug, Display};

/// An identifier.
pub struct Identifier();

// -------------------------------------------------------------------------------------------------

impl Display for Identifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "Identifier") }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Identifier").finish()
    }
}

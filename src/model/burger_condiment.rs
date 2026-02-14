use std::fmt;

// ===== Object declaration ====================================================

/// Represents a condiment that can be placed on a burger.
pub enum BurgerCondiment {
    Ketchup,
}

// ===== Display implementation ================================================

impl fmt::Display for BurgerCondiment {
    /// Formats the BurgerCondiment for display purposes.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BurgerCondiment::Ketchup => write!(f, "Ketchup"),
        }
    }
}

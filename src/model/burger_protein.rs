use std::fmt;

// ===== Object declaration ====================================================

/// Represents a protein patty that can be placed on a burger.
pub enum BurgerProtein {
    BeefPatty,
}

// ===== Display implementation ================================================

impl fmt::Display for BurgerProtein {
    /// Formats the BurgerProtein for display purposes.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BurgerProtein::BeefPatty => write!(f, "Beef Patty"),
        }
    }
}

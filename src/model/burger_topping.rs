use std::fmt;

// ===== Object declaration ====================================================

/// Represents 'topping' items that can be placed on a burger.
pub enum BurgerTopping {
    Lettuce,
}

// ===== Display implementation ================================================

impl fmt::Display for BurgerTopping {
    /// Formats the BurgerTopping for display purposes.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BurgerTopping::Lettuce => write!(f, "Lettuce"),
        }
    }
}

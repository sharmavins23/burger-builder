use crate::BurgerCondiment;
use crate::BurgerProtein;
use crate::BurgerTopping;
use std::fmt;

// ===== Object declaration ====================================================

/// Represents a singular component of a burger.
pub enum BurgerComponent {
    BottomBun,
    Condiment(BurgerCondiment),
    Protein(BurgerProtein),
    Topping(BurgerTopping),
    TopBun,
}

// ===== Display implementation ================================================

impl fmt::Display for BurgerComponent {
    /// Formats the BurgerComponent for display purposes.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BurgerComponent::BottomBun => write!(f, "Bottom Bun"),
            BurgerComponent::TopBun => write!(f, "Top Bun"),
            BurgerComponent::Condiment(c) => write!(f, "Condiment: {}", c),
            BurgerComponent::Protein(p) => write!(f, "Protein: {}", p),
            BurgerComponent::Topping(t) => write!(f, "Topping: {}", t),
        }
    }
}

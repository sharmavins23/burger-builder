use crate::BurgerComponent;

// ===== Object declaration ====================================================

/// Defines a fully-built burger.
pub struct Burger {
    pub components: Vec<BurgerComponent>,
    pub name: String,
}

// ===== Method implementations ================================================

impl Burger {
    /// Displays the components of the built burger.
    pub fn display(&self) {
        println!("Burger components for '{}':", self.name);
        for component in &self.components {
            println!("{}", component);
        }
    }
}

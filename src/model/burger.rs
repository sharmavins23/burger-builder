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
        println!(
            "Burger components for '{}' (score: {}):",
            self.name,
            self.score()
        );
        for component in &self.components {
            println!("{}", component);
        }
    }

    /// Scores a built burger.
    fn score(&self) -> u32 {
        let mut score = 100; // Start with a perfect score

        // Deduct 50 points if the burger doesn't have a protein
        if !self
            .components
            .iter()
            .any(|c| matches!(c, BurgerComponent::Protein(_)))
        {
            score -= 50;
        };

        // Deduct 50 points if the burger doesn't have a condiment
        if !self
            .components
            .iter()
            .any(|c| matches!(c, BurgerComponent::Condiment(_)))
        {
            score -= 50;
        };

        score
    }
}

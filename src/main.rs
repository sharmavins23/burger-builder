// ===== Lint configuration ====================================================

// Deny the use of `.expect()`
#![deny(clippy::expect_used)]
// Deny the use of `.unwrap()`
#![deny(clippy::unwrap_used)]
// Forbid any unsafe{} blocks in the codebase
#![forbid(unsafe_code)]

// ===== Imports ===============================================================

// * Model declarations
mod builder;
mod model;

// * Imports
use builder::burger_builder::BurgerBuilder;
use model::burger_component::BurgerComponent;
use model::burger_condiment::BurgerCondiment;
use model::burger_protein::BurgerProtein;
use model::burger_topping::BurgerTopping;

// ===== Driver code ===========================================================

/// Entrypoint to the application.
fn main() {
    let burger = BurgerBuilder::new("Classic Burger".to_string())
        .add_protein(BurgerProtein::BeefPatty)
        .add_topping(BurgerTopping::Lettuce)
        .add_condiment(BurgerCondiment::Ketchup)
        .build();

    // Print out our burger's configuration
    burger.display();
}

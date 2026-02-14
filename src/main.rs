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

// ===== Driver code ===========================================================

/// Entrypoint to the application.
fn main() {
    let burger = BurgerBuilder::new()
        .add_component(BurgerComponent::Patty)
        .add_component(BurgerComponent::Tomato)
        .add_component(BurgerComponent::Cheese)
        .add_component(BurgerComponent::Lettuce)
        .build();

    // Print out our burger's configuration
    println!("Burger components:");
    for component in burger.components {
        match component {
            BurgerComponent::BottomBun => println!("Bottom Bun"),
            BurgerComponent::Patty => println!("Patty"),
            BurgerComponent::Tomato => println!("Tomato"),
            BurgerComponent::Cheese => println!("Cheese"),
            BurgerComponent::Lettuce => println!("Lettuce"),
            BurgerComponent::TopBun => println!("Top Bun"),
        }
    }
}

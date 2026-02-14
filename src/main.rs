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
    let burger = BurgerBuilder::new()
        .add_component(BurgerComponent::Protein(BurgerProtein::BeefPatty))
        .add_component(BurgerComponent::Topping(BurgerTopping::Lettuce))
        .add_component(BurgerComponent::Condiment(BurgerCondiment::Ketchup))
        .build();

    // Print out our burger's configuration
    println!("Burger components:");
    for component in burger.components {
        match component {
            BurgerComponent::BottomBun => println!("Bottom Bun"),
            BurgerComponent::Protein(p) => match p {
                BurgerProtein::BeefPatty => println!("Protein: Beef Patty"),
            },
            BurgerComponent::Topping(t) => match t {
                BurgerTopping::Lettuce => println!("Topping: Lettuce"),
            },
            BurgerComponent::Condiment(c) => match c {
                BurgerCondiment::Ketchup => println!("Condiment: Ketchup"),
            },
            BurgerComponent::TopBun => println!("Top Bun"),
        }
    }
}

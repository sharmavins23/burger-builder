use crate::model::burger_component::BurgerComponent;

// ===== Object declaration ====================================================

/// Represents a burger being built, with a list of components that make up the
/// burger.
pub struct BurgerBuilder {
    pub components: Vec<BurgerComponent>,
}

// ===== Method implementations ================================================

impl BurgerBuilder {
    /// Creates a new BurgerBuilder instance, starting with a BottomBun as the
    /// first component of the burger.
    pub fn new() -> BurgerBuilder {
        BurgerBuilder {
            components: vec![BurgerComponent::BottomBun],
        }
    }

    /// Adds a component to the burger being built.
    pub fn add_component(mut self, component: BurgerComponent) -> BurgerBuilder {
        self.components.push(component);
        self
    }

    /// Finalizes the burger being built by adding a TopBun as the last
    /// component.
    pub fn build(mut self) -> BurgerBuilder {
        self.components.push(BurgerComponent::TopBun);
        self
    }
}

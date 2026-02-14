use crate::Burger;
use crate::BurgerComponent;
use crate::BurgerCondiment;
use crate::BurgerProtein;
use crate::BurgerTopping;

// ===== Object declaration ====================================================

/// Represents a burger being built, with a list of components that make up the
/// burger.
pub struct BurgerBuilder {
    pub components: Vec<BurgerComponent>,
    pub name: String,
}

// ===== Method implementations ================================================

impl BurgerBuilder {
    /// Creates a new BurgerBuilder instance, starting with a BottomBun as the
    /// first component of the burger.
    pub fn new(name: String) -> BurgerBuilder {
        BurgerBuilder {
            components: vec![BurgerComponent::BottomBun],
            name,
        }
    }

    /// Adds a component to the burger being built.
    fn add_component(mut self, component: BurgerComponent) -> BurgerBuilder {
        self.components.push(component);
        self
    }

    /// Adds a condiment to the burger being built.
    pub fn add_condiment(self, condiment: BurgerCondiment) -> BurgerBuilder {
        self.add_component(BurgerComponent::Condiment(condiment))
    }

    /// Adds a protein to the burger being built.
    pub fn add_protein(self, protein: BurgerProtein) -> BurgerBuilder {
        self.add_component(BurgerComponent::Protein(protein))
    }

    /// Adds a topping to the burger being built.
    pub fn add_topping(self, topping: BurgerTopping) -> BurgerBuilder {
        self.add_component(BurgerComponent::Topping(topping))
    }

    /// Finalizes the burger being built by adding a TopBun as the last
    /// component.
    pub fn build(mut self) -> Burger {
        self.components.push(BurgerComponent::TopBun);
        Burger {
            components: self.components,
            name: self.name,
        }
    }
}

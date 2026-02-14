use crate::model::burger_condiment::BurgerCondiment;
use crate::model::burger_protein::BurgerProtein;
use crate::model::burger_topping::BurgerTopping;

/// Represents a singular component of a burger.
pub enum BurgerComponent {
    BottomBun,
    Condiment(BurgerCondiment),
    Protein(BurgerProtein),
    Topping(BurgerTopping),
    TopBun,
}

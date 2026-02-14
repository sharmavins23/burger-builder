struct BurgerBuilder {
    components: Vec<BurgerComponent>,
}

enum BurgerComponent {
    BottomBun,
    Patty,
    Tomato,
    Cheese,
    Lettuce,
    TopBun,
}

impl BurgerBuilder {
    fn new() -> BurgerBuilder {
        BurgerBuilder {
            components: vec![BurgerComponent::BottomBun],
        }
    }

    pub fn add_component(mut self, component: BurgerComponent) -> BurgerBuilder {
        self.components.push(component);
        self
    }

    pub fn build(mut self) -> BurgerBuilder {
        self.components.push(BurgerComponent::TopBun);
        self
    }
}

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

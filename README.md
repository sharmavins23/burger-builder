# Burger Builder

Recently I've been learning Rust. Rather than learning it by jumping in the deep
end (as I've done with every other language I've learned), I figured I would
teach myself Rust from the bottom up. In order to learn languages, you have to
consistently try random semi-challenging things.

I was watching through a
[YouTube video](https://www.youtube.com/watch?v=1Ql7sQG8snA) on Rust design
patterns when I saw this toy example of pseudo OOP. Being a (current) career
Java programmer, and having seen the builder pattern being used here, I had to
play around with it.

This repository is mainly for me to learn and understand:

- Simple OOP in Rust;
- Consistent type safety and making invalid states unrepresentable (a core
  feature of Rust);
- How to split code up into individual files, which is something I prioritize
  when programming;
- Options for cleaner builder patterns.

## Task

As this is the first "project" (super simple, of course) that I'm doing in Rust,
I'm focusing on a simple refactoring task: Take the following code and make it
look like something that I would be comfortable pushing into production.

```rust
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
```

## Features I've Added

- Code separation. Splitting up the implementation into different files makes
  things easier to maintain and more readable.
- Refactored patties, condiments, and etc. out to separate enumerations.
- Stopped users from adding extra top/bottom buns into the burger.
    - Rather than doing this as a runtime check, this is implemented as a
      compile-time _typing_, which stops users' code from compiling if they
      choose to add a top or bottom bun. Basically, this invalid state of a
      burger is unreachable.

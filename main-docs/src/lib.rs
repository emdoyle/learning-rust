mod coffee_house;

use coffee_house::{CoffeeBrewer, CoffeeSize, Roast};

pub fn get_coffee() -> coffee_house::Coffee {
    let mut brewer = CoffeeBrewer::factory();
    brewer.brew_coffee(Roast::Dark, CoffeeSize::Grande);
    let mut coffee = brewer.get_coffee();
    println!("Got {:?}", coffee);
    coffee = brewer.get_coffee();
    println!("Got {:?}", coffee);
    brewer.brew_coffee(Roast::Medium, CoffeeSize::Venti);
    coffee = brewer.get_coffee();
    println!("Got {:?}", coffee);
    coffee
}

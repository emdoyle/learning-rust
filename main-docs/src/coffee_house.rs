#[derive(Copy, Clone, Debug)]
pub enum Roast {
    Light,
    Medium,
    Dark,
}

#[derive(Copy, Clone, Debug)]
pub enum CoffeeSize {
    Tall,
    Grande,
    Venti,
}

#[derive(Copy, Clone, Debug)]
enum Barista {
    Evan,
    Isik,
}

#[derive(Debug)]
pub struct Coffee {
    pub roast: Roast,
    pub size: CoffeeSize,
    barista: Barista,
}

#[derive(Debug)]
pub struct CoffeeBrewer {
    cups: [Option<Coffee>; 3],
}

impl CoffeeBrewer {
    pub fn factory() -> CoffeeBrewer {
        CoffeeBrewer {
            cups: [None, None, None],
        }
    }

    pub fn brew_coffee(&mut self, roast: Roast, size: CoffeeSize) {
        let mut first_open_index: Option<usize> = None;
        for (index, cup) in self.cups.iter().enumerate() {
            if let None = cup {
                first_open_index = Some(index);
            }
        }
        if let Some(index) = first_open_index {
            self.cups[index] = Some(Coffee {
                roast,
                size,
                barista: Barista::Evan,
            })
        } else {
            println!("Can't brew any more coffee!")
        }
    }

    pub fn get_coffee(&mut self) -> Coffee {
        let mut chosen_index: Option<usize> = None;
        for (index, cup) in self.cups.iter().enumerate() {
            if let Some(_) = cup {
                chosen_index = Some(index);
            }
        }
        if let Some(index) = chosen_index {
            let current_coffee = self.cups[index]
                .as_ref()
                .expect("Could not find coffee even though chosen_index exists!");
            let coffee_copy = Coffee {
                roast: current_coffee.roast,
                size: current_coffee.size,
                barista: current_coffee.barista,
            };
            self.cups[index] = None;
            return coffee_copy;
        }
        Coffee {
            roast: Roast::Light,
            size: CoffeeSize::Tall,
            barista: Barista::Evan,
        }
    }
}

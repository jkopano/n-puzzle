use npuzzle_lib::core::{Dir, Order};

#[derive(Clone)]
pub enum So {
    Rdul,
    Rdlu,
    Drul,
    Drlu,
    Ulrd,
    Uldr,
    Lurd,
    Ludr,
}

impl So {
    pub fn values() -> &'static [So] {
        &[
            So::Rdul,
            So::Rdlu,
            So::Drul,
            So::Drlu,
            So::Ulrd,
            So::Uldr,
            So::Lurd,
            So::Ludr,
        ]
    }
}

impl From<So> for Order {
    fn from(search_order: So) -> Self {
        match search_order {
            So::Rdul => Order::Perm([Dir::Right, Dir::Down, Dir::Up, Dir::Left]),
            So::Rdlu => Order::Perm([Dir::Right, Dir::Down, Dir::Left, Dir::Up]),
            So::Drul => Order::Perm([Dir::Down, Dir::Right, Dir::Up, Dir::Left]),
            So::Drlu => Order::Perm([Dir::Down, Dir::Right, Dir::Left, Dir::Up]),
            So::Ulrd => Order::Perm([Dir::Up, Dir::Left, Dir::Right, Dir::Down]),
            So::Uldr => Order::Perm([Dir::Up, Dir::Left, Dir::Down, Dir::Right]),
            So::Lurd => Order::Perm([Dir::Left, Dir::Up, Dir::Right, Dir::Down]),
            So::Ludr => Order::Perm([Dir::Left, Dir::Up, Dir::Down, Dir::Right]),
        }
    }
}

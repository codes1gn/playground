
use std::fmt::Debug;
struct Apple {
    weight: f32,
    size: f32,
}
// use trait Fruit to bound Apple
trait Fruit {}
impl Fruit for Apple {}

#[derive(Debug)]
struct Price(f32);
// use trait Countable to bound Price
trait Countable: Debug {}
impl Countable for Price {}


impl Apple {
    pub fn new(weight: f32, size: f32) -> Self {
        Apple {
            weight: weight,
            size: size,
        }
    }
}

fn buy<T: Fruit>(thing: T) -> impl Countable {
    Price(0.1)
}

fn buy1(thing: &dyn Fruit) -> impl Countable {
    Price(0.1)
}

// fn buy2<T: Countable>(thing: &dyn Fruit) -> T {
//     Price(0.1)
// }
//
fn buy3<T>(thing: T) -> impl Countable 
where T: Fruit {
    Price(0.1)
}

fn main() {
    let apple = Apple::new(1.1, 2.2);
    println!("{:?}", buy(apple));

    let apple = Apple::new(1.1, 2.2);
    println!("{:?}", buy1(&apple));

    let apple = Apple::new(1.1, 2.2);
    println!("{:?}", buy3(apple));

    println!("Hello, world!");
}

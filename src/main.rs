use rand::Rng;

pub mod core_logic;
mod ui;

fn main() {
    println!("rand: {}", rand::thread_rng().gen::<f64>())
}

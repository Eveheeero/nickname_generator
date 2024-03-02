mod data_collector;
pub(crate) mod prelude;
use once_cell::sync::Lazy;

static DB: Lazy<sled::Db> = Lazy::new(|| sled::open(".nickname_generator").unwrap());

fn main() {
    println!("Hello, world!");
}

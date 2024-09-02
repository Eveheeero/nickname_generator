use std::io::Write;

pub(super) async fn main() {
    crate::prelude::init();

    let opendict_key = crate::prelude::get_opendict_key();
    if let Some(opendict_key) = opendict_key {
        println!("Current api key: {}", opendict_key);
    }
    print!("Please input api key: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    if !input.is_empty() {
        crate::prelude::set_opendict_key(input);
    }
}

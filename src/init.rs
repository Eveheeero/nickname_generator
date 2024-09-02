pub(super) async fn main() {
    let opendict_key = crate::prelude::get_opendict_key();
    if let Some(opendict_key) = opendict_key {
        println!("Current api key: {}", opendict_key);
    }
    println!("Please input api key: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    crate::prelude::set_opendict_key(input);
}

mod generator;

fn main() {
    println!("Hello, world!");

    generator::parse_json();
    generator::generate_schema();
}

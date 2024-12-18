fn main() {
    let mut mutable_string = String::from("World");

    mutable_string.push_str(" Driver's Championship");

    println!("{mutable_string}");
}
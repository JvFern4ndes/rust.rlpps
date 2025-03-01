fn main() {
    ownership();
}

fn ownership() {
    let uma_string = String::from("João");
    rouba(uma_string);

    println!("{}", uma_string);
}
fn rouba(string: String) {
    println!("{}", string);
}
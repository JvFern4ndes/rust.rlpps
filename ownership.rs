fn main() {
    ownership();
}

fn ownership() {
    let mut uma_string = String::from("João");
    rouba(&mut uma_string);

    println!("{}", uma_string);
}

fn rouba(string: &mut String) {
    string.push_str(" Victor");
    println!("{}", string);
}
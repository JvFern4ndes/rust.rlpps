const PI:f32 = 3.14;
static mut VARIAVEL_GLOBAL:u8 = 1;

fn main() {
    println!("PI = {}", PI);

    unsafe {
        println!("variavel_global = {}", VARIAVEL_GLOBAL);
    }
    let variavel:i32 = 300;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("decimal = {}", decimal);

    let mut booleana:bool = false;
    println!("Booleana = {}, Tamanho booleana = {}", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho do char = {}", std::mem::size_of_val(&letra));
}
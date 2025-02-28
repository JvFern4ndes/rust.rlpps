fn main() {
    multiplicador();
}

fn multiplicador() {
    let multiplicador:u8 = 8;
    let mut contador:u8 = 0;

    while contador < 10 {
        contador += 1;

        if contador == 5 {
            continue;
        }
        
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;
    loop {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);

        if contador == 10 {
            break;
        }
    }
}
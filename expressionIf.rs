fn condicionais() {
    let idade: u8 = 18;
    let pais_autorizaram = true;
    let eh_maior = idade >= 18;

    if idade >= 18 {
        println!("A idade é {} então está liberado", idade);
    } else if idade >= 16 && pais_autorizaram  {
        println!("A idade é {}, porém os pais autorizaram, então está liberado", idade);
    } else {
        println!("A idade é {} então não está liberado!", idade);
    }

    let condicao = if eh_maior { "É maior" } else { "É menor" };

    println!("{} de idade", condicao);
}

fn main() {
    condicionais();
}
fn main() {
    match_exercise();
}

fn match_exercise() {
    let linguagem = "Javascript";
    let proposito = match linguagem {
        "Java" => "Backend",
        "Javascript" => "Web",
        "Python" => "Data Science",
        _ => "Desconhecido"
    };

    println!("O propósito da linguagem {} é {}", linguagem, proposito);
}
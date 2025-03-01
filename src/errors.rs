fn main() {
    errors();
}

fn errors() {
    match result() {
        Ok(string) => println!("String de sucesso = {}", string),
        Err(number) => println!("Código de erro = {}", number)
    };
}

fn result() -> Result<String, u32> {
    {
        Ok(String::from("Requisição enviada com sucesso!"))
    }
}
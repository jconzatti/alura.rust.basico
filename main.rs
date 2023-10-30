const PI:f32 = 3.14;
static mut GLOBAL:u8 = 1;

fn main() {
    escopo();
    sombra();

    println!("Soma = {}", soma(2, 5));
    // println!("decimal = {}", decimal);
    verificacao_de_idade();
    tabuada_de_5();
    estrutura_match();
    ownership();
    pattern_matiching();
    disparador_de_erros();
}

fn escopo() {
    println!("PI = {}", PI);

    unsafe {
        println!("variavel_global = {}", GLOBAL);
    }

    let variavel:i32 = 300;
    println!("variavel = {}, tamanho = {} byte(s)", variavel, std::mem::size_of_val(&variavel));
    let variavel:i32 = 301;
    println!("variavel = {}, tamanho = {} byte(s)", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("decimal = {}", decimal);

    let booleana:bool = true;
    println!("Booleana = {}, Tamanho booleana = {} byte(s)", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho do char = {}", std::mem::size_of_val(&letra));
}

fn sombra() {
    let a = 123;

    {
        let b = 456;
        println!("dentro, b = {}", b);

        let a = 777;
        println!("dentro, a = {}", a);
    }

    println!("fora, a = {}", a);
}

fn soma(a:i32, b:i32) -> i32{
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn verificacao_de_idade(){
    let idade:u8 = 18;
    let responsavel_autorizou:bool = true;
    let eh_maior:bool = idade >= 18;
    if idade > 18 {
        println!("Pode entrar na balada!");
    } else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar na balada com a declaração assinada pelo responsável!");
    } else {
        println!("NÃO Pode entrar na balada!");
    }
    
    let condicao = if eh_maior {"maior"} else {"menor"};
    println!("É {} de idade!", condicao);
}

fn tabuada_de_5(){
    let multiplicador:u8 = 5;
    let mut contador:u8 = 0;
    while contador < 10 {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }
    contador = 0;
    loop{
        contador += 1;
        if contador == 5{
            continue;
        }
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
        if contador == 10{
            break;
        }
    }
    for i in 1..=10 { //equivalente a 1..11, a contagem vai de 1 a 10 (1 inclusive a 11 exclusive)
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }
}

fn estrutura_match(){
    let linguagem = "Delphi";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Ciência de Dados",
        "Delphi" => "Desktop",
        _ => "Desconhecido"
    };
    println!("O propósito de {} é {}!", linguagem, proposito);
}

fn ownership(){
    let mut uma_string:String = String::from("Jhoni");
    rouba(&mut uma_string);
    println!("{}", uma_string);
}

fn rouba(string:&mut String){
    string.push_str(" Conzatti");
    println!("{}", string)
}

fn pattern_matiching(){
    for x in 1..=20{
        println!("{}: {}", x, match x {
           1 => "Pouco",
           2 | 3 => "Um pouquinho",
           4..=10 => "Um bocado",
           _ if x % 2 == 0 => "Uma boa quantidade",
           _ => "Muito"
        });
    }
}

fn disparador_de_erros(){
    // let vetor = vec![1,2,3];
    // vetor[4];
    // panic!("Erro proposital!");
    match resultado(){
        Ok(s) => println!("{}", s),
        Err(e) => println!("O código de erro {}", e)
    }
}

fn resultado() ->Result<String,u8>{
    // Ok(String::from("Tudo deu certo"))
    Err(42)
}
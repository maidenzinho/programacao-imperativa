fn main(){
    let mut buffer: String = String::new();
    println!("Digite um número:");
    std::io::stdin().read_line(&mut buffer);
    let mut num1: i32 = buffer.trim().parse().expect("");

    let mut buffer: String = String::new();
    println!("Digite um número:");
    std::io::stdin().read_line(&mut buffer);
    let mut num2: i32 = buffer.trim().parse().expect("");

    let mut buffer: String = String::new();
    println!("Digite um número:");
    std::io::stdin().read_line(&mut buffer);
    let mut num3: i32 = buffer.trim().parse().expect("");
    
    if num1 == num2 && num2 == num3{
        println!("O triangulo é equilatero.");
    }else if num1 == num2 || num2 == num3 || num1 == num3{
        println!("O triangulo é isoceles.");
    }else{
        println!("O triangulo é escaleno.")
    }
    }
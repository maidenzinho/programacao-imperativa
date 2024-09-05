fn main(){
    let mut n: String = String::new();
    println!("Digite alguns números:");
    std::io::stdin().read_line(&mut n);
    let mut num: i32 = n.trim().parse().expect("");
    
    let mut n: String = String::new();
    println!("Digite alguns números:");
    std::io::stdin().read_line(&mut n);
    let mut num2: i32 = n.trim().parse().expect("");
    println!("Soma {}", num + num2);
    println!("Subtração {}", num - num2);
    println!("Multiplicação {}", num * num2);
    println!("Divisão {}", num / num2);
}
fn fatorial(a: i32) -> i32 {
    //função que é copiada
    let mut fat: i32 = 1;
    //para ir de 1 até o número coloque =
    //para ir ao inverso é só dar um .rev() (for i in 1..=a.rev(){})
    for i in 1..=a {
        fat *= i;
    }
    return fat;
}

fn main() {
    println!("{}", fatorial(5)) //copia
}

/*por recursão

fn fatorial_recursivo(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    fatorial_recursivo(n - 1) * n
}
fn main(){
let n i32 = 5;
}
*/

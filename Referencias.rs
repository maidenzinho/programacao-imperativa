fn troca(i: &mut i32, e: &mut i32) {
    let o = *i;
    *i = *e;
    *e = o;
}

fn main() {
    let mut x = 1;
    let mut y = 2;
    println!("Antes eram: {}, {}", x, y);
    troca(&mut x, &mut y);

    println!("Agora são: {}, {}", x, y)
}

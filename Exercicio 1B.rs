//escreva uma função que receba dois valores, troque os valores!
fn swap(x: &mut i32, y: &mut i32) {
    let temp:i32 = *x;
    *x = *y;
    *y = temp;

}

fn main(){
    let mut a:i32 = 10;
    let mut b:i32 = -7;

    println!("a = {a}, b = {b}");
    swap(&mut a, &mut b);
    println!("a = {a}, b = {b}");
}
fn media_aritmetica(vetor: [i32; 7]) -> f64 {
    let mut soma: f64 = 0.0;
    let n: f64 = vetor.len() as f64;

    for i in vetor {
        soma = soma + i as f64;
    }
    soma / n
}

fn main() {
    let v: [i32; 7] = [10, 0, 10, 0, 10, 0, 10];
    let media: f64 = media_aritmetica(v);

    println!("Média = {media:.2}")
}

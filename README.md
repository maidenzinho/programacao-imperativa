# Programação Imperativa
### Exercícios e exemplos em Rust, programa usado: Rust Rover
### Exemplo de código em Rust (.rs)(cronometro)
```
use std::io::{self, Write};
use std::time::{Instant, Duration};
use std::thread::sleep;

fn main() {
    let mut input = String::new();
    
    println!("Cronômetro iniciado! Pressione Enter para parar.");

    let start_time = Instant::now(); // Marca o início do cronômetro

    loop {
        print!("Tempo decorrido: ");
        io::stdout().flush().unwrap(); // Garante que o texto seja impresso imediatamente

        let elapsed = start_time.elapsed(); // Calcula o tempo decorrido
        let seconds = elapsed.as_secs();
        let millis = elapsed.subsec_millis();
        
        // Formata o tempo decorrido
        println!("{:02}:{:02}.{:03}", seconds / 60, seconds % 60, millis);

        // Aguarda um pouco para não sobrecarregar a CPU
        sleep(Duration::from_millis(100));

        // Checa se o usuário pressionou Enter para parar
        if io::stdin().read_line(&mut input).is_ok() {
            break;
        }
    }

    println!("Cronômetro parado.");
}

```

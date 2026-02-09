use std::io::{self, BufRead, Write};

use crate::difficulty::Difficulty;

#[derive(Debug, Clone)]
pub struct PuzzleInput {
    pub title: String,
    pub difficulty: Difficulty,
    pub grid_size: usize,
    pub words: Vec<String>,
}

impl PuzzleInput {
    pub fn from_stdin() -> io::Result<Self> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        println!("🍕 Pizza Word Search Generator");
        println!("==============================\n");

        print!("Título do Caça-Palavras: ");
        stdout.flush()?;
        let title = read_line(&stdin)?;

        println!("\nDificuldade:");
        println!("  1. Fácil (horizontal e vertical)");
        println!("  2. Médio (horizontal, vertical e diagonal)");
        println!("  3. Difícil (todas as direções, incluindo reverso)");
        print!("Escolha (1-3): ");
        stdout.flush()?;
        let diff_choice = read_line(&stdin)?;
        let difficulty = match diff_choice.trim() {
            "1" => Difficulty::easy(),
            "2" => Difficulty::medium(),
            "3" => Difficulty::hard(),
            _ => {
                println!("Opção inválida, usando Médio.");
                Difficulty::medium()
            }
        };

        println!("\nTamanho do grid:");
        println!("  1. 12x12 (pequeno)");
        println!("  2. 15x15 (médio)");
        println!("  3. 20x20 (grande)");
        print!("Escolha (1-3): ");
        stdout.flush()?;
        let size_choice = read_line(&stdin)?;
        let grid_size = match size_choice.trim() {
            "1" => 12,
            "2" => 15,
            "3" => 20,
            _ => {
                println!("Opção inválida, usando 15x15.");
                15
            }
        };

        println!("\nDigite as palavras (uma por linha OU separadas por vírgula).");
        println!("Quando terminar, digite uma linha vazia ou 'FIM':");
        println!();

        let mut words = Vec::new();
        loop {
            print!("> ");
            stdout.flush()?;
            let line = read_line(&stdin)?;
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("FIM") {
                break;
            }

            if trimmed.contains(',') {
                for word in trimmed.split(',') {
                    let w = word.trim();
                    if !w.is_empty() {
                        words.push(w.to_string());
                    }
                }
            } else if !trimmed.is_empty() {
                words.push(trimmed.to_string());
            }
        }

        if words.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Nenhuma palavra fornecida",
            ));
        }

        println!("\n✅ {} palavras recebidas.", words.len());

        Ok(Self {
            title,
            difficulty,
            grid_size,
            words,
        })
    }
}

fn read_line(stdin: &io::Stdin) -> io::Result<String> {
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    Ok(line.trim_end_matches('\n').trim_end_matches('\r').to_string())
}

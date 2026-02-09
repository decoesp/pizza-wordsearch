mod difficulty;
mod direction;
mod filler;
mod generator;
mod grid;
mod input;
mod pdf;
mod word;

use generator::{Generator, GeneratorConfig};
use input::PuzzleInput;
use pdf::PdfGenerator;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let input = match PuzzleInput::from_stdin() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Erro ao ler entrada: {}", e);
            std::process::exit(1);
        }
    };

    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let config = GeneratorConfig::new(input.grid_size, input.difficulty.clone()).with_max_attempts(200);
    let generator = Generator::new(config);

    let words_refs: Vec<&str> = input.words.iter().map(|s| s.as_str()).collect();
    let mut rng = StdRng::seed_from_u64(seed);
    let result = generator.generate(&words_refs, &mut rng);

    println!("\n📊 Gerando caça-palavras...\n");
    println!("Grid ({}x{}):\n", input.grid_size, input.grid_size);
    print_grid(&result.grid);

    println!("\nPalavras encontradas ({}):", result.placed_words.len());
    for placement in &result.placed_words {
        println!("  ✓ {} @ ({}, {}) {:?}", placement.word.original, placement.row, placement.col, placement.direction);
    }

    if !result.discarded_words.is_empty() {
        println!("\nPalavras descartadas ({}):", result.discarded_words.len());
        for word in &result.discarded_words {
            println!("  ✗ {}", word.original);
        }
    }

    let safe_title: String = input.title.chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect::<String>()
        .replace(' ', "_")
        .to_lowercase();

    let pdf_dir = format!("pdf/{}", safe_title);
    if let Err(e) = fs::create_dir_all(&pdf_dir) {
        eprintln!("Erro ao criar diretório {}: {}", pdf_dir, e);
        std::process::exit(1);
    }

    let puzzle_filename = format!("{}/puzzle.pdf", pdf_dir);
    let answer_filename = format!("{}/gabarito.pdf", pdf_dir);
    let pdf_gen = PdfGenerator::new(&input.title);

    println!("\n📄 Gerando PDFs em {}/ ...", pdf_dir);

    match pdf_gen.generate_puzzle(&result.grid, &input.words, &puzzle_filename) {
        Ok(_) => println!("  ✓ Puzzle: {}", puzzle_filename),
        Err(e) => eprintln!("  ✗ Erro ao gerar puzzle: {}", e),
    }

    match pdf_gen.generate_answer_key(&result.grid, &result.placed_words, &answer_filename) {
        Ok(_) => println!("  ✓ Gabarito: {}", answer_filename),
        Err(e) => eprintln!("  ✗ Erro ao gerar gabarito: {}", e),
    }

    println!("\n✅ Geração concluída!");
}

fn print_grid(grid: &grid::Grid) {
    let border = "─".repeat(grid.size * 2 + 1);
    println!("┌{}┐", border);
    for row in &grid.cells {
        print!("│ ");
        for cell in row {
            print!("{} ", cell.unwrap_or('.'));
        }
        println!("│");
    }
    println!("└{}┘", border);
}

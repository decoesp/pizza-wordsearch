# 🍕 Pizza Word Search Generator

Um gerador de caça-palavras determinístico escrito em Rust, usando algoritmos clássicos sem IA.

## ✨ Funcionalidades

- **Entrada interativa** — Título, dificuldade, tamanho do grid e lista de palavras
- **Palavras flexíveis** — Aceita uma por linha ou separadas por vírgula
- **3 níveis de dificuldade**:
  - **Fácil**: horizontal e vertical
  - **Médio**: horizontal, vertical e diagonal
  - **Difícil**: todas as direções, incluindo reverso
- **3 tamanhos de grid**: 12x12, 15x15, 20x20
- **Geração de PDF** — Puzzle para imprimir + gabarito com respostas destacadas
- **Preenchimento inteligente** — Usa frequência de letras do português brasileiro
- **Algoritmo determinístico** — Resultados reproduzíveis com seed

## 🚀 Instalação

```bash
git clone https://github.com/decoesp/pizza-wordsearch.git
cd pizza-wordsearch
cargo build --release
```

## 📖 Uso

```bash
cargo run
```

### Exemplo de execução

```text
🍕 Pizza Word Search Generator
==============================

Título do Caça-Palavras: Pizza Sabores

Dificuldade:
  1. Fácil (horizontal e vertical)
  2. Médio (horizontal, vertical e diagonal)
  3. Difícil (todas as direções, incluindo reverso)
Escolha (1-3): 2

Tamanho do grid:
  1. 12x12 (pequeno)
  2. 15x15 (médio)
  3. 20x20 (grande)
Escolha (1-3): 2

Digite as palavras (uma por linha OU separadas por vírgula).
Quando terminar, digite uma linha vazia ou 'FIM':

> pizza, queijo, mussarela, calabresa, frango, forno, lenha, catupiry
>

✅ 8 palavras recebidas.
```

### Saída

Os PDFs são gerados em `pdf/{tema}/`:

```text
pdf/
└── pizza_sabores/
    ├── puzzle.pdf      # Caça-palavras para imprimir
    └── gabarito.pdf    # Gabarito com palavras destacadas
```

## 🏗️ Estrutura do Projeto

```text
pizza-wordsearch/
├── Cargo.toml
└── src/
    ├── main.rs         # Ponto de entrada CLI
    ├── input.rs        # Entrada interativa do usuário
    ├── generator.rs    # Algoritmo principal de geração
    ├── grid.rs         # Estrutura do grid e posicionamento
    ├── word.rs         # Normalização de palavras
    ├── direction.rs    # Direções de posicionamento
    ├── difficulty.rs   # Configuração de dificuldade
    ├── filler.rs       # Preenchimento com frequência PT-BR
    └── pdf.rs          # Geração de PDFs
```

## 🔧 Como Funciona

1. **Normalização** — Palavras são convertidas para maiúsculas, sem acentos
2. **Ordenação** — Palavras maiores são posicionadas primeiro (mais difíceis de encaixar)
3. **Posicionamento** — Para cada palavra, tenta N posições/direções aleatórias
4. **Validação** — Verifica bounds e permite sobreposição apenas de letras iguais
5. **Preenchimento** — Células vazias recebem letras baseadas na frequência do português

### Frequência de Letras (PT-BR)

```text
A E O S R I D M N T C U V L P G Q B F H X J Z Y W K
```

Letras mais à esquerda aparecem com maior probabilidade no preenchimento.

## 📦 Dependências

- `rand` — Geração de números aleatórios
- `unicode-normalization` — Remoção de acentos
- `printpdf` — Geração de PDFs

## 📄 Licença

MIT

## 🤝 Contribuindo

Contribuições são bem-vindas! Abra uma issue ou pull request.

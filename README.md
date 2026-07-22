*Read this in: [English](#bestls--a-simple-ls-replacement-written-in-rust) | [Português](#bestls--um-substituto-simples-para-o-ls-escrito-em-rust)*

---

# "bestls" — A simple `ls` replacement written in Rust

A command line tool that lists directory contents, started as a learning project following a YouTube tutorial and evolving with my own features and experiments.

> **Study project** — This is a personal learning project and is not intended for production use.
>
> **On the use of AI** — All code in this project was written by hand. AI was used as a learning aid — for explanations, ideas, and concepts — but not to generate or produce any of the code directly.

## About

This project is a reimplementation of the Unix `ls` command, written in Rust. It started as a hands-on way to practice core Rust concepts, but is being expanded with my own ideas beyond the original tutorial.

## Usage

```bash
# List the current directory
bls

# List a specific directory
bls ./src
```

<!-- ## Building from source
Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.
```bash
git clone https://github.com/your-username/rls.git
cd rls
cargo build --release
``` -->

<!-- The compiled binary will be at `./target/release/rls`. -->

## What I learned

This project has been a hands-on way to practice:

- Ownership and borrowing
- Iterators and closures
- Structs and enums
- Error handling with `Result` and `Option`
- Reading the filesystem with `std::fs`
- Parsing command line arguments

## Planned / In progress

[]

## Credits

Initially inspired by a [YouTube tutorial](https://youtu.be/5UA9UWWAagc?si=FGU-b4MnHgUK3zDb). The project has since been extended with my own experiments and ideas.

## License

This project is for educational purposes and has no formal license.

---

# "bestls" — Um substituto simples para o `ls` escrito em Rust

Uma ferramenta de linha de comando que lista o conteúdo de diretórios, iniciada como projeto de estudo seguindo um tutorial no YouTube e sendo expandida com minhas próprias funcionalidades e experimentos.

> **Projeto de estudo** — Este é um projeto pessoal de aprendizado e não é indicado para uso em produção.
>
> **Sobre o uso de IA** — Todo o código deste projeto foi escrito à mão. IA foi utilizada como auxílio no aprendizado — para explicações, ideias e conceitos — mas não para gerar ou produzir nenhum código diretamente.

## Sobre

Este projeto é uma reimplementação do comando Unix `ls`, escrito em Rust. Começou como uma forma prática de exercitar conceitos fundamentais da linguagem, mas está sendo expandido com ideias próprias além do tutorial original.

## Uso

```bash
# Listar o diretório atual
bls

# Listar um diretório específico
bls ./src
```

<!-- ## Compilando o projeto
Certifique-se de ter o [Rust e o Cargo](https://www.rust-lang.org/tools/install) instalados.
```bash
git clone https://github.com/seu-usuario/rls.git
cd rls
cargo build --release
``` -->

<!-- O binário compilado estará em `./target/release/rls`. -->

## O que aprendi

Este projeto tem sido uma forma prática de exercitar:

- Ownership e borrowing
- Iteradores e closures
- Structs e enums
- Tratamento de erros com `Result` e `Option`
- Leitura do sistema de arquivos com `std::fs`
- Parsing de argumentos de linha de comando

## Planejado / Em andamento

Coisas que quero explorar e adicionar por conta própria:
... WIP

## Créditos

Inicialmente inspirado por um [tutorial no YouTube](https://youtu.be/5UA9UWWAagc?si=FGU-b4MnHgUK3zDb). O projeto vem sendo expandido com meus próprios experimentos e ideias.

## Licença

Este projeto tem fins educacionais e não possui uma licença formal.

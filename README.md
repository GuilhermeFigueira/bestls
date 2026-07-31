*Read this in: [English](#bestls--a-simple-ls-replacement-written-in-rust) | [Português](#bestls--um-substituto-simples-para-o-ls-escrito-em-rust)*

---

# "bestls" — A simple `ls` replacement written in Rust

A command line tool that lists directory contents, started as a learning project following a YouTube tutorial and evolving with my own features and experiments.

> **Study project** — This is a personal learning project and is not intended for production use.
>
> **On the use of AI** — All code in this project was written by hand. AI was used as a learning aid — for explanations, ideas, and concepts — but not to generate or produce any of the code directly.

## About

This project is a reimplementation of the Unix `ls` command, written in Rust and targeting Windows. It started as a hands-on way to practice core Rust concepts, but is being expanded with my own ideas beyond the original tutorial.

## Installation

1. Go to the [Releases page](https://github.com/GuilhermeFigueira/bestls/releases) and download `bls.exe` from the latest release.
2. Move `bls.exe` into a folder that's already on your `PATH` (or create a new folder, put it there, and add that folder to your `PATH`).
3. Open a new terminal and run `bls` from any directory.

## Usage

```bash
# List the current directory
bls

# List a specific directory
bls ./src

# Show hidden files for this call only
bls -a

# Output the listing as JSON
bls -j
```

### Settings

`bestls` keeps a persistent config file (created automatically on first run) with your display preferences: whether hidden files and folder sizes are shown by default, and how long file names are allowed to be before they're truncated.

```bash
# Print the current settings
bls settings

# Toggle whether hidden files are shown by default
bls toggle -A

# Toggle whether folder sizes are calculated and shown by default
bls toggle -F

# Both at once
bls toggle -A -F

# Change (or check) the max file/folder name length before truncation
bls file-name-length 30
bls file-name-length
```

## Building from source

Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

```bash
git clone https://github.com/GuilhermeFigueira/bestls.git
cd bestls
cargo build --release
```

The compiled binary will be at `./target/release/bls.exe`.

## What I learned

This project has been a hands-on way to practice:

- Ownership and borrowing
- Iterators and closures
- Structs and enums
- Error handling with `Result` and `Option`, and how `anyhow` chains context across error layers
- Reading the filesystem with `std::fs`
- Parsing command line arguments and subcommands
- Reading and persisting configuration files
- Working with platform-specific APIs (Windows file attributes)
- Module organization and visibility scoping (`pub(crate)`, `pub(super)`)
- Weighing dependency footprint vs. rolling your own implementation
- Separating a field's internal representation from how it's displayed (table) or serialized (JSON)
- Setting up a release build profile and an automated GitHub Actions release workflow

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

Este projeto é uma reimplementação do comando Unix `ls`, escrito em Rust e voltado para o Windows. Começou como uma forma prática de exercitar conceitos fundamentais da linguagem, mas está sendo expandido com ideias próprias além do tutorial original.

## Instalação

1. Acesse a [página de Releases](https://github.com/GuilhermeFigueira/bestls/releases) e baixe o `bls.exe` da versão mais recente.
2. Mova o `bls.exe` para uma pasta que já esteja no seu `PATH` (ou crie uma pasta nova, coloque o arquivo lá e adicione essa pasta ao `PATH`).
3. Abra um novo terminal e rode `bls` a partir de qualquer diretório.

## Uso

```bash
# Listar o diretório atual
bls

# Listar um diretório específico
bls ./src

# Mostrar arquivos ocultos apenas nessa chamada
bls -a

# Exibir a listagem em formato JSON
bls -j
```

### Configurações

O `bestls` mantém um arquivo de configuração persistente (criado automaticamente na primeira execução) com suas preferências de exibição: se arquivos ocultos e tamanhos de pasta são mostrados por padrão, e qual o tamanho máximo dos nomes de arquivo antes de serem truncados.

```bash
# Exibir as configurações atuais
bls settings

# Alternar se arquivos ocultos são mostrados por padrão
bls toggle -A

# Alternar se o tamanho das pastas é calculado e exibido por padrão
bls toggle -F

# Os dois de uma vez
bls toggle -A -F

# Alterar (ou consultar) o tamanho máximo de nome de arquivo/pasta antes do truncamento
bls file-name-length 30
bls file-name-length
```

## Compilando o projeto

Certifique-se de ter o [Rust e o Cargo](https://www.rust-lang.org/tools/install) instalados.

```bash
git clone https://github.com/GuilhermeFigueira/bestls.git
cd bestls
cargo build --release
```

O binário compilado estará em `./target/release/bls.exe`.

## O que aprendi

Este projeto tem sido uma forma prática de exercitar:

- Ownership e borrowing
- Iteradores e closures
- Structs e enums
- Tratamento de erros com `Result` e `Option`, e como o `anyhow` encadeia contexto entre camadas de erro
- Leitura do sistema de arquivos com `std::fs`
- Parsing de argumentos de linha de comando e subcomandos
- Leitura e persistência de arquivos de configuração
- Uso de APIs específicas de plataforma (atributos de arquivo do Windows)
- Organização de módulos e escopo de visibilidade (`pub(crate)`, `pub(super)`)
- Avaliar o peso de uma dependência versus implementar algo por conta própria
- Separar a representação interna de um campo de como ele é exibido (tabela) ou serializado (JSON)
- Configuração de um perfil de build de release e automação de releases com GitHub Actions

## Créditos

Inicialmente inspirado por um [tutorial no YouTube](https://youtu.be/5UA9UWWAagc?si=FGU-b4MnHgUK3zDb). O projeto vem sendo expandido com meus próprios experimentos e ideias.

## Licença

Este projeto tem fins educacionais e não possui uma licença formal.

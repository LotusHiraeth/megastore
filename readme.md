
# ConectaStore — Sistema de Recomendação de Produtos

## Sobre o projeto

O ConectaStore é um protótipo de sistema de recomendação desenvolvido em Rust para a empresa fictícia MegaStore.

O sistema utiliza grafos para representar relações entre produtos e o algoritmo BFS para encontrar produtos relacionados.

## Tecnologias e estruturas utilizadas

- Rust
- Cargo
- HashMap
- HashSet
- Vec
- VecDeque
- BFS (Busca em Largura)

## Funcionalidades

- Cadastro de produtos em memória
- Consulta de produtos pelo identificador
- Criação de conexões entre produtos
- Recomendações utilizando BFS
- Prevenção de recomendações duplicadas
- Testes unitários e de integração
- Medição de desempenho

## Estrutura do projeto

```text
megastore/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── produto.rs
│   ├── grafo.rs
│   └── recomendacao.rs
├── tests/
│   └── teste_grafo.rs
├── Cargo.toml
└── README.md
```

## Como executar

É necessário instalar Rust e Cargo.

Abra um terminal na pasta principal do projeto.

Execute:

```bash
cargo run
```

O programa exibirá o produto consultado, suas recomendações e os testes de desempenho.

## Exemplo de uso

Produto consultado: Notebook

Recomendações:

- Mouse Gamer
- Teclado Mecânico
- Headset
- Cadeira Gamer

Os produtos e suas conexões estão cadastrados no código do protótipo.

## Testes automatizados

Execute:

```bash
cargo test
```

Foram implementados:

- 1 teste unitário
- 3 testes de integração

Resultado obtido: 4 testes aprovados e 0 falhas.

## Arquitetura

O sistema é dividido em módulos:

- `produto.rs`: representação dos produtos
- `grafo.rs`: armazenamento das conexões e BFS
- `recomendacao.rs`: apresentação das recomendações
- `main.rs`: execução principal e medições
- `lib.rs`: exportação dos módulos
- `tests/`: testes de integração

O grafo é não direcionado, não ponderado e representado por listas de adjacência.

## Desempenho

Medições realizadas em uma execução no perfil de desenvolvimento:

| Produtos | Recomendações | Tempo |
|---|---|---|
| 100 | 99 | 118,8 µs |
| 1.000 | 999 | 1,2599 ms |
| 10.000 | 9.999 | 13,5844 ms |

Os resultados são preliminares e podem variar conforme o computador e as condições de execução.

## Vídeo de apresentação

Link do vídeo pitch: será adicionado após a gravação no Dia 3.

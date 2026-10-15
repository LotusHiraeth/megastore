use std::time::Instant;

use megastore::grafo::Grafo;
use megastore::produto::Produto;
use megastore::recomendacao::mostrar_recomendacoes;

fn main() {
    println!("=================================");
    println!("          CONECTASTORE");
    println!(" Sistema de Recomendacao");
    println!("=================================\n");

    let mut grafo = Grafo::novo();

    // Cadastro dos produtos
    grafo.adicionar_produto(
        Produto::novo(1, "Notebook", "Eletronicos")
    );

    grafo.adicionar_produto(
        Produto::novo(2, "Mouse Gamer", "Eletronicos")
    );

    grafo.adicionar_produto(
        Produto::novo(3, "Teclado Mecanico", "Eletronicos")
    );

    grafo.adicionar_produto(
        Produto::novo(4, "Headset", "Eletronicos")
    );

    grafo.adicionar_produto(
        Produto::novo(5, "Cadeira Gamer", "Moveis")
    );

    // Conexoes entre os produtos
    grafo.adicionar_conexao(1, 2);
    grafo.adicionar_conexao(1, 3);
    grafo.adicionar_conexao(2, 4);
    grafo.adicionar_conexao(3, 5);

    // Consulta
    println!("=== PRODUTO CONSULTADO ===");

    if let Some(produto) = grafo.buscar_produto(1) {
        println!(
            "ID: {} | {} | Categoria: {}",
            produto.id,
            produto.nome,
            produto.categoria
        );
    }

    // Medicao do algoritmo
    let inicio = Instant::now();

    mostrar_recomendacoes(&grafo, 1);

    let duracao = inicio.elapsed();

    println!("\nTempo da recomendacao: {:?}", duracao);
    
    println!("\n=== TESTE DE DESEMPENHO ===");

teste_desempenho(100);
teste_desempenho(1_000);
teste_desempenho(10_000);
}

fn teste_desempenho(quantidade: u32) {
    let mut grafo = Grafo::novo();

    for id in 1..=quantidade {
        grafo.adicionar_produto(
            Produto::novo(
                id,
                &format!("Produto {}", id),
                "Categoria Teste",
            )
        );
    }

    for id in 1..quantidade {
        grafo.adicionar_conexao(id, id + 1);
    }

    let inicio = Instant::now();

    let recomendacoes = grafo.recomendar_bfs(1);

    let duracao = inicio.elapsed();

    println!(
        "{} produtos -> {} recomendacoes -> {:?}",
        quantidade,
        recomendacoes.len(),
        duracao
    );
}
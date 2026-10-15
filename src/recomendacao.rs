// Este módulo utiliza o grafo para obter
// e mostrar os produtos recomendados.

use crate::grafo::Grafo;

pub fn mostrar_recomendacoes(grafo: &Grafo, produto_id: u32) {
    let recomendacoes = grafo.recomendar_bfs(produto_id);

    println!("\n=== PRODUTOS RECOMENDADOS ===");

    if recomendacoes.is_empty() {
        println!("Nenhuma recomendacao encontrada.");
        return;
    }

    for produto in recomendacoes {
        println!(
            "ID: {} | {} | Categoria: {}",
            produto.id,
            produto.nome,
            produto.categoria
        );
    }
}
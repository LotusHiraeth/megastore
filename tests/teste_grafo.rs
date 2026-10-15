use megastore::grafo::Grafo;
use megastore::produto::Produto;

#[test]
fn deve_cadastrar_e_buscar_produto() {
    let mut grafo = Grafo::novo();

    grafo.adicionar_produto(
        Produto::novo(1, "Notebook", "Eletronicos")
    );

    let produto = grafo.buscar_produto(1);

    assert!(produto.is_some());
    assert_eq!(produto.unwrap().nome, "Notebook");
}

#[test]
fn deve_recomendar_produtos() {
    let mut grafo = Grafo::novo();

    grafo.adicionar_produto(
        Produto::novo(1, "Notebook", "Eletronicos")
    );

    grafo.adicionar_produto(
        Produto::novo(2, "Mouse", "Eletronicos")
    );

    grafo.adicionar_produto(
        Produto::novo(3, "Teclado", "Eletronicos")
    );

    grafo.adicionar_conexao(1, 2);
    grafo.adicionar_conexao(1, 3);

    let recomendacoes = grafo.recomendar_bfs(1);

    assert_eq!(recomendacoes.len(), 2);
}

#[test]
fn nao_deve_duplicar_recomendacao() {
    let mut grafo = Grafo::novo();

    grafo.adicionar_produto(
        Produto::novo(1, "Notebook", "Eletronicos")
    );

    grafo.adicionar_produto(
        Produto::novo(2, "Mouse", "Eletronicos")
    );

    grafo.adicionar_conexao(1, 2);

    // Tenta cadastrar a mesma conexao novamente
    grafo.adicionar_conexao(1, 2);

    let recomendacoes = grafo.recomendar_bfs(1);

    assert_eq!(recomendacoes.len(), 1);
}
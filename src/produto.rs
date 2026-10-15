// Estrutura responsável por representar um produto da MegaStore.
//
// struct:
// Cria uma estrutura que agrupa informações relacionadas.
//
// pub:
// Permite que outras partes do projeto utilizem a estrutura.
//
// impl:
// Define funções relacionadas ao Produto.
//
// String:
// Armazena textos, como nome e categoria.

#[derive(Clone, Debug)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
}

impl Produto {
    pub fn novo(id: u32, nome: &str, categoria: &str) -> Self {
        Produto {
            id,
            nome: nome.to_string(),
            categoria: categoria.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deve_criar_produto_corretamente() {
        let produto = Produto::novo(
            1,
            "Notebook",
            "Eletronicos"
        );

        assert_eq!(produto.id, 1);
        assert_eq!(produto.nome, "Notebook");
        assert_eq!(produto.categoria, "Eletronicos");
    }
}
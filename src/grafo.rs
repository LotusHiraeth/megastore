// O grafo representa as relações entre os produtos.
//
// HashMap:
// Permite localizar rapidamente produtos pelo ID.
//
// HashSet:
// Guarda os produtos já visitados e evita duplicações.
//
// VecDeque:
// Funciona como a fila utilizada pelo algoritmo BFS.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::produto::Produto;

pub struct Grafo {
    produtos: HashMap<u32, Produto>,
    conexoes: HashMap<u32, Vec<u32>>,
}

impl Grafo {
    pub fn novo() -> Self {
        Grafo {
            produtos: HashMap::new(),
            conexoes: HashMap::new(),
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        self.conexoes.entry(produto.id).or_default();
        self.produtos.insert(produto.id, produto);
    }

    pub fn buscar_produto(&self, id: u32) -> Option<&Produto> {
        self.produtos.get(&id)
    }

    pub fn adicionar_conexao(&mut self, produto1: u32, produto2: u32) {
        if self.produtos.contains_key(&produto1)
            && self.produtos.contains_key(&produto2)
        {
            if !self.conexoes.get(&produto1).unwrap().contains(&produto2) {
                self.conexoes
                    .entry(produto1)
                    .or_default()
                    .push(produto2);
            }

            if !self.conexoes.get(&produto2).unwrap().contains(&produto1) {
                self.conexoes
                    .entry(produto2)
                    .or_default()
                    .push(produto1);
            }
        }
    }

    pub fn recomendar_bfs(&self, inicio: u32) -> Vec<&Produto> {
        let mut visitados = HashSet::new();
        let mut fila = VecDeque::new();
        let mut recomendacoes = Vec::new();

        visitados.insert(inicio);
        fila.push_back(inicio);

        while let Some(atual) = fila.pop_front() {
            if let Some(vizinhos) = self.conexoes.get(&atual) {
                for vizinho in vizinhos {
                    if visitados.insert(*vizinho) {
                        fila.push_back(*vizinho);

                        if let Some(produto) = self.produtos.get(vizinho) {
                            recomendacoes.push(produto);
                        }
                    }
                }
            }
        }

        recomendacoes
    }
}
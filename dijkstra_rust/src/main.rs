use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Estado {
    costo: u32,
    nodo: char,
}

impl Ord for Estado {
    fn cmp(&self, other: &Self) -> Ordering {
        other.costo.cmp(&self.costo)
    }
}

impl PartialOrd for Estado {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    grafo: &HashMap<char, Vec<(char, u32)>>,
    inicio: char,
) -> HashMap<char, u32> {

    let mut distancias = HashMap::new();
    let mut heap = BinaryHeap::new();

    for nodo in grafo.keys() {
        distancias.insert(*nodo, u32::MAX);
    }

    if !grafo.contains_key(&inicio) {
        return distancias;
    }

    distancias.insert(inicio, 0);

    heap.push(Estado {
        costo: 0,
        nodo: inicio,
    });

    while let Some(Estado { costo, nodo }) = heap.pop() {

        if costo > *distancias.get(&nodo).unwrap() {
            continue;
        }

        if let Some(vecinos) = grafo.get(&nodo) {

            for &(vecino, peso) in vecinos {

                let siguiente = Estado {
                    costo: costo + peso,
                    nodo: vecino,
                };

                if siguiente.costo
                    < *distancias.get(&vecino).unwrap_or(&u32::MAX)
                {
                    distancias.insert(vecino, siguiente.costo);

                    heap.push(siguiente);
                }
            }
        }
    }

    distancias
}

fn mostrar_resultados(
    resultado: HashMap<char, u32>
) {
    for (nodo, distancia) in resultado {

        if distancia == u32::MAX {
            println!("{} -> Inalcanzable", nodo);
        } else {
            println!("{} -> {}", nodo, distancia);
        }
    }
}

fn main() {

    println!("====================");
    println!("CASO DE PRUEBA 1");
    println!("====================");

    let mut grafo1 = HashMap::new();

    grafo1.insert('A', vec![('B', 4), ('C', 2)]);
    grafo1.insert('B', vec![('D', 5)]);
    grafo1.insert('C', vec![('B', 1), ('D', 8)]);
    grafo1.insert('D', vec![]);

    mostrar_resultados(
        dijkstra(&grafo1, 'A')
    );

    println!("\n====================");
    println!("CASO DE PRUEBA 2");
    println!("====================");

    let mut grafo2 = HashMap::new();

    grafo2.insert('A', vec![('B', 3)]);
    grafo2.insert('B', vec![]);
    grafo2.insert('C', vec![]);

    mostrar_resultados(
        dijkstra(&grafo2, 'A')
    );

    println!("\n====================");
    println!("CASO DE PRUEBA 3");
    println!("====================");

    let mut grafo3 = HashMap::new();

    grafo3.insert('A', vec![('B', 1), ('C', 4)]);
    grafo3.insert('B', vec![('D', 2), ('E', 5)]);
    grafo3.insert('C', vec![('E', 1)]);
    grafo3.insert('D', vec![('F', 3)]);
    grafo3.insert('E', vec![('F', 2)]);
    grafo3.insert('F', vec![]);

    mostrar_resultados(
        dijkstra(&grafo3, 'A')
    );

    println!("\n====================");
    println!("CASO BORDE");
    println!("====================");

    let grafo_vacio:
        HashMap<char, Vec<(char, u32)>> =
        HashMap::new();

    mostrar_resultados(
        dijkstra(&grafo_vacio, 'A')
    );
}
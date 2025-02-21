fn main() {
    // Exemplo Básico de Array
    let array: [i32, 3] = [1, 2, 3];

    // Exemplo Básico de Vetor
    let vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Exemplo Básico de Tupla
    let tupla: (i32, f64, bool) = (1, 2.0, true);

    // Exemplo Básico de Mapa
    let mapa: HashMap<String, i32> = HashMap::new();
    mapa.insert("key1".to_string(), 1);
    mapa.insert("key2".to_string(), 2);

    // Exemplo Básico de Set
    let set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
}

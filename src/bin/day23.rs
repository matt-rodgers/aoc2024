use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn main() {
    let input = include_str!("../../inputs/23.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

fn run(input: &str) -> (u64, String) {
    let pairs: Vec<(&str, &str)> = input
        .trim()
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect();

    let graph = build_graph(pairs);

    let mut sets_of_three: HashSet<Vec<&str>> = HashSet::new();
    for (key, val) in graph.iter() {
        for combo in val.iter().combinations(2) {
            let a = combo[0];
            let b = combo[1];
            if let Some(ga) = graph.get(a) {
                if ga.contains(b) {
                    let mut set = Vec::new();
                    set.push(*key);
                    set.push(a);
                    set.push(b);
                    set.sort(); // Must be sorted to avoid duplicates in HashSet
                    sets_of_three.insert(set);
                }
            }
        }
    }

    let pt1 = sets_of_three
        .iter()
        .filter(|s| s.iter().any(|elem| elem.starts_with('t')))
        .count();

    let current_clique = HashSet::new();
    let mut candidates = graph.keys().map(|k| *k).collect();
    let mut excluded = HashSet::new();
    let mut cliques = Vec::new();

    bron_kerbosch(
        &current_clique,
        &mut candidates,
        &mut excluded,
        &graph,
        &mut cliques,
    );

    let largest_clique = cliques
        .iter_mut()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();
    largest_clique.sort();

    (pt1 as u64, format!("{}", largest_clique.join(",")))
}

fn build_graph<'a>(pairs: Vec<(&'a str, &'a str)>) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut graph = HashMap::new();

    for (a, b) in pairs {
        let ga = graph.entry(a).or_insert_with(HashSet::new);
        ga.insert(b);

        let gb = graph.entry(b).or_insert_with(HashSet::new);
        gb.insert(a);
    }

    graph
}

fn bron_kerbosch<'a>(
    current_clique: &HashSet<&'a str>,
    candidates: &mut HashSet<&'a str>,
    excluded: &mut HashSet<&'a str>,
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    cliques: &mut Vec<Vec<&'a str>>,
) {
    // Check recursion end condition
    if candidates.is_empty() && excluded.is_empty() {
        if current_clique.len() > 2 {
            let mut clique: Vec<&str> = current_clique.iter().cloned().collect();
            clique.sort();
            cliques.push(clique);
        }
    }

    // Select a pivot vertex that has the maximum number of neighbors
    let pivot = candidates
        .union(excluded)
        .max_by_key(|v| graph.get(*v).map_or(0, |neigbors| neigbors.len()));

    if let Some(pivot_vertex) = pivot {
        let neighbors = graph.get(pivot_vertex).unwrap();
        let cc = candidates.clone();
        let reduced_candidates = cc.difference(&neighbors);

        for vertex in reduced_candidates {
            let mut new_current_clique = current_clique.clone();
            new_current_clique.insert(vertex);

            let vertex_neighbors = graph.get(vertex).unwrap();
            let mut new_candidates = candidates
                .intersection(vertex_neighbors)
                .map(|s| *s)
                .collect();

            let mut new_excluded = excluded
                .intersection(vertex_neighbors)
                .map(|s| *s)
                .collect();

            bron_kerbosch(
                &new_current_clique,
                &mut new_candidates,
                &mut new_excluded,
                graph,
                cliques,
            );

            candidates.remove(vertex);
            excluded.insert(vertex);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/23.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 7);
        assert_eq!(pt2, "co,de,ka,ta");
    }

    #[test]
    fn test_bron_kerbosch() {
        let pairs = vec![
            ("a", "b"),
            ("a", "c"),
            ("b", "c"),
            ("d", "e"),
            ("d", "f"),
            ("e", "f"),
        ];

        let graph = build_graph(pairs);
        let current_clique = HashSet::new();
        let mut candidates = graph.keys().map(|k| *k).collect();
        let mut excluded = HashSet::new();
        let mut cliques = Vec::new();

        bron_kerbosch(
            &current_clique,
            &mut candidates,
            &mut excluded,
            &graph,
            &mut cliques,
        );

        for clique in cliques.iter_mut() {
            clique.sort();
        }
        cliques.sort();

        assert_eq!(cliques, vec![vec!["a", "b", "c"], vec!["d", "e", "f"]]);
    }
}

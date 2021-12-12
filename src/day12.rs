use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_list(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("Could not read file");
    BufReader::new(f).lines().map(|l| l.expect("Err")).collect()
}

#[derive(Debug)]
struct CaveGraph {
    // Using String here rather than &str to avoid caring about lifetimes.
    adj_map: HashMap<String, HashSet<String>>,
}

impl CaveGraph {
    fn new(filename: &str) -> Self {
        let mut adj_map = HashMap::new();
        for line in read_list(filename) {
            // There must be a better way of 'splitting to a pair' (without
            // resorting to regex...)
            let mut line_parts = line.split('-');
            let from = line_parts.next().unwrap();
            let to = line_parts.next().unwrap();

            // since this is not a directed graph, add both directions
            let from_set = adj_map.entry(from.to_string()).or_insert_with(HashSet::new);
            from_set.insert(to.to_string());

            let to_set = adj_map.entry(to.to_string()).or_insert_with(HashSet::new);
            to_set.insert(from.to_string());
        }
        Self { adj_map }
    }

    fn count_paths(&self) {
        let mut visited = HashSet::new();

        println!("{}", self.dfs("start".to_string(), &mut visited));
    }

    fn dfs(&self, node: String, visited: &mut HashSet<String>) -> i32 {
        let mut total = 0;
        if node == "end" {
            return 1;
        }
        if node.chars().all(|x| x.is_lowercase()) {
            visited.insert(node.to_string());
        }
        for cave in self
            .adj_map
            .get(&node)
            .expect("node missing from graph")
            .iter()
        {
            if visited.contains(cave) {
                continue;
            }
            total += self.dfs(cave.to_string(), &mut visited.clone());
        }

        total
    }

    // Re-implemented for part 2 to accumulate paths in a set rather than
    // just count them. Slower, but allows a simple / naive solution to
    // just check for each possible value of 'cave allowed twice'.
    fn count_paths_alt(&self) {
        let mut visited = HashMap::new();
        let mut paths = HashSet::new();

        for allow_twice in self.adj_map.keys() {
            if allow_twice.chars().all(|x| x.is_lowercase()) {
                if allow_twice == "start" || allow_twice == "end" {
                    continue;
                }
                self.dfs_alt(
                    "start".to_string(),
                    &mut visited,
                    &mut paths,
                    allow_twice.to_string(),
                    "".to_string(),
                );
            }
        }
        println!("{}", paths.len());
    }

    fn dfs_alt(
        &self,
        node: String,
        visited: &mut HashMap<String, i32>,
        paths: &mut HashSet<String>,
        allow_twice: String,
        path: String,
    ) {
        if node == "end" {
            //println!("Path: {}", &path);
            paths.insert(path.clone());
        }
        if node.chars().all(|x| x.is_lowercase()) {
            let visit_count = visited.entry(node.clone()).or_insert(0);
            *visit_count += 1;
        }
        let path = path + "," + &node;
        for cave in self
            .adj_map
            .get(&node)
            .expect("node missing from graph")
            .iter()
        {
            if let Some(visit_count) = visited.get(&cave.to_string()) {
                if cave == &allow_twice {
                    // Rust quibble (again) - need for either *x or &1
                    // There's probably a reason this can't be implicit for i32
                    // (but there is implicit eq for various str/String things)
                    if *visit_count > 1 {
                        continue;
                    }
                } else if *visit_count > 0 {
                    continue;
                }
            }
            self.dfs_alt(
                cave.to_string(),
                &mut visited.clone(),
                paths,
                allow_twice.clone(),
                path.clone(),
            );
        }
    }
}

pub fn step1() {
    let cg = CaveGraph::new("inputs/day12.txt");

    cg.count_paths();
}

pub fn step2() {
    let cg = CaveGraph::new("inputs/day12.txt");

    cg.count_paths_alt();
}

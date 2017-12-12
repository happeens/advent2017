use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::collections::HashSet;

fn main() {
    let mut input = File::open("input.txt").expect("input not found");
    let mut contents = String::new();
    input
        .read_to_string(&mut contents)
        .expect("could not read input to string");

    let programs = contents.split("\n")
        .filter(|it| !it.is_empty())
        .map(|it| Program::from_string(it))
        .collect::<Vec<Program>>();

    let program_zero = programs.iter()
        .find(|it| it.id == "0")
        .unwrap();

    let programs_in_group_zero =
        program_zero.find_all_connected_programs(&programs);

    let count = programs_in_group_zero.len();

    println!("number of programs in group zero: {}", count);

    // this is super slow but it's easy
    let mut group_hashes = HashSet::new();
    for it in programs.iter().cloned() {
        let all_connections =
            it.find_all_connected_programs(&programs);

        let mut list = all_connections.iter().cloned()
            .collect::<Vec<String>>();
        
        list.sort_by(|a, b| b.cmp(a));

        let hash = list.join(",");
        group_hashes.insert(hash);
    }

    println!("number of groups: {}", group_hashes.len());
}

#[derive(Debug, Clone)]
struct Program {
    id: String,
    direct_cons: Vec<String>,
}

impl Program {
    fn from_string(input: &str) -> Program {
        let parts = input.split("<->")
            .map(|it| it.trim())
            .filter(|it| !it.is_empty())
            .collect::<Vec<&str>>();

        assert!(parts.len() == 2);

        let id = String::from(parts[0]);

        let direct_cons = parts[1].split(",")
            .map(|it| it.trim())
            .filter(|it| !it.is_empty())
            .map(|it| it.to_owned())
            .collect::<Vec<String>>();

        Program { id, direct_cons }
    }

    fn find_all_connected_programs(
        &self,
        programs: &Vec<Program>
    ) -> HashSet<String> {
        let mut all_connected = HashSet::new();
        all_connected.insert(self.id.clone());

        let mut current_children: HashSet<String> =
            HashSet::from_iter(self.direct_cons.iter().cloned());

        loop {
            for it in &current_children {
                all_connected.insert(it.clone());
            }

            let children = programs.iter().cloned()
                .filter(|it| current_children.contains(&it.id))
                .flat_map(|it| it.direct_cons)
                .filter(|it| !all_connected.contains(it))
                .collect::<Vec<String>>();

            if children.is_empty() { break; }

            current_children = HashSet::from_iter(children);
        }

        all_connected
    }
}

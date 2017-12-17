extern crate advent2017;
use advent2017::file::Input;

use std::collections::HashMap;

fn main() {
    let entries = Input::read("day07").into_lines().iter()
        .map(|it| Entry::from_string(it))
        .collect::<Vec<Entry>>();

    let root_entry = find_root_entry(&entries);
    println!("root node name is {}", root_entry.name);

    find_wrong_weight(&root_entry, &entries);
}

fn find_wrong_weight(base: &Entry, entries: &Vec<Entry>) {
    let children = base.get_children(&entries);

    if children.is_empty() {
        println!("i have no children! and my weight is {}", base.weight);
        return;
    }

    let mut weight_map: HashMap<u32, (u32, Entry)> = HashMap::new();
    for it in &children {
        let weight = it.get_full_weight(&entries);

        if weight_map.contains_key(&weight) {
            let value: u32 = weight_map[&weight].0;
            weight_map.insert(weight, (value + 1, it.clone()));
        } else {
            weight_map.insert(weight, (1, it.clone()));
        }
    }

    println!("weight map: {:?}", weight_map);

    for (weight, it) in weight_map {
        if it.0 > 1 {
            continue;
        }

        println!("unbalanced child was {} with {}", it.1.name, weight);
        let unbalanced_child = it.1;
        find_wrong_weight(&unbalanced_child, &entries);
    }
}

fn find_root_entry(e: &Vec<Entry>) -> Entry {
    let child_entries = e.iter()
        .flat_map(|ref it| it.children.iter().cloned())
        .collect::<Vec<String>>();

    let not_child_entries = e.iter().cloned()
        .filter(|ref it| !child_entries.contains(&it.name))
        .collect::<Vec<Entry>>();

    if not_child_entries.len() != 1 {
        panic!("error! more or less than 1 parent found");
    }

    not_child_entries.first().unwrap().clone()
}

#[derive(Debug, Clone)]
struct Entry {
    name: String,
    weight: u32,
    children: Vec<String>,
}

impl Entry {
    fn from_string(s: &str) -> Entry {
        let parts: Vec<&str> = s.split("(").collect();
        let name = parts.first().unwrap().trim().to_owned();

        let parts: Vec<&str> = parts[1].split(")").collect();
        let weight = parts[0].parse::<u32>().unwrap();

        let parts: Vec<&str> = parts[1].split("->").collect();
        let children: Vec<String> = parts.last().unwrap().split(",")
            .map(|parent| parent.trim())
            .map(|parent| parent.to_owned())
            .filter(|parent| !parent.is_empty())
            .collect();

        Entry { name, weight, children }
    }

    fn get_children(&self, entries: &Vec<Entry>) -> Vec<Entry> {
        entries.iter().cloned()
            .filter(|ref it| self.children.contains(&it.name))
            .collect::<Vec<Entry>>()
    }

    fn get_full_weight(&self, entries: &Vec<Entry>) -> u32 {
        if self.children.is_empty() {
            return self.weight;
        }

        self.get_children(&entries).iter()
            .map(|ref it| it.get_full_weight(&entries))
            .fold(self.weight, |acc, it| acc + it)
    }
}

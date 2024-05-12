use std::collections::VecDeque;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Person {
    pub name: String,
    pub sex: char,
    pub date_of_birth: chrono::NaiveDate,
    // the field you should modify, if applicable
    pub teknonym: String,
    pub children: Vec<Person>,
}

pub fn teknonymize(family_tree: &mut Person) {
    define_teknonym(family_tree);
    for child in &mut family_tree.children {
        teknonymize(child);
    }
}

fn define_teknonym(family_tree: &mut Person) {
    let mut flat_family: Vec<(&Person, u32)> = Vec::new();
    let mut people_buffer: VecDeque<(&Person, u32)> = VecDeque::new();
    people_buffer.push_front((&family_tree, 0));

    while !people_buffer.is_empty() {
        if let Some(layered) = people_buffer.pop_front() {
            let person = layered.0;
            let layer = layered.1;
            let new_generation = traverse_children(person, layer);
            new_generation.iter().for_each(|p|people_buffer.push_back(*p));
            flat_family.push((person, layer));
        }
    }

    let (of_str, layer) = select_child(flat_family);

    // build teknonymy string
    if layer > 0 {
        family_tree.teknonym = add_generational_steps(layer) + &*parent(&family_tree) + " " + &*of_str;
    } else {
        // No children found, don't change teknonym
    }
}

fn select_child(flat_family_tree: Vec<(&Person, u32)>) -> (String, u32) {
    let mut out = ("of".to_string(), 0);
    let mut sorted_tree = flat_family_tree.clone();
    sorted_tree.sort_by_key(|&(_, layer)| std::cmp::Reverse(layer));
    let max_layer_tuple = sorted_tree.iter().next();

    if let Some(&(person, layer)) = max_layer_tuple {
        let max_layer_tuples: Vec<&(&Person, u32)> = flat_family_tree.iter()
            .filter(|&&(_, l)| l == layer)
            .collect();

        let (person, layer) = max_layer_tuples.iter().max_by_key(|(p, _)|p.date_of_birth).unwrap();
        out = ("of ".to_string() + &person.name, *layer);
    }
    out
}

fn traverse_children(person: &Person, layer: u32) -> Vec<(&Person, u32)> {
    person.children.iter().map(|c|(c, layer + 1)).collect()
}

fn parent(person: &Person) -> String {
    if person.sex.eq(&'f') { "mother".to_string() } else { "father".to_string()}
}

fn add_generational_steps(steps: u32) -> String {
    let mut out = String::from("");
    if steps > 1 {
        out = String::from("grand");
    }
    if steps > 2 {
        for _ in 2..steps {
            out = "great-".to_string() + &*out;
        }
    }
    out
}
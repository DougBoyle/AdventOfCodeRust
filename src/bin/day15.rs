use linked_hash_map::LinkedHashMap;

fn main() {
    let line = rust_aoc::read_input(15).next().unwrap();
    let total: usize = line.split(',').map(hash).sum();
    println!("Part 1: Total hashes {total}"); // 505379

    let mut boxes = Boxes::new();
    for action in line.split(',') {
        boxes.apply(action);
    }

    let power = boxes.get_total_power();

    println!("Part 2: Total power {power}"); // 263211
}

struct Boxes {
    boxes: Vec<LinkedHashMap<String, usize>>,
}

impl Boxes {
    fn new() -> Boxes {
        Boxes { boxes: (0..256).map(|_| LinkedHashMap::new()).collect() }
    }

    fn apply(&mut self, action: &str) {
        if let Some(idx) = action.find('-') {
            let label = &action[..idx];
            self.remove(label);
        } else if let Some(idx) = action.find('=') {
            let label = &action[..idx];
            let lens = &action[(idx + 1)..];
            self.update(label, lens);
        } else {
            panic!("Invalid action {action}");
        }
    }

    fn remove(&mut self, label: &str) {
        let hash = hash(label);
        self.boxes[hash].remove(label);
    }

    fn update(&mut self, label: &str, lens: &str) {
        let hash = hash(&label);
        let lens = lens.parse().unwrap();
        self.boxes[hash].entry(String::from(label)).and_modify(|stored_lens| *stored_lens = lens).or_insert(lens);
    }

    fn get_total_power(&self) -> usize {
        self.boxes.iter().enumerate().map(|(box_num, lenses)| {
            let box_num = box_num + 1;
            box_num * lenses.values().enumerate().map(|(slot, lens)| (slot + 1) * lens).sum::<usize>()
        }).sum()
    }
}

fn hash(s: &str) -> usize {
    let mut value = 0;
    for b in s.bytes() {
        value += b as usize;
        value *= 17;
        value %= 256;
    }
    value
}

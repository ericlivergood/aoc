use std::fmt::{Display, Formatter};

pub struct XHashMap {
    boxes: Vec<Vec<Lens>>
}

impl Display for XHashMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut n = 0;
        for b in &self.boxes {
            if b.len() > 0 {
                write!(f, "Box {n}: ").unwrap();
                for l in b {
                    write!(f, "{0} ", l).unwrap();
                }
                writeln!(f).unwrap();
            }
            n += 1;
        }

        writeln!(f)
    }
}


impl XHashMap {
    pub fn new() -> Self {
        let mut boxes = Vec::new();
        for _i in 0..256 {
            boxes.push(Vec::new())
        }
        Self {
            boxes
        }
    }

    pub fn calculate_focusing_power(&self) -> i32 {
        let mut sum = 0;
        let mut box_num = 1;
        for b in &self.boxes {
            let mut slot = 1;
            for lens in b {
                sum += box_num * slot * lens.power;
                slot += 1;
            }
            box_num += 1;
        }
        sum
    }

    fn remove(&mut self, label: &str) {
        let h = hash(label.to_string()) as usize;
        let b = &mut self.boxes[h];

        let i = b.iter().position(|x| x.label == label);

        match i {
            None => {}
            Some(x) => {
                b.remove(x);
            }
        }
    }

    fn add(&mut self, lens: Lens) {
        let h = hash(lens.label.to_string()) as usize;
        let b = &mut self.boxes[h];

        let i = b.iter().position(|x| x.label == lens.label);

        match i {
            None => {
                b.push(lens);
            }
            Some(x) => {
                b[x].power = lens.power;
            }
        }
    }

    pub fn update(&mut self, s: String) {
        if s.trim().ends_with("-") {
            let parts = s.trim().split("-").collect::<Vec<&str>>();
            self.remove(parts[0]);
        }
        else {
            let parts = s.trim().split('=').collect::<Vec<&str>>();
            self.add(Lens {
                label: parts[0].to_string(),
                power: parts[1].to_string().parse().unwrap()
            });
        }
        //println!("{self}");
    }
}


struct Lens {
    label: String,
    power: i32
}

impl Display for Lens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{0} {1}]", self.label, self.power)
    }
}


pub fn hash(s: String) -> i32 {
    let mut h = 0;
    for s in s.chars() {
        h += s as i32;
        h = h * 17;
        h = h % 256;
    }
    h
}


pub fn hash_input(s: String) -> i32 {
    let mut sum = 0;
    for p in s.split(',') {
        sum += hash(p.to_string());
    }
    sum
}
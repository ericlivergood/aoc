use regex::Regex;

pub struct Node {
    pub(crate) name: String,
    pub(crate) left: String,
    pub(crate) right: String
}

impl Node {
    pub fn from(s: &str) -> Node {
        let re = Regex::new(r"([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)").unwrap();
        let c = re.captures(s).unwrap();

        Node {
            name: c.get(1).unwrap().as_str().to_string(),
            left: c.get(2).unwrap().as_str().to_string(),
            right: c.get(3).unwrap().as_str().to_string(),
        }
    }
}

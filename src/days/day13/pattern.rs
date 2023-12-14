use std::ffi::c_char;
use std::fmt::{Display, Formatter};
use array2d::Array2D;

pub struct Pattern {
    data: Array2D<char>
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.data.as_rows() {
            for c in row {
                write!(f, "{c}").unwrap();
            }
            writeln!(f);
        }
        writeln!(f)
    }
}

fn compare_vec(v1: &Vec<char>, v2: &Vec<char>) -> bool {
    for i in 0..v1.len() {
        if v1[i] != v2[i] {
            return false;
        }
    }
    true
}

fn check_reflection(data: &Vec<Vec<char>>, n: usize) -> bool {
    let mut l = n;
    let mut r = n+1;

    while l > 0 && r < data.len()-1  {
        r += 1;
        l -= 1;
        let v1 = &data[l];
        let v2 = &data[r];
        if !compare_vec(&v1, &v2) {
            return false;
        }
    }
    true
}

impl Pattern {
    pub fn new(lines: Vec<String>) -> Self {
        let mut rows = Vec::new();
        for l in lines {
            rows.push(l.trim().chars().collect())
        }

        let data = Array2D::from_rows(&rows).unwrap();
        Self {
            data
        }
    }

    pub fn find_horizontal_reflection_line(&self) -> i32 {
        self.find_horizontal_reflection_line_other_than(-1)
    }

    pub fn find_horizontal_reflection_line_other_than(&self, other: i32) -> i32 {
        let rows = self.data.as_rows();

        for n in 0..self.data.column_len() - 1 {
            if (n+1) as i32 == other {
                continue;
            }
            let r1 = rows.get(n).unwrap();
            let r2 = rows.get(n+1).unwrap();

            if compare_vec(r1, r2) {
                if check_reflection(&rows, n) {
                    //println!("horizontal line: {0}", n + 1);
                    return (n + 1) as i32;
                }
            }
        }

        0
    }

    pub fn find_vertical_reflection_line(&self) -> i32 {
        self.find_vertical_reflection_line_other_than(-1)
    }

    pub fn find_vertical_reflection_line_other_than(&self, other: i32) -> i32 {
        let cols = self.data.as_columns();

        for n in 0..self.data.row_len() - 1 {
            if (n+1) as i32 == other {
                continue;
            }
            let c1 = cols.get(n).unwrap();
            let c2 = cols.get(n+1).unwrap();

            if compare_vec(c1, c2) {
                if check_reflection(&cols, n) {
                    //println!("vertical line: {0}", n + 1);
                    return (n + 1) as i32;
                }
            }
        }
        0
    }

    pub fn toggle(&mut self, x: usize, y: usize) {
        let cur = self.data.get(y, x).unwrap();
        match cur {
            '#' => {
                self.data.set(y, x, '.')
                    .expect("failed to toggle");
            },
            '.' => {
                self.data.set(y, x, '#')
                    .expect("failed to toggle");
            }
            _=> {
                println!("wut {cur}");
                panic!();
            }
        }
    }

    pub fn fix_smudge(&mut self) {
        let vert = self.find_vertical_reflection_line();
        let horiz = self.find_horizontal_reflection_line();
        println!("orig: v: {vert}, h: {horiz}");
        for y in 0..self.data.column_len() {
            for x in 0..self.data.row_len() {
                self.toggle(x,y);
                let new_vert = self.find_vertical_reflection_line_other_than(vert);
                if new_vert > 0 {
                    println!("found new vert @ {new_vert}");
                    return;
                }
                let new_horiz = self.find_horizontal_reflection_line_other_than(horiz);
                if new_horiz > 0 {
                    println!("found new vert @ {new_vert}");
                    return;
                }
                self.toggle(x,y);
            }
        }
    }

    pub fn summarize(&mut self, fix_smudge: bool) -> i32 {
        let mut v = self.find_vertical_reflection_line();
        let mut h = self.find_horizontal_reflection_line();
        //println!("{self}");

        if(fix_smudge) {
            let orig_v = v;
            let orig_h = h;
            self.fix_smudge();

            v = self.find_vertical_reflection_line_other_than(orig_v);
            h = self.find_horizontal_reflection_line_other_than(orig_h);

            // if v == orig_v {
            //     v = 0;
            // }
            // if h == orig_h {
            //     h = 0;
            // }
            //
            // if h == 0 && v == 0 {
            //     println!("{self}");
            //     panic!();
            // }
        }
        println!("summary v: {v}, h: {h}");
        v + h*100
    }
}
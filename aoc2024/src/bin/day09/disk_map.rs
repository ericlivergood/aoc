#[derive(Clone,Copy)]
pub struct File {
    _id: i64,
    length: i64,
    location: i64
}

pub struct DiskMap {
    pub blocks: Vec<i64>, //0 if free, id of file otherwise
    pub files: Vec<File>
}

impl DiskMap {
    pub fn from_string(string: &String) -> DiskMap {
        let mut blocks: Vec<i64> = Vec::new();
        let mut files: Vec<File> = Vec::new();
        let mut i = 0;
        let mut block_id = 0;
        for c in string.chars() {
            let n_res = c.to_string().parse::<i64>();
            if n_res.is_err() {
                panic!("could not parse {}: {}", c, n_res.unwrap_err());
            }
            let n = n_res.unwrap();
            match i % 2 {
                0 => {
                    let id = i / 2;
                    files.push(File {
                        _id: id,
                        length: n,
                        location: block_id
                    });

                    for _i in 0..n {
                        blocks.push(id);
                        block_id += 1;
                    }
                },
                1 => {
                    for _i in 0..n {
                        blocks.push(-1);
                        block_id += 1;
                    }
                },
                _ => panic!("not possible...")
            }
            i += 1;
        }

        DiskMap {
            blocks,
            files
        }
    }

    pub fn checksum(&self) -> i64 {
        let mut sum: i64 = 0;
        let mut i = 0 as i64;
        for n in self.blocks.iter() {
            if n > &0 {
                sum += i * n;
            }
            i += 1;
        }
        sum
    }

    fn swap_blocks(&mut self, i: i64, j: i64) {
        let tmp = self.blocks[i as usize];
        self.blocks[i as usize] = self.blocks[j as usize];
        self.blocks[j as usize] = tmp;
        //self.print_blocks();
    }

    fn next_free_block(&self, from: i64) -> Result<i64, String> {
        let length = self.blocks.len() as i64;
        for i in from..length {
            if self.blocks[i as usize] == -1 {
                return Ok(i);
            }
        }

        Err("no free block found".to_string())
    }

    fn next_free_block_of_size(&self, size: i64) -> Result<i64, String> {
        let mut run_length = 0;
        let mut run_start = 0;
        for i in 0..self.blocks.len() {
            match self.blocks[i] {
                -1 => {
                    if run_length == 0 {
                        run_start = i;
                    }

                    run_length += 1;
                    if run_length == size {
                        return Ok(run_start as i64);
                    }
                },
                _ => {
                    run_length = 0;
                }
            }
        }
        Err("none available".to_string())
    }

    pub fn defrag(&mut self) {
        while !self.is_defragged() {
            for i in (0..self.blocks.len()).rev() {
                if self.blocks[i] != -1 {
                    let swap_to = self.next_free_block(0).unwrap();
                    if swap_to < (i as i64) {
                        self.swap_blocks(i as i64, swap_to);
                    }
                    else {
                        break;
                    }
                }
            }
        }
    }

    pub fn compact(&mut self) {
        let mut files = self.files.clone();
        files.reverse();
        //self.print_blocks();
        for f in files {
            //println!("compacting file {} starting at location {}", f.id, f.location);
            match self.next_free_block_of_size(f.length) {
                Ok(n) => {
                    if n >= f.location {
                        continue;
                    }
                    for i in 0..f.length {
                        self.swap_blocks(n+i, f.location + i)
                    }
                },
                Err(_) => {
                    continue;
                }
            }
            //self.print_blocks();
        }
    }

    pub fn is_defragged(&self) -> bool {
        let mut saw_data = false;
        let mut saw_zero = false;

        for n in &self.blocks {
            if n > &-1 {
                saw_data = true;
            }
            else {
                saw_zero = true;
            }
            if saw_data && saw_zero && n > &-1 {
                return false;
            }
        }
        true
    }

    pub fn _print_blocks(&self) {
        for n in self.blocks.iter() {
            if *n == -1 {
                print!(".");
            }
            else {
                print!("{}", n);
            }
        }
        println!();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builds_disk_map() {
        let dm = DiskMap::from_string(&"12345".to_string());
        assert_eq!(dm.blocks.len(), 15);
        assert_eq!(dm.files.len(), 3);
        assert_eq!(dm.files[0]._id, 0);
        assert_eq!(dm.files[0].length, 1);
        assert_eq!(dm.files[1]._id, 1);
        assert_eq!(dm.files[1].length, 3);
        assert_eq!(dm.files[2]._id, 2);
        assert_eq!(dm.files[2].length, 5);

        let dm = DiskMap::from_string(&"90909".to_string());
        assert_eq!(dm.blocks.len(), 27);
        assert_eq!(dm.files.len(), 3);
        assert_eq!(dm.files[0]._id, 0);
        assert_eq!(dm.files[0].length, 9);
        assert_eq!(dm.files[1]._id, 1);
        assert_eq!(dm.files[1].length, 9);
        assert_eq!(dm.files[2]._id, 2);
        assert_eq!(dm.files[2].length, 9);
    }

    #[test]
    fn finds_free_block() {
        let blocks = vec![1,1,-1,-1,1,-1];
        let files = Vec::new();
        let dm = DiskMap {
            blocks,
            files
        };
        assert_eq!(dm.next_free_block(0), Ok(2));
        assert_eq!(dm.next_free_block(1), Ok(2));
        assert_eq!(dm.next_free_block(2), Ok(2));
        assert_eq!(dm.next_free_block(3), Ok(3));
        assert_eq!(dm.next_free_block(4), Ok(5));
        assert_eq!(dm.next_free_block(5), Ok(5));
    }

    #[test]
    fn swaps_blocks() {
        let blocks = vec![0,1,2,3,4,5,6];
        let files = Vec::new();
        let mut dm = DiskMap {
            blocks,
            files
        };
        dm.swap_blocks(0, 6);
        assert_eq!(dm.blocks[0], 6);
        assert_eq!(dm.blocks[6], 0);

        dm.swap_blocks(3, 1);
        assert_eq!(dm.blocks[3], 1);
        assert_eq!(dm.blocks[1], 3);
    }

    #[test]
    fn checks_defrag_state() {
        let dm = DiskMap {
            blocks: vec![1,2,3,4,5,6],
            files: Vec::new()
        };
        assert_eq!(dm.is_defragged(), true);

        let dm = DiskMap {
            blocks: vec![-1,1,2,3,4,5,6],
            files: Vec::new()
        };
        assert_eq!(dm.is_defragged(), false);

        let dm = DiskMap {
            blocks: vec![1,2,3,-1,4,5,6],
            files: Vec::new()
        };
        assert_eq!(dm.is_defragged(), false);

        let dm = DiskMap {
            blocks: vec![1,2,3,4,5,6,-1],
            files: Vec::new()
        };
        assert_eq!(dm.is_defragged(), true);
    }

    #[test]
    fn calculates_checksums() {
        let dm = DiskMap {
            blocks: vec![0,0,9,9,8,1,1,1,8,8,8,2,7,7,7,3,3,3,6,4,4,6,5,5,5,5,6,6,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1],
            files: Vec::new()
        };
        assert_eq!(dm.checksum(), 1928);
    }
}

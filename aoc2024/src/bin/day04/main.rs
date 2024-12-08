use array2d::Array2D;
use aoc2024::common::input_reader;

fn is_slice_a_match(slice: &str) -> Result<bool, &str> {
    if slice.len() != 4 {
        return Err("we only check slices of length 4");
    }

    Ok(slice == "XMAS" || slice == "SAMX")
}

fn get_slice_points(from_x: i32, from_y: i32, x_inc: i32, y_inc: i32, x_max: i32, y_max: i32, len: i32)
    -> Result<Vec<(i32, i32)>, String> {
    let mut points = Vec::new();
    let mut x = from_x;
    let mut y = from_y;
    for _i in 0..len {
        if x > x_max || x < 0{
            return Err("x is out of range".to_string());
        }
        if y > y_max || y < 0 {
            return Err("y is out of range".to_string());
        }
        points.push((y, x));
        x += x_inc;
        y += y_inc;
    }
    Ok(points)
}


fn create_directional_slices(puzzle: &Array2D<char>, x_inc: i32, y_inc: i32, slice_len: i32) -> Vec<String> {
    let x_max = puzzle.row_len() as i32 - 1;
    let y_max = puzzle.column_len() as i32 - 1;

    let mut slices = Vec::new();

    for y in 0..puzzle.column_len() {
        for x in 0..puzzle.row_len() {
            match get_slice_points(x as i32, y as i32, x_inc, y_inc, x_max, y_max, slice_len) {
                Ok(value) => {
                    let mut chars = Vec::new();
                    for point in value {
                        chars.push(*puzzle.get(point.0 as usize, point.1 as usize).unwrap());
                    }
                    slices.push(chars.iter().collect());
                },
                Err(_) => break,
            }
        }
    }
    slices
}

fn create_slices(puzzle: &Array2D<char>, slice_len: i32) -> Vec<String> {
    //println!("horiz");
    let mut slices = create_directional_slices(puzzle, 1, 0, slice_len);
    // println!("vert");
    slices.extend(create_directional_slices(puzzle, 0, 1, slice_len));
    // println!("diag");
    slices.extend(create_directional_slices(puzzle, 1, 1, slice_len));
    slices.extend(create_directional_slices(puzzle, -1, 1, slice_len));
    slices.extend(create_directional_slices(puzzle, 1, -1, slice_len));
    slices.extend(create_directional_slices(puzzle, -1, -1, slice_len));
    slices
}

fn part1(puzzle: Array2D<char>) {
    let mut count = 0;
    for s in create_slices(&puzzle, 4) {
        match is_slice_a_match(&s) {
            Ok(true) => {
                //println!()
                count += 1
            },
            Ok(false) => (),
            Err(_) => panic!("sliced wrong")
        }
    }

    println!("Part 1: {}", count);
}

fn build_3_x_3_tile(puzzle: &Array2D<char>, x_center: usize, y_center: usize) -> Result<Array2D<char>, String> {
    if x_center <= 0 || x_center >= puzzle.row_len() - 1 {
        return Err(String::from("x_center is out of bounds"));
    }
    if y_center <= 0 || y_center >= puzzle.column_len() - 1 {
        return Err(String::from("y_center is out of bounds"));
    }
    let mut tile = Array2D::filled_with('.', 3, 3);
    tile[[0,0].into()] = puzzle[[y_center-1,x_center-1].into()];
    tile[[1,0].into()] = puzzle[[y_center,x_center-1].into()];
    tile[[2,0].into()] = puzzle[[y_center+1,x_center-1].into()];
    tile[[0,1].into()] = puzzle[[y_center,x_center].into()];
    tile[[1,1].into()] = puzzle[[y_center,x_center].into()];
    tile[[2,1].into()] = puzzle[[y_center+1,x_center].into()];
    tile[[0,2].into()] = puzzle[[y_center-1,x_center+1].into()];
    tile[[1,2].into()] = puzzle[[y_center,x_center+1].into()];
    tile[[2,2].into()] = puzzle[[y_center+1,x_center+1].into()];
    Ok(tile)
}

fn is_m_or_s(c: &char) -> bool {
    *c == 'S' || *c == 'M'
}

fn all_m_or_s(cs: &Vec<char>) -> bool {
    for c in cs {
        if !is_m_or_s(c) {
            return false;
        }
    }
    true
}

fn is_x_mas_tile(tile: &Array2D<char>) -> bool {
    if tile[[1,1].into()] != 'A' {
        return false;
    }

    let corners = vec![tile[[0,0].into()],tile[[0,2].into()],tile[[2,0].into()],tile[[2,2].into()]];
    if !all_m_or_s(&corners) {
        return false;
    }

    if corners[0] == corners[3] || corners[1] == corners[2] {
        return false;
    }

    true
}

fn part2(puzzle: Array2D<char>) {
    let mut count = 0;
    for y in 0..puzzle.column_len() {
        for x in 0..puzzle.row_len() {
            match puzzle.get(y, x) {
                Some('A') => {
                    match build_3_x_3_tile(&puzzle, x, y) {
                        Ok(value) => {
                            if is_x_mas_tile(&value) {
                                count += 1;
                            }
                        }
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

    println!("Part 2: {}", count);
}

fn main() {
    let puzzle = input_reader::input_reader::get_as_2d_array("./data/day04/input");
    part1(puzzle.clone());
    part2(puzzle.clone());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generates_horizontal_points() {
        let points = get_slice_points(0, 0, 1, 0, 10, 10, 4).unwrap();
        assert_eq!(points.len(), 4);
        assert_eq!(points[0], (0, 0));
        assert_eq!(points[1], (1, 0));
        assert_eq!(points[2], (2, 0));
        assert_eq!(points[3], (3, 0));


        let points = get_slice_points(4, 4, 1, 0, 10, 10, 4).unwrap();
        assert_eq!(points.len(), 4);
        assert_eq!(points[0], (4, 4));
        assert_eq!(points[1], (5, 4));
        assert_eq!(points[2], (6, 4));
        assert_eq!(points[3], (7, 4));

        let points = get_slice_points(8, 5, 1, 0, 10, 10, 4);
        assert!(points.is_err());
    }
    #[test]
    fn generates_vertical_points() {
        let points = get_slice_points(0, 0, 0, 1, 10, 10, 4).unwrap();
        assert_eq!(points.len(), 4);
        assert_eq!(points[0], (0, 0));
        assert_eq!(points[1], (0, 1));
        assert_eq!(points[2], (0, 2));
        assert_eq!(points[3], (0, 3));


        let points = get_slice_points(4, 4, 0, 1, 10, 10, 4).unwrap();
        assert_eq!(points.len(), 4);
        assert_eq!(points[0], (4, 4));
        assert_eq!(points[1], (4, 5));
        assert_eq!(points[2], (4, 6));
        assert_eq!(points[3], (4, 7));

        let points = get_slice_points(5, 8, 0, 1, 10, 10, 4);
        assert!(points.is_err());
    }
    #[test]
    fn generates_diagonal_points() {
        let points = get_slice_points(0, 0, 1, 1, 10, 10, 4).unwrap();
        assert_eq!(points.len(), 4);
        assert_eq!(points[0], (0, 0));
        assert_eq!(points[1], (1, 1));
        assert_eq!(points[2], (2, 2));
        assert_eq!(points[3], (3, 3));


        let points = get_slice_points(4, 4, 1, 1, 10, 10, 4).unwrap();
        assert_eq!(points.len(), 4);
        assert_eq!(points[0], (4, 4));
        assert_eq!(points[1], (5, 5));
        assert_eq!(points[2], (6, 6));
        assert_eq!(points[3], (7, 7));

        let points = get_slice_points(8, 5, 1, 1, 10, 10, 4);
        assert!(points.is_err());
    }

    #[test]
    fn generates_slices() {
        let data = vec![vec!['0','1','2'], vec!['4','5','6'], vec!['7','8','9']];
        let puzzle = Array2D::from_rows(&data).unwrap();
        //horizontal
        let slices = create_directional_slices(&puzzle, 1, 0, 2);
        assert_eq!(slices.len(), 6);

        //vertical
        let slices = create_directional_slices(&puzzle, 0, 1, 2);
        assert_eq!(slices.len(), 6);

        //diagonal
        let slices = create_directional_slices(&puzzle, 1, 1, 2);
        assert_eq!(slices.len(), 4);
    }


    #[test]
    fn matches_slices(){
        assert_eq!(is_slice_a_match("XMAS"), Ok(true));
        assert_eq!(is_slice_a_match("SAMX"), Ok(true));
        assert_eq!(is_slice_a_match("ASDF"), Ok(false));
        assert_eq!(is_slice_a_match("xmas"), Ok(false));
        assert_eq!(is_slice_a_match("ASDFA"), Err("we only check slices of length 4"));
    }

    #[test]
    fn detects_x_mas_tile() {
        let rows = vec![
            vec!['M', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'S'],
        ];

        let tile = Array2D::from_rows(&rows).unwrap();
        assert_eq!(is_x_mas_tile(&tile), true);

        let rows = vec![
            vec!['M', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'S'],
        ];

        let tile = Array2D::from_rows(&rows).unwrap();
        assert_eq!(is_x_mas_tile(&tile), true);

        let rows = vec![
            vec!['S', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'M'],
        ];

        let tile = Array2D::from_rows(&rows).unwrap();
        assert_eq!(is_x_mas_tile(&tile), true);

        let rows = vec![
            vec!['S', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'M'],
        ];

        let tile = Array2D::from_rows(&rows).unwrap();
        assert_eq!(is_x_mas_tile(&tile), true);

        let rows = vec![
            vec!['S', '.', 'M'],
            vec!['.', 'X', '.'],
            vec!['S', '.', 'M'],
        ];

        let tile = Array2D::from_rows(&rows).unwrap();
        assert_eq!(is_x_mas_tile(&tile), false);

        let rows = vec![
            vec!['M', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'M'],
        ];

        let tile = Array2D::from_rows(&rows).unwrap();
        assert_eq!(is_x_mas_tile(&tile), false);
    }

}
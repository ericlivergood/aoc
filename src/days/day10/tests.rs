#[cfg(test)]
mod tests {
    use crate::days::day10::pipe::Pipe;
    use crate::days::day10::point::Point;

    #[test]
    fn point_equality() {
        let p1 = Point::new(0,0);
        let p2 = Point::new(0,0);
        let p3 = Point::new(1,0);
        let p4 = Point::new(1,1);
        let p5 = Point::new(0,1);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
        assert_ne!(p1, p4);
        assert_ne!(p1, p5);
        assert_ne!(p2, p3);
        assert_ne!(p2, p3);
        assert_ne!(p2, p4);
        assert_ne!(p3, p4);
        assert_ne!(p3, p5);
        assert_ne!(p4, p5);
    }

    #[test]
    fn pipe_parses_pipes() {
        let pipe = Pipe::new(2,2, '|');
        assert_eq!(pipe.shape, '|');
        assert_eq!(pipe.point.x, 2);
        assert_eq!(pipe.point.y, 2);
        assert_eq!(pipe.is_connected_to(Point::new(2, 1)), true);
        assert_eq!(pipe.is_connected_to(Point::new(2, 3)), true);

        assert_eq!(pipe.is_connected_to(Point::new(1, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(1, 2)), false);
        assert_eq!(pipe.is_connected_to(Point::new(1, 3)), false);

        assert_eq!(pipe.is_connected_to(Point::new(3, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(3, 2)), false);
        assert_eq!(pipe.is_connected_to(Point::new(3, 3)), false);
    }

    #[test]
    fn pipe_parses_dashes() {
        let pipe = Pipe::new(2,2, '-');
        assert_eq!(pipe.shape, '-');
        assert_eq!(pipe.point.x, 2);
        assert_eq!(pipe.point.y, 2);
        assert_eq!(pipe.is_connected_to(Point::new(2, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(2, 3)), false);

        assert_eq!(pipe.is_connected_to(Point::new(1, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(1, 2)), true);
        assert_eq!(pipe.is_connected_to(Point::new(1, 3)), false);

        assert_eq!(pipe.is_connected_to(Point::new(3, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(3, 2)), true);
        assert_eq!(pipe.is_connected_to(Point::new(3, 3)), false);
    }

    #[test]
    fn pipe_parses_ls() {
        let pipe = Pipe::new(2,2, 'L');
        assert_eq!(pipe.shape, 'L');
        assert_eq!(pipe.point.x, 2);
        assert_eq!(pipe.point.y, 2);
        assert_eq!(pipe.is_connected_to(Point::new(2, 1)), true);
        assert_eq!(pipe.is_connected_to(Point::new(2, 3)), false);

        assert_eq!(pipe.is_connected_to(Point::new(1, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(1, 2)), false);
        assert_eq!(pipe.is_connected_to(Point::new(1, 3)), false);

        assert_eq!(pipe.is_connected_to(Point::new(3, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(3, 2)), true);
        assert_eq!(pipe.is_connected_to(Point::new(3, 3)), false);
    }

    #[test]
    fn pipe_parses_js() {
        let pipe = Pipe::new(2,2, 'J');
        assert_eq!(pipe.shape, 'J');
        assert_eq!(pipe.point.x, 2);
        assert_eq!(pipe.point.y, 2);
        assert_eq!(pipe.is_connected_to(Point::new(2, 1)), true);
        assert_eq!(pipe.is_connected_to(Point::new(2, 3)), false);

        assert_eq!(pipe.is_connected_to(Point::new(1, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(1, 2)), true);
        assert_eq!(pipe.is_connected_to(Point::new(1, 3)), false);

        assert_eq!(pipe.is_connected_to(Point::new(3, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(3, 2)), false);
        assert_eq!(pipe.is_connected_to(Point::new(3, 3)), false);
    }

    #[test]
    fn pipe_parses_7s() {
        let pipe = Pipe::new(2,2, '7');
        assert_eq!(pipe.shape, '7');
        assert_eq!(pipe.point.x, 2);
        assert_eq!(pipe.point.y, 2);
        assert_eq!(pipe.is_connected_to(Point::new(2, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(2, 3)), true);

        assert_eq!(pipe.is_connected_to(Point::new(1, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(1, 2)), true);
        assert_eq!(pipe.is_connected_to(Point::new(1, 3)), false);

        assert_eq!(pipe.is_connected_to(Point::new(3, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(3, 2)), false);
        assert_eq!(pipe.is_connected_to(Point::new(3, 3)), false);
    }

    #[test]
    fn pipe_parses_fs() {
        let pipe = Pipe::new(2,2, 'F');
        assert_eq!(pipe.shape, 'F');
        assert_eq!(pipe.point.x, 2);
        assert_eq!(pipe.point.y, 2);
        assert_eq!(pipe.is_connected_to(Point::new(2, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(2, 3)), true);

        assert_eq!(pipe.is_connected_to(Point::new(1, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(1, 2)), false);
        assert_eq!(pipe.is_connected_to(Point::new(1, 3)), false);

        assert_eq!(pipe.is_connected_to(Point::new(3, 1)), false);
        assert_eq!(pipe.is_connected_to(Point::new(3, 2)), true);
        assert_eq!(pipe.is_connected_to(Point::new(3, 3)), false);
    }
}
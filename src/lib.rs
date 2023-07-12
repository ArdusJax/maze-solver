//base cases
// 1. It's a wall
// 2. Off of the map
// 3. It's the end
// 4. Has been seen already

// Represents North, East, South and West
const DIR: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

pub fn walk(
    maze: Vec<Vec<char>>,
    wall: char,
    curr: Point,
    end: Point,
    seen: &mut Vec<Vec<bool>>,
    path: &mut Vec<Point>,
) -> bool {
    if (curr.x as i32) < 0 || curr.x as usize >= maze[0].len() {
        return false;
    }

    if maze[curr.y as usize][curr.x as usize] == wall {
        return false;
    }

    if curr.x == end.x && curr.y == end.y {
        path.push(curr.clone());
        return true;
    }

    if seen[curr.y as usize][curr.x as usize] {
        return false;
    }

    //pre
    path.push(curr.clone());
    seen[curr.y as usize][curr.x as usize] = true;
    //recurse
    for i in DIR.iter() {
        let (x, y) = (i[0], i[1]);
        if walk(
            maze.clone(),
            wall.clone(),
            Point::new(curr.x + x, curr.y + y),
            end.clone(),
            seen,
            path,
        ) {
            return true;
        };
    }
    //post
    path.pop();

    false
}

pub fn solve(maze: Vec<Vec<char>>, wall: char, start: Point, end: Point) -> Vec<Point> {
    let mut seen = vec![vec![false; maze[0].len()]; maze.len()];
    let mut path = Vec::<Point>::new();

    walk(maze, wall, start, end, &mut seen, &mut path);
    path
}

#[cfg(test)]
mod tests {

    use crate::{solve, Point};

    #[test]
    fn amazing_maze() {
        let mut maze: Vec<Vec<char>> = Vec::new();
        maze.push("xxxxxxxxxx x".chars().collect());
        maze.push("x        x x".chars().collect());
        maze.push("x        x x".chars().collect());
        maze.push("x xxxxxxxx x".chars().collect());
        maze.push("x          x".chars().collect());
        maze.push("x xxxxxxxxxx".chars().collect());

        let mut expected: Vec<Point> = Vec::new();
        expected.push(Point::new(10, 0));
        expected.push(Point::new(10, 1));
        expected.push(Point::new(10, 2));
        expected.push(Point::new(10, 3));
        expected.push(Point::new(10, 4));
        expected.push(Point::new(9, 4));
        expected.push(Point::new(8, 4));
        expected.push(Point::new(7, 4));
        expected.push(Point::new(6, 4));
        expected.push(Point::new(5, 4));
        expected.push(Point::new(4, 4));
        expected.push(Point::new(3, 4));
        expected.push(Point::new(2, 4));
        expected.push(Point::new(1, 4));
        expected.push(Point::new(1, 5));

        let result = solve(maze, 'x', Point::new(10, 0), Point::new(1, 5));
        assert_eq!(result, expected);
    }
}

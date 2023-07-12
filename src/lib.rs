//base cases
// 1. It's a wall
// 2. Off of the map
// 3. It's the end
// 4. Has been seen already

use std::str::Chars;

// Represents North, East, South and West
const DIR: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

#[derive(Clone)]
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

    #[test]
    fn amazing_maze() {
        let mut maze: Vec<Vec<char>> = Vec::new();
        maze.push("xxxxxxxxxx x".chars().collect());
        maze.push("x        x x".chars().collect());
        maze.push("x        x x".chars().collect());
        maze.push("x xxxxxxxx x".chars().collect());
        maze.push("x          x".chars().collect());
        maze.push("x xxxxxxxxxx".chars().collect());
    }
}

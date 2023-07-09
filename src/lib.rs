//base cases
// 1. It's a wall
// 2. Off of the map
// 3. It's the end
// 4. Has been seen already

pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub fn walk(maze: Vec<Vec<String>>, wall: String, curr: Point, end: Point) -> bool {
    if (curr.x as i32) < 0 || curr.x >= maze[0].len() {
        return false;
    }

    if maze[curr.y][curr.x] == wall {
        return false;
    }

    if curr.x == end.x && curr.y == end.y {
        return true;
    }

    false
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

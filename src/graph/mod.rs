// Based on https://www.redblobgames.com/pathfinding/a-star/implementation.html

use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Sub;

#[derive(Debug, PartialEq)]
pub struct Graph {
    pub edges: HashMap<char, Vec<char>>,
}

#[allow(dead_code)]
impl Graph {
    pub fn neighbors(&self, id: char)-> Option<&Vec<char>> {
        self.edges.get(&id)
    }

    pub fn breadth_first_search_1(&self, start: char) {
        let mut frontier: Queue<char> = Queue{ elements: VecDeque::new()};
        frontier.put(start);
        let mut visited: HashSet<char> = HashSet::new();
        &mut visited.insert(start);

        while !&frontier.empty() {
            let current = &frontier.get().unwrap();
            println!("Visiting {}", current);
            for next in self.neighbors(*current).unwrap() {
                if !visited.contains(&next) {
                    &mut frontier.put(next.clone());
                    &mut visited.insert(*next);
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub struct SquareGrid {
    pub width: usize,
    pub height: usize,
    pub walls: HashSet<Point>,
}

#[allow(dead_code)]
pub enum DisplayOptions {
    Parent,
    CostSoFar,
}

#[allow(dead_code)]
impl SquareGrid {
    fn in_bounds(&self, id: &Point)-> bool {
        (0 <= id.x) && (id.x < (self.width as isize)) &&
        (0 <= id.y) && (id.y < (self.height as isize))
    }

    fn passable(&self, id: &Point)-> bool {
        !self.walls.contains(id)
    }

    fn neighbors(&self, id: &Point)-> Vec<Point> {
        let n1 = Point{x: id.x + 1, y: id.y};
        let n2 = Point{x: id.x, y: id.y - 1};
        let n3 = Point{x: id.x - 1, y: id.y};
        let n4 = Point{x: id.x, y: id.y + 1};

        let pre_check_results = vec![n1, n2, n3, n4];
        pre_check_results.iter()
                         .map(|n| *n)
                         .filter(|n| self.in_bounds(n))
                         .filter(|n| self.passable(n))
                         .collect()
    }

    pub fn breadth_first_search_2(&self, start: Point)-> HashMap<Point, Point> {
        let mut frontier: Queue<Point> = Queue{ elements: VecDeque::new()};
        frontier.put(start);

        let mut came_from: HashMap<Point, Point> = HashMap::new();
        &mut came_from.insert(start, start);

        while !&frontier.empty() {
            let current = &frontier.get().unwrap();
            for next in self.neighbors(current) {
                if !came_from.contains_key(&next) {
                    &mut frontier.put(next.clone());
                    &mut came_from.insert(next, *current);
                }
            }
        }
        came_from
    }

    pub fn draw_grid(&self, width: usize, parents: &HashMap<Point, Point>) {
        for y in 0..self.height {
            let mut row = String::new();
            for x in 0..self.width {
                let current_point = Point{x: x as isize, y: y as isize};
                let mut next_node_str = String::new();
                if !self.walls.contains(&current_point) {
                    next_node_str = match parents.get(&current_point) {
                        Some(parent) => direction_of_neighbour(current_point, *parent),
                        None => String::from(".")
                    };
                    while next_node_str.len() < width {
                        next_node_str.push(' ');
                    }
                } else {
                    while next_node_str.len() < width {
                        next_node_str.push('#');
                    }
                }
                row.push_str(&next_node_str)
            }
            println!("{}", row);
        }
    }

}

#[derive(Debug, PartialEq)]
struct Queue<T> {
    elements: VecDeque<T>
}

impl <T> Queue<T> {
    pub fn empty(&self)-> bool {
        self.elements.len() == 0
    }

    pub fn put(&mut self, x: T) {
        self.elements.push_back(x)
    }

    pub fn get(&mut self)-> Option<T> {
        self.elements.pop_front()
    }
}

#[allow(dead_code)]
pub fn direction_of_neighbour(from_point: Point, to_point: Point)-> String {
    match from_point - to_point {
        Point{x: -1, y:  0} => String::from(">"),
        Point{x:  1, y:  0} => String::from("<"),
        Point{x:  0, y: -1} => String::from("v"),
        Point{x:  0, y:  1} => String::from("^"),
        _ => String::from("."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_subtraction() {
        let test_point1 = Point{x: 5, y: 5};
        let test_point2 = Point{x: 6, y: 6};
        let test_point3 = Point{x: 4, y: 5};
        let test_point4 = Point{x: 5, y: 4};

        assert_eq!(Point{x:  0, y:  0}, test_point1 - test_point1);
        assert_eq!(Point{x:  1, y:  1}, test_point2 - test_point1);
        assert_eq!(Point{x: -1, y:  0}, test_point3 - test_point1);
        assert_eq!(Point{x:  0, y: -1}, test_point4 - test_point1);
    }

    #[test]
    fn neighbor_direction_is_correct() {
        let test_point1 = Point{x: 5, y: 5};
        let test_point2 = Point{x: 5, y: 6};
        let test_point3 = Point{x: 5, y: 4};
        let test_point4 = Point{x: 6, y: 5};
        let test_point5 = Point{x: 4, y: 5};

        assert_eq!(String::from("."), direction_of_neighbour(test_point1, test_point1));
        assert_eq!(String::from("v"), direction_of_neighbour(test_point1, test_point2));
        assert_eq!(String::from("^"), direction_of_neighbour(test_point1, test_point3));
        assert_eq!(String::from(">"), direction_of_neighbour(test_point1, test_point4));
        assert_eq!(String::from("<"), direction_of_neighbour(test_point1, test_point5));
    }
}
#![feature(field_init_shorthand)]

use std::collections::{VecDeque, BTreeSet};

fn main() {
    let maze = Maze::new(1362);
    let path = maze.shortest_path(Point { x: 31, y: 39 });
    let steps = maze.locations_within_steps(50);

    println!("The path was {:?}", path);
    println!("Within 50 steps, there are {} unique steps", steps);
}

#[derive(Debug)]
pub struct Maze(u32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: u32,
    y: u32,
}

impl From<(u32, u32)> for Point {
    fn from((x, y): (u32, u32)) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State(Point, usize);

const INITIAL_POINT: Point = Point { x: 1, y: 1};
const INITIAL_DEPTH: usize = 0;

#[derive(Debug, Default)]
struct MazeQueue {
    queue: VecDeque<State>,
    seen: BTreeSet<Point>,
}

impl MazeQueue {
    fn new() -> Self {
        Self::default()
    }

    fn push_if_new(&mut self, point: Point, depth: usize) {
        if !self.seen.contains(&point) {
            self.seen.insert(point);
            self.queue.push_back(State(point, depth));
        }
    }

    fn pop(&mut self) -> Option<State> {
        self.queue.pop_front()
    }

    fn unique_visited(&self) -> usize {
        let seen_but_not_considered = self.queue.len();
        self.seen.len() - seen_but_not_considered
    }
}

impl Maze {
    fn new(favorite_number: u32) -> Self {
        Maze(favorite_number)
    }

    fn is_empty(&self, point: Point) -> bool {
        let Point { x, y } = point;
        let algorithm = x * x + 3 * x + 2 * x * y + y + y * y;
        let sum = algorithm + self.0;
        sum.count_ones() % 2 == 0
    }

    // TODO: Could this be made into an iterator? Not as-is, as we
    // can't return references to the stack-allocated arrays.
    fn next_steps(&self, point: Point) -> BTreeSet<Point> {
        let Point { x, y } = point;

        let points_with_x_varied = [x.checked_sub(1), x.checked_add(1)];
        let points_with_x_varied = points_with_x_varied.iter().flat_map(|x| x).map(|&x| (x, y));

        let points_with_y_varied = [y.checked_sub(1), y.checked_add(1)];
        let points_with_y_varied = points_with_y_varied.iter().flat_map(|y| y).map(|&y| (x, y));

        points_with_x_varied
            .chain(points_with_y_varied)
            .map(Into::into)
            .filter(|&point| self.is_empty(point))
            .collect()
    }

    fn shortest_path(&self, end_point: Point) -> Option<usize> {
        if end_point == INITIAL_POINT {
            return Some(INITIAL_DEPTH);
        }

        let mut queue = MazeQueue::new();
        let mut max_depth_seen = 0;

        queue.push_if_new(INITIAL_POINT, INITIAL_DEPTH);

        while let Some(State(point, depth)) = queue.pop() {
            if max_depth_seen != depth {
                max_depth_seen = depth;
            }

            let depth = depth + 1;

            for point in self.next_steps(point) {
                if point == end_point {
                    return Some(depth);
                }

                queue.push_if_new(point, depth);
            }
        }

        None
    }

    fn locations_within_steps(&self, steps: usize) -> usize {
        let mut queue = MazeQueue::new();

        queue.push_if_new(INITIAL_POINT, INITIAL_DEPTH);

        while let Some(State(point, depth)) = queue.pop() {
            if depth > steps {
                break;
            }

            let depth = depth + 1;

            for point in self.next_steps(point) {
                queue.push_if_new(point, depth);
            }
        }

        // We popped one off the queue and visited it before stopping iteration
        // This may be incorrect if we run out of nodes?
        queue.unique_visited() - 1
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use super::*;

    fn points<I>(iter: I) -> BTreeSet<Point>
        where I: IntoIterator,
              I::Item: Into<Point>,
    {
        iter.into_iter().map(Into::into).collect()
    }

    #[test]
    fn one_one_is_empty() {
        let maze = Maze::new(10);
        assert!(maze.is_empty(Point { x: 1, y: 1 }));
    }

    #[test]
    fn one_zero_is_a_wall() {
        let maze = Maze::new(10);
        assert!(!maze.is_empty(Point { x: 1, y: 0 }));
    }

    #[test]
        fn next_steps() {
        let maze = Maze::new(10);
        let expected = points(vec![(3, 2), (3, 4)]);

        assert_eq!(maze.next_steps(Point { x: 3, y: 3 }), expected);
    }

    #[test]
    fn next_steps_corner() {
        let maze = Maze::new(10);
        let expected = points(vec![(0, 1)]);

        assert_eq!(maze.next_steps(Point { x: 0, y: 0 }), expected);
    }

    #[test]
    fn shortest_path() {
        let maze = Maze::new(10);
        assert_eq!(maze.shortest_path(Point { x: 7, y: 4 }), Some(11));
    }

    #[test]
    fn locations_within_steps() {
        let maze = Maze::new(10);
        assert_eq!(maze.locations_within_steps(0), 1);
        assert_eq!(maze.locations_within_steps(1), 3);
        assert_eq!(maze.locations_within_steps(2), 5);
        assert_eq!(maze.locations_within_steps(3), 6);
        assert_eq!(maze.locations_within_steps(4), 9);
    }
}

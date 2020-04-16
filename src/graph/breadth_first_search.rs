use std::cmp::PartialEq;
use std::collections::{HashSet, LinkedList};
use std::hash::{Hash, Hasher};
use std::vec::Vec;

#[derive(Eq, Debug)]
pub struct Point {
    r: usize,
    c: usize,
}

impl Point {
    fn new(r: usize, c: usize) -> Point {
        Point { r: r, c: c }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.r == other.r && self.c == other.c
    }
}

impl Hash for Point {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let data = [self.r as u8, self.c as u8];
        state.write(&data);
        state.finish();
    }
}

fn is_inside_maze(r: i64, c: i64, number_of_rows: i64, number_of_columns: i64) -> bool {
    r >= 0 && r < number_of_rows && c >= 0 && c < number_of_columns
}

pub fn breadth_first_search(
    maze: Vec<Vec<char>>,
    entry: Point,
    exit: Point,
) -> Result<u64, &'static str> {
    let neighbors_row: [i64; 4] = [-1, 0, 0, 1];
    let neighbors_column: [i64; 4] = [0, -1, 1, 0];
    let neighbors_size: usize = 4;

    let number_of_rows: i64 = maze.len() as i64;
    let number_of_columns: i64 = maze[0].len() as i64;

    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: LinkedList<Point> = LinkedList::new();
    queue.push_back(entry);
    let mut moves: u64 = 0;
    while !queue.is_empty() {
        let size = queue.len();
        for _ in 0..size {
            let current: Point = queue.pop_front().unwrap();
            if current == exit {
                return Result::Ok(moves);
            }

            if maze[current.r][current.c] == '#' {
                continue;
            }

            if visited.contains(&current) {
                continue;
            }

            let current_row = current.r;
            let current_column = current.c;
            visited.insert(current);

            for i in 0..neighbors_size {
                let next_row = (current_row as i64) + neighbors_row[i];
                let next_column = (current_column as i64) + neighbors_column[i];

                if !is_inside_maze(next_row, next_column, number_of_rows, number_of_columns) {
                    continue;
                }

                let next_row = next_row as usize;
                let next_column = next_column as usize;
                if maze[next_row][next_column] == '#' {
                    continue;
                }

                let next_point: Point = Point::new(next_row, next_column);
                if visited.contains(&next_point) {
                    continue;
                }
                queue.push_back(next_point);
            }
        }
        moves += 1;
    }

    Result::Err("Exit point is not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eleven_by_eleven_maze() {
        let mut maze: Vec<Vec<char>> = Vec::new();
        maze.push("#X#########".chars().collect());
        maze.push("#         #".chars().collect());
        maze.push("### # ### #".chars().collect());
        maze.push("# #       #".chars().collect());
        maze.push("# # #######".chars().collect());
        maze.push("#   # # # #".chars().collect());
        maze.push("# ### # # #".chars().collect());
        maze.push("# #     # #".chars().collect());
        maze.push("# ### # # #".chars().collect());
        maze.push("#     #   #".chars().collect());
        maze.push("#########E#".chars().collect());

        let entry_point = Point { r: 10, c: 9 };

        let exit_point = Point { r: 0, c: 1 };

        assert_eq!(
            26,
            breadth_first_search(maze, entry_point, exit_point).unwrap()
        );
    }

    #[test]
    fn twentyone_by_twentyone_maze() {
        let mut maze: Vec<Vec<char>> = Vec::new();
        maze.push("#X###################".chars().collect());
        maze.push("#   #     #         #".chars().collect());
        maze.push("### ### ##### ### ###".chars().collect());
        maze.push("#         #     # # #".chars().collect());
        maze.push("##### ####### ### # #".chars().collect());
        maze.push("# #   #   #   # #   #".chars().collect());
        maze.push("# ### # # # ### #####".chars().collect());
        maze.push("#   #   #     #   # #".chars().collect());
        maze.push("### # ##### # ### # #".chars().collect());
        maze.push("#   # # #   #     # #".chars().collect());
        maze.push("# # ### ####### # # #".chars().collect());
        maze.push("# #   #         # # #".chars().collect());
        maze.push("# ### # # ######### #".chars().collect());
        maze.push("#   # # # # #   #   #".chars().collect());
        maze.push("# ##### ### # # ### #".chars().collect());
        maze.push("#     #   #   #     #".chars().collect());
        maze.push("### ### ### ##### ###".chars().collect());
        maze.push("# # # # # #     #   #".chars().collect());
        maze.push("# # # # # ### # ### #".chars().collect());
        maze.push("#             #   # #".chars().collect());
        maze.push("################### E".chars().collect());

        let entry_point = Point { r: 20, c: 20 };

        let exit_point = Point { r: 0, c: 1 };
        assert_eq!(
            75,
            breadth_first_search(maze, entry_point, exit_point).unwrap()
        );
    }
}

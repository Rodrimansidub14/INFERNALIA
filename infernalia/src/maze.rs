use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::File;
use std::io::{Write, BufWriter};

pub fn generate_and_save_maze(width: usize, height: usize, filename: &str, path_width: usize) -> (usize, usize, usize, usize) {
    let mut rng = rand::thread_rng();

    // Adjust dimensions to accommodate path width
    let maze_width = width * path_width + 1;
    let maze_height = height * path_width + 1;

    let mut maze = vec![vec!['#'; maze_width]; maze_height];

    fn carve_path(x: usize, y: usize, maze: &mut Vec<Vec<char>>, path_width: usize) {
        for dx in 0..path_width {
            for dy in 0..path_width {
                maze[y + dy][x + dx] = ' ';
            }
        }
    }

    fn walk(x: usize, y: usize, maze: &mut Vec<Vec<char>>, path_width: usize, rng: &mut impl Rng) {
        carve_path(x, y, maze, path_width);

        let mut directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        directions.shuffle(rng);

        for (dx, dy) in directions {
            let new_x = (x as isize + dx * (path_width as isize + 1)) as usize;
            let new_y = (y as isize + dy * (path_width as isize + 1)) as usize;

            if new_x > 0 && new_x < maze[0].len() - path_width && new_y > 0 && new_y < maze.len() - path_width {
                if maze[new_y][new_x] == '#' {
                    // Carve the connection between cells
                    let mid_x = (x as isize + dx * (path_width as isize + 1) / 2) as usize;
                    let mid_y = (y as isize + dy * (path_width as isize + 1) / 2) as usize;
                    carve_path(mid_x, mid_y, maze, path_width);

                    carve_path(new_x, new_y, maze, path_width);
                    walk(new_x, new_y, maze, path_width, rng);
                }
            }
        }
    }

    let start_x = path_width;
    let start_y = path_width;

    walk(start_x, start_y, &mut maze, path_width, &mut rng);

    // Place the start and goal points
    maze[start_y][start_x] = 'p'; // Start point
    let goal_x = maze_width - path_width - 1;
    let goal_y = maze_height - path_width - 1;
    maze[goal_y][goal_x] = 'g'; // Goal point

    // Write the maze to a file
    let mut file = BufWriter::new(File::create(filename).expect("Could not create maze file"));
    for row in maze.iter() {
        writeln!(file, "{}", row.iter().collect::<String>()).expect("Could not write to maze file");
    }

    // Return both the start and goal positions
    (start_x, start_y, goal_x, goal_y)
}

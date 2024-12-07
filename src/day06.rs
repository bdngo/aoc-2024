type Obstacles = Vec<(i64, i64)>;

#[derive(Clone, Debug, PartialEq)]
enum Orientation {
    North,
    West,
    South,
    East,
}

struct MapShape {
    rows: usize,
    cols: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct Guard {
    orientation: Orientation,
    position: (i64, i64),
}

fn walk_once(guard: Guard, obstacles: &Obstacles) -> Guard {
    match guard.orientation {
        Orientation::North => {
            let new_position = (guard.position.0 - 1, guard.position.1);
            if obstacles.contains(&new_position) {
                Guard {
                    orientation: Orientation::East,
                    position: guard.position,
                }
            } else {
                Guard {
                    orientation: guard.orientation,
                    position: new_position,
                }
            }
        }
        Orientation::East => {
            let new_position = (guard.position.0, guard.position.1 + 1);
            if obstacles.contains(&new_position) {
                Guard {
                    orientation: Orientation::South,
                    position: guard.position,
                }
            } else {
                Guard {
                    orientation: guard.orientation,
                    position: new_position,
                }
            }
        }
        Orientation::South => {
            let new_position = (guard.position.0 + 1, guard.position.1);
            if obstacles.contains(&new_position) {
                Guard {
                    orientation: Orientation::West,
                    position: guard.position,
                }
            } else {
                Guard {
                    orientation: guard.orientation,
                    position: new_position,
                }
            }
        }
        Orientation::West => {
            let new_position = (guard.position.0, guard.position.1 - 1);
            if obstacles.contains(&new_position) {
                Guard {
                    orientation: Orientation::North,
                    position: guard.position,
                }
            } else {
                Guard {
                    orientation: guard.orientation,
                    position: new_position,
                }
            }
        }
    }
}

fn full_walk(mut guard: Guard, obstacles: &Obstacles, map: &MapShape) -> Vec<(i64, i64)> {
    let mut covered_positions = Vec::new();
    while 0 <= guard.position.0
        && guard.position.0 < map.rows as i64
        && 0 <= guard.position.1
        && guard.position.1 < map.cols as i64
    {
        if !covered_positions.contains(&guard.position) {
            covered_positions.push(guard.position);
        }
        guard = walk_once(guard, &obstacles);
    }
    covered_positions
}

pub fn part1(input: String) -> u64 {
    let map_matrix: Vec<_> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let map = MapShape {
        rows: map_matrix.len(),
        cols: map_matrix[0].len(),
    };
    let obstacles: Obstacles = (0..map.rows)
        .flat_map(|i| (0..map.cols).map(move |j| (i, j)))
        .filter(|&(i, j)| map_matrix[i][j] == '#')
        .map(|(i, j)| (i as i64, j as i64))
        .collect();
    let flat_guard_position = map_matrix
        .iter()
        .flatten()
        .position(|&x| x == '^')
        .expect("no guard in this map");
    let init_guard_position = (
        (flat_guard_position as i64 / map.rows as i64),
        (flat_guard_position as i64 % map.cols as i64),
    );
    let guard = Guard {
        orientation: Orientation::North,
        position: init_guard_position,
    };
    full_walk(guard, &obstacles, &map).len() as u64
}

pub fn part2(input: String) -> u64 {
    let map_matrix: Vec<_> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let map = MapShape {
        rows: map_matrix.len(),
        cols: map_matrix[0].len(),
    };
    let obstacles: Obstacles = (0..map.rows)
        .flat_map(|i| (0..map.cols).map(move |j| (i, j)))
        .filter(|&(i, j)| map_matrix[i][j] == '#')
        .map(|(i, j)| (i as i64, j as i64))
        .collect();
    let flat_guard_position = map_matrix
        .iter()
        .flatten()
        .position(|&x| x == '^')
        .expect("no guard in this map");
    let init_guard_position = (
        (flat_guard_position as i64 / map.rows as i64),
        (flat_guard_position as i64 % map.cols as i64),
    );
    let guard = Guard {
        orientation: Orientation::North,
        position: init_guard_position,
    };
    let covered_positions = full_walk(guard, &obstacles, &map);
    let mut num_loops = 0;
    for (i, new_obstacle) in covered_positions.iter().enumerate() {
        println!("{}, {:?}", i, new_obstacle);
        let mut test_obstacles = obstacles.clone();
        test_obstacles.push(*new_obstacle);
        let mut loop_buffer = Vec::new();
        let mut loop_guard = Guard {
            orientation: Orientation::North,
            position: init_guard_position,
        };
        while 0 <= loop_guard.position.0
            && loop_guard.position.0 < map.rows as i64
            && 0 <= loop_guard.position.1
            && loop_guard.position.1 < map.cols as i64
        {
            if loop_buffer.contains(&loop_guard) {
                num_loops += 1;
                break;
            }
            loop_buffer.push(loop_guard.clone());
            loop_guard = walk_once(loop_guard, &test_obstacles);
        }
    }
    num_loops
}

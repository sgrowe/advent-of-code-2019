use super::file_utils;
use std::collections::HashSet;

pub fn main() {
    let input = file_utils::read("src/three.txt");

    println!("Part one:");
    println!("Distance to closest crossover: {}", part_one(&input));

    println!();
    println!("Part two:");
    println!(
        "Fewest combined steps to reach crossover: {}",
        part_two(&input)
    );
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Move {
    dir: Dir,
    dist: usize,
}

type Route = Vec<Move>;

type Coord = (i64, i64);

fn part_one(input: &str) -> i64 {
    let (a, b) = parse_routes(input);

    get_distance_to_nearest_crossover(&a, &b)
}

fn parse_routes(input: &str) -> (Route, Route) {
    let mut routes = input.trim().lines().map(|line| parse_route(line.trim()));

    (routes.next().unwrap(), routes.next().unwrap())
}

fn parse_route(input: &str) -> Route {
    input.split(',').map(parse_move).collect()
}

fn parse_move(input: &str) -> Move {
    let first_char = input.chars().next().unwrap();

    let dir = match first_char {
        'U' => Dir::Up,
        'D' => Dir::Down,
        'L' => Dir::Left,
        'R' => Dir::Right,
        _ => panic!("Unexpected char: {}", first_char),
    };

    let dist = input[1..].parse::<usize>().unwrap();

    Move { dir, dist }
}

fn get_distance_to_nearest_crossover(a: &Route, b: &Route) -> i64 {
    let path_a = places_visited(&a);
    let path_b = places_visited(&b);

    crossover_points(path_a, path_b)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

fn crossover_points<Path>(a: Path, b: Path) -> impl Iterator<Item = Coord>
where
    Path: Iterator<Item = Coord>,
{
    let places_a_visited = a.collect::<HashSet<(i64, i64)>>();

    b.filter(move |place| places_a_visited.contains(place))
}

fn places_visited<'a>(route: &'a Route) -> impl Iterator<Item = (i64, i64)> + 'a {
    route
        .iter()
        .flat_map(|&Move { dir, dist }| std::iter::repeat(dir).take(dist))
        .scan((0, 0), |state, dir| {
            let new_coord = make_move(*state, dir);

            *state = new_coord;

            Some(new_coord)
        })
}

fn make_move((x, y): Coord, step: Dir) -> Coord {
    match step {
        Dir::Up => (x, y + 1),
        Dir::Down => (x, y - 1),
        Dir::Left => (x - 1, y),
        Dir::Right => (x + 1, y),
    }
}

// PART TWO

fn part_two(input: &str) -> i64 {
    let (a, b) = parse_routes(input);

    fewest_combined_steps_to_crossover(&a, &b)
}

fn fewest_combined_steps_to_crossover(a: &Route, b: &Route) -> i64 {
    let path_a = places_visited(&a).collect::<Vec<Coord>>();
    let path_b = places_visited(&b).collect::<Vec<Coord>>();

    let crossovers = crossover_points(path_a.iter().cloned(), path_b.iter().cloned());

    crossovers
        .map(|(x, y)| {
            let steps_a = 1 + path_a
                .iter()
                .position(|(ax, ay)| *ax == x && *ay == y)
                .unwrap();

            let steps_b = 1 + path_b
                .iter()
                .position(|(bx, by)| *bx == x && *by == y)
                .unwrap();

            (steps_a + steps_b) as i64
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod part_one_tests {
    use super::*;

    #[test]
    fn example() {
        let a = parse_route("R8,U5,L5,D3");
        let b = parse_route("U7,R6,D4,L4");

        assert_eq!(get_distance_to_nearest_crossover(&a, &b), 6);
    }

    #[test]
    fn example_distance_test_1() {
        let (a, b) = parse_routes(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83",
        );

        assert_eq!(get_distance_to_nearest_crossover(&a, &b), 159);
    }

    #[test]
    fn example_distance_test_2() {
        let (a, b) = parse_routes(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
            U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );

        assert_eq!(get_distance_to_nearest_crossover(&a, &b), 135);
    }
}

#[cfg(test)]
mod part_two_tests {
    use super::*;

    #[test]
    fn example() {
        let a = parse_route("R8,U5,L5,D3");
        let b = parse_route("U7,R6,D4,L4");

        assert_eq!(fewest_combined_steps_to_crossover(&a, &b), 30);
    }

    #[test]
    fn example_test_case_1() {
        let (a, b) = parse_routes(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
            U62,R66,U55,R34,D71,R55,D58,R83",
        );

        assert_eq!(fewest_combined_steps_to_crossover(&a, &b), 610);
    }

    #[test]
    fn example_test_case_2() {
        let (a, b) = parse_routes(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
            U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );

        assert_eq!(fewest_combined_steps_to_crossover(&a, &b), 410);
    }
}

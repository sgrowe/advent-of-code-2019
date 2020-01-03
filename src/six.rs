use std::collections::HashMap;
use std::fs::read_to_string;

pub fn main() {
    let input = read_to_string("src/six.txt").unwrap();
    let input_trimmed = input.trim_end();

    println!("Part one:");
    println!("{}", count_direct_and_indirect_orbits(input_trimmed));
}

type PlanetOrbits<'a> = HashMap<&'a str, &'a str>;

fn count_direct_and_indirect_orbits(input: &str) -> u64 {
    let orbits: PlanetOrbits = input
        .lines()
        .map(parse_line)
        .fold(HashMap::new(), add_orbit);

    let mut count = 0;

    for planet in orbits.keys() {
        let mut orbiter = planet;

        while let Some(orbitee) = orbits.get(orbiter) {
            count += 1;
            orbiter = orbitee;
        }
    }

    count
}

struct Orbit<'a> {
    central_mass: &'a str,
    orbiter: &'a str,
}

fn parse_line<'a>(line: &'a str) -> Orbit<'a> {
    match line.split(")").collect::<Vec<&str>>()[..] {
        [central_mass, orbiter] => Orbit {
            central_mass,
            orbiter,
        },
        _ => panic!("Could not parse input line: {}", line),
    }
}

fn add_orbit<'a>(
    mut planets: PlanetOrbits<'a>,
    Orbit {
        central_mass,
        orbiter,
    }: Orbit<'a>,
) -> PlanetOrbits<'a> {
    planets.insert(orbiter, central_mass);

    planets
}

#[cfg(test)]
mod part_one_tests {
    use super::*;

    #[test]
    fn example_case_1() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

        assert_eq!(count_direct_and_indirect_orbits(input), 42);
    }
}

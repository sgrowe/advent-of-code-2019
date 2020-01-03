use std::collections::HashMap;
use std::fs::read_to_string;

pub fn main() {
    let input = read_to_string("src/six.txt").unwrap();
    let orbits = parse_orbits(&input);

    println!("Part one:");
    println!("{}", count_direct_and_indirect_orbits(&orbits));
    println!();

    println!("Part two:");
    println!("{}", orbital_transfers_between_you_and_santa(&orbits));
}

type PlanetOrbits<'a> = HashMap<&'a str, &'a str>;

fn parse_orbits<'a>(input: &'a str) -> PlanetOrbits<'a> {
    input
        .trim()
        .lines()
        .map(parse_line)
        .fold(HashMap::new(), add_orbit)
}

fn count_direct_and_indirect_orbits(orbits: &PlanetOrbits) -> usize {
    orbits
        .keys()
        .map(|planet| planets_orbits(&orbits, planet).count())
        .sum()
}

fn planets_orbits<'a>(
    all_orbits: &'a PlanetOrbits<'a>,
    planet: &'a str,
) -> PlanetsOrbitsIterator<'a> {
    PlanetsOrbitsIterator {
        orbits: all_orbits,
        current_planet: Some(planet),
    }
}

struct PlanetsOrbitsIterator<'a> {
    orbits: &'a PlanetOrbits<'a>,
    current_planet: Option<&'a str>,
}

impl<'a> Iterator for PlanetsOrbitsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_planet = self
            .current_planet
            .and_then(|planet| self.orbits.get(&planet).copied());

        self.current_planet
    }
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

fn orbital_transfers_between_you_and_santa(orbits: &PlanetOrbits) -> usize {
    let planet_distances_from_santa: HashMap<&str, usize> = planets_orbits(orbits, "SAN")
        .enumerate()
        .map(|(i, planet)| (planet, i))
        .collect();

    for (steps, planet) in planets_orbits(orbits, "YOU").enumerate() {
        if let Some(remaining) = planet_distances_from_santa.get(planet) {
            return steps + remaining;
        }
    }

    panic!("Did not find a common parent planet");
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

        let orbits = parse_orbits(input);

        assert_eq!(count_direct_and_indirect_orbits(&orbits), 42);
    }
}

#[cfg(test)]
mod part_two_tests {
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
K)L
K)YOU
I)SAN";

        let orbits = parse_orbits(input);

        assert_eq!(orbital_transfers_between_you_and_santa(&orbits), 4);
    }
}

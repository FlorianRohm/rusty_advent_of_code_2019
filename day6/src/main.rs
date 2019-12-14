use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn insert_into_map(
    mut map: HashMap<String, usize>,
    orbit_point: String,
    new_planet: String,
) -> HashMap<String, usize> {
    let length_until_now = map
        .get(&orbit_point)
        .expect(format!("orbit point {} not defined", orbit_point).as_ref());

    map.insert(new_planet, length_until_now + 1);
    map
}

fn get_number_of_orbits(orbit_points: Vec<(String, String)>) -> usize {
    let mut input = HashMap::new();
    input.insert("COM".into(), 0);

    orbit_points
        .into_iter()
        .fold(input, |acc, (orbit_point, new_planet)| {
            insert_into_map(acc, orbit_point, new_planet)
        })
        .values()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_insert_into_map() {
        let input = HashMap::from_iter(vec![("A".into(), 0), ("B".into(), 1)]);
        let expected = HashMap::from_iter(vec![("A".into(), 0), ("B".into(), 1), ("C".into(), 2)]);

        let new_map = insert_into_map(input, "B".into(), "C".into());

        assert_eq!(new_map, expected)
    }

    #[test]
    fn test_number_of_orbits() {
        let input: Vec<(String, String)> = vec![
            ("COM".into(), "B".into()),
            ("B".into(), "C".into()),
            ("C".into(), "D".into()),
            ("D".into(), "E".into()),
            ("E".into(), "F".into()),
            ("B".into(), "G".into()),
            ("G".into(), "H".into()),
            ("D".into(), "I".into()),
            ("E".into(), "J".into()),
            ("J".into(), "K".into()),
            ("K".into(), "L".into()),
        ];

        let orbits = get_number_of_orbits(input);

        assert_eq!(orbits, 42)
    }
}

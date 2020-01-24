use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "./day6/resources/input";
    let string = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let orbit_points: Vec<(String, String)> = string.lines().map(parse_line).collect();

    let orbit_points = sort(input);
    let number = get_number_of_orbits(orbit_points.copy());
    println!("number of orbits {}", number);

    let (number_of_swaps, path) = get_path_to_santa(orbit_points);

    println!("number of swaps {} along path {}", number_of_swaps, path);
}

fn parse_line(line: &str) -> (String, String) {
    let split: Vec<&str> = line.split(')').collect();

    let one: String = (*split
        .get(0)
        .expect(format!("parsing error at {}", line).as_str()))
    .into();
    let two: String = (*split
        .get(1)
        .expect(format!("parsing error at {}", line).as_str()))
    .into();

    (one, two)
}

fn get_path_to_santa(orbit_points: Vec<(String, String)>) -> (usize, Vec<String>) {}

fn insert_into_map(
    mut map: HashMap<String, usize>,
    orbit_point: String,
    new_planet: String,
) -> HashMap<String, usize> {
    let &length_until_now = map
        .get(&orbit_point)
        .expect(format!("orbit point {} not defined", orbit_point).as_ref());

    map.insert(new_planet, length_until_now + 1);
    map
}

fn get_number_of_orbits(sorted_orbit_points: Vec<(String, String)>) -> usize {
    let mut input = HashMap::new();
    input.insert("COM".into(), 0);

    sorted_orbit_points
        .into_iter()
        .fold(input, |acc, (orbit_point, new_planet)| {
            insert_into_map(acc, orbit_point, new_planet)
        })
        .values()
        .sum()
}

fn sort(unsorted_vec: Vec<(String, String)>) -> Vec<(String, String)> {
    let (mut sorted, mut unsorted): (Vec<(String, String)>, Vec<(String, String)>) = unsorted_vec
        .into_iter()
        .partition(|(orb, planet)| orb == "COM");

    loop {
        let (mut known_batch, unknown): (Vec<(String, String)>, Vec<(String, String)>) = unsorted
            .into_iter()
            .partition(|(orb, _)| sorted.iter().any(|(_, planet)| orb == planet));

        unsorted = unknown;
        let known_batch_is_empty = known_batch.is_empty();
        sorted.append(&mut known_batch);

        if unsorted.is_empty() {
            break;
        } else {
            assert!(
                !known_batch_is_empty,
                format!("{:?} could not be sorted into {:?}", unsorted, sorted)
            )
        }
    }
    sorted
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

    mod sort {
        use super::*;

        #[test]
        fn test_should_add_com_at_beginning() {
            // given
            let unsorted: Vec<(String, String)> = vec![
                ("COM".into(), "B".into()),
                ("B".into(), "C".into()),
                ("COM".into(), "A".into()),
            ];

            // when
            let sorted = sort(unsorted);

            assert_eq!(
                vec![
                    ("COM".into(), "B".into()),
                    ("COM".into(), "A".into()),
                    ("B".into(), "C".into())
                ],
                sorted
            )
        }

        #[test]
        fn test_should_not_alter_original() {
            // given
            let unsorted: Vec<(String, String)> = vec![
                ("COM".into(), "B".into()),
                ("B".into(), "C".into()),
                ("C".into(), "D".into()),
            ];

            // when
            let sorted = sort(unsorted.clone());

            assert_eq!(sorted, unsorted)
        }

        #[test]
        fn test_should_sort_by_known_planet() {
            // given
            let unsorted: Vec<(String, String)> = vec![
                ("C".into(), "D".into()),
                ("COM".into(), "B".into()),
                ("B".into(), "C".into()),
            ];

            // when
            let sorted = sort(unsorted.clone());

            assert_eq!(
                sorted,
                vec![
                    ("COM".into(), "B".into()),
                    ("B".into(), "C".into()),
                    ("C".into(), "D".into())
                ]
            )
        }
    }

    #[test]
    fn test_number_of_orbits_shuffled() {
        let input: Vec<(String, String)> = vec![
            ("C".into(), "D".into()),
            ("E".into(), "J".into()),
            ("D".into(), "I".into()),
            ("B".into(), "G".into()),
            ("E".into(), "F".into()),
            ("J".into(), "K".into()),
            ("COM".into(), "B".into()),
            ("K".into(), "L".into()),
            ("B".into(), "C".into()),
            ("D".into(), "E".into()),
            ("G".into(), "H".into()),
        ];

        let orbit_points = sort(input);
        let orbits = get_number_of_orbits(orbit_points);

        assert_eq!(orbits, 42)
    }

    #[test]
    fn test_get_path_to_santa() {
        let input = vec![
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
            ("K".into(), "YOU".into()),
            ("I".into(), "SAN".into()),
        ];

        let (number, path) = get_path_to_santa(sort(input));

        assert_eq!(number, 4);
        assert_eq!(
            path,
            vec!["K".into(), "J".into(), "E".into(), "D".into(), "I".into()]
        );
    }
}

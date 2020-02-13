use generic_array::typenum::{U25, U6};
use generic_array::{ArrayLength, GenericArray};
use itertools::{izip, zip, Itertools};
use std::convert::TryFrom;
use std::fs;
use std::iter::FromIterator;
use std::marker::PhantomData;
use generic_array::sequence::GenericSequence;

fn main() {
    let filename = "./day8/resources/input";
    let data: Vec<usize> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(usize::try_from)
                .expect(format!("parsing error with {}", c).as_ref())
                .expect("conversion error")
        })
        .collect();

    let image: Image<U25, U6> = Raw::<U25, U6> {
        data,
        width: PhantomData,
        height: PhantomData,
    }
    .into();

    println!("checksum is {}", image.checksum());

    for layer in image.merge_layers() {

        println!("{:?}", layer)
    }
}

struct Raw<WIDTH, HEIGHT> {
    width: PhantomData<WIDTH>,
    height: PhantomData<HEIGHT>,
    data: Vec<usize>,
}

type Lines<WIDTH, HEIGHT> = GenericArray<GenericArray<usize, WIDTH>, HEIGHT>;

#[derive(Debug, PartialEq)]
struct Layer<WIDTH, HEIGHT>
where
    WIDTH: ArrayLength<usize>,
    HEIGHT: ArrayLength<GenericArray<usize, WIDTH>>,
{
    lines: Lines<WIDTH, HEIGHT>,
}

#[derive(Debug, PartialEq)]
struct Image<WIDTH, HEIGHT>
where
    WIDTH: ArrayLength<usize>,
    HEIGHT: ArrayLength<GenericArray<usize, WIDTH>>,
{
    layers: Vec<Layer<WIDTH, HEIGHT>>,
}

impl<WIDTH, HEIGHT> From<Raw<WIDTH, HEIGHT>> for Image<WIDTH, HEIGHT>
where
    WIDTH: ArrayLength<usize>,
    HEIGHT: ArrayLength<GenericArray<usize, WIDTH>>,
{
    fn from(raw: Raw<WIDTH, HEIGHT>) -> Self {
        let image_size = WIDTH::to_usize() * HEIGHT::to_usize();
        assert_eq!(
            raw.data.len() % image_size,
            0,
            "cannot convert to image with non integer number of layers"
        );

        Image {
            layers: raw
                .data
                .chunks(image_size)
                .map(|chunk| get_layer::<WIDTH, HEIGHT>(chunk))
                .map(|layer| Layer { lines: layer })
                .collect(),
        }
    }
}

fn get_layer<WIDTH, HEIGHT>(chunk: &[usize]) -> Lines<WIDTH, HEIGHT>
where
    WIDTH: ArrayLength<usize>,
    HEIGHT: ArrayLength<GenericArray<usize, WIDTH>>,
{
    GenericArray::from_iter(
        chunk
            .chunks(WIDTH::to_usize())
            .map(|slice| slice.to_vec())
            .map(|slice| GenericArray::from_iter(slice)),
    )
}

impl<WIDTH, HEIGHT> Image<WIDTH, HEIGHT>
where
    WIDTH: ArrayLength<usize>,
    HEIGHT: ArrayLength<GenericArray<usize, WIDTH>>,
{
    fn checksum(self: &Self) -> usize {
        let min_layer = self
            .layers
            .iter()
            .min_by(|&l1, &l2| {
                l1.number_of_zeros()
                    .partial_cmp(&l2.number_of_zeros())
                    .unwrap()
            })
            .unwrap();

        let (ones_iter, twos_iter) = min_layer.lines.iter().flat_map(|line| line.iter()).tee();

        let nr_ones = ones_iter.filter(|&&a| a == 1).count();
        let nr_twos = twos_iter.filter(|&&a| a == 2).count();

        nr_ones * nr_twos
    }

    fn merge_layers(self: Self) -> Lines<WIDTH, HEIGHT> {
        let base: Lines<WIDTH, HEIGHT> = GenericArray::generate(|_| GenericArray::generate(|_| 2));

        izip!(self.layers).fold(base, |old, new|
            merge_lines::<WIDTH, HEIGHT>(old, new.lines)
        )
    }
}
fn merge_lines<WIDTH, HEIGHT>(old: Lines<WIDTH, HEIGHT>, new: Lines<WIDTH, HEIGHT>) -> Lines<WIDTH, HEIGHT>
where
    WIDTH: ArrayLength<usize>,
    HEIGHT: ArrayLength<GenericArray<usize, WIDTH>>,
{
    GenericArray::from_iter(old.iter().zip(new.iter()).map(|(old_line, new_line)| {
        GenericArray::from_iter(
            old_line
                .iter()
                .zip(new_line.iter())
                .map(|(&a, &b)| transparency(a, b)),
        )
    }))
}

fn transparency(old_px: usize, new_px: usize) -> usize {
    if old_px == 2 {
        new_px
    } else {
        old_px
    }
}

impl<WIDTH, HEIGHT> Layer<WIDTH, HEIGHT>
where
    WIDTH: ArrayLength<usize>,
    HEIGHT: ArrayLength<GenericArray<usize, WIDTH>>,
{
    fn number_of_zeros(self: &Self) -> usize {
        self.lines
            .iter()
            .flat_map(|line| line.iter())
            .filter(|&&s| s == 0)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use generic_array::arr;
    use generic_array::typenum::{U2, U3};

    #[test]
    fn should_convert_online_data() {
        // given
        let raw = Raw::<U3, U2> {
            data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
            width: PhantomData,
            height: PhantomData,
        };

        // when
        let image: Image<U3, U2> = raw.into();

        // then
        let a1 = arr![usize; 1, 2, 3];
        let a2 = arr![usize;4, 5, 6];
        let a3 = arr![usize;7, 8, 9];
        let a4 = arr![usize;0, 1, 2];
        assert_eq!(
            image,
            Image {
                layers: vec![
                    Layer {
                        lines: arr![GenericArray<usize, U3>; a1, a2]
                    },
                    Layer {
                        lines: arr![GenericArray<usize, U3>; a3, a4]
                    }
                ]
            }
        )
    }

    #[test]
    fn should_compute_checksum() {
        // given
        let image: Image<U3, U2> = Raw::<U3, U2> {
            data: vec![1, 2, 0, 0, 5, 6, 2, 1, 6, 0, 1, 2],
            width: PhantomData,
            height: PhantomData,
        }
        .into();

        // when
        let checksum = image.checksum();

        // then
        assert_eq!(checksum, 4)
    }

    #[test]
    fn should_reduce_correctly() {
        // given
        let image: Image<U2, U2> = Raw::<U2, U2> {
            data: vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0],
            width: PhantomData,
            height: PhantomData,
        }
        .into();

        // when
        let merged = image.merge_layers();

        // then
        assert_eq!(
            merged,
            arr![GenericArray<usize, U2>; arr![usize; 0,1], arr![usize; 1, 0]]
        )
    }

    #[test]
    fn test_transparency() {
        assert_eq!(transparency(0, 1), 0);
        assert_eq!(transparency(2, 1), 1);
        assert_eq!(transparency(1, 2), 1);
        assert_eq!(transparency(2, 0), 0);
    }


    #[test]
    fn test_merge_two_lines() {
        // given
        let top = arr![GenericArray<usize, U2>; arr![usize; 0,2], arr![usize; 1, 2]];
        let bot = arr![GenericArray<usize, U2>; arr![usize; 2,1], arr![usize; 0, 0]];

        // when
        let merged = merge_lines(top, bot);

        // then
        assert_eq!(
            merged,
            arr![GenericArray<usize, U2>; arr![usize; 0,1], arr![usize; 1, 0]]
        )
    }
}

use crate::solution::Solution;
use std::ops::Range;

pub struct Day5;

#[derive(Debug)]
struct MapRange {
    range: Range<u64>,
    dst_start: u64,
}

type Map = Vec<MapRange>;
pub struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl std::str::FromStr for MapRange {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<u64> = s.split(' ').map(|n| n.parse().unwrap()).collect();

        Ok(MapRange {
            range: v[1]..(v[1] + v[2]),
            dst_start: v[0],
        })
    }
}

fn get_dst_from_map(m: &Map, src: u64) -> u64 {
    let index = match m.binary_search_by_key(&src, |r| r.range.start) {
        Ok(index) => Some(index),
        Err(index) => index.checked_sub(1),
    };

    if let Some(index) = index {
        let range = &m[index];
        if src < range.range.end {
            return range.dst_start + (src - range.range.start);
        }
    }

    src
}

impl Solution for Day5 {
    type Input = Almanac;
    type ReturnType = u64;
    const DAY: u32 = 5;

    /* We represent a Map as a list of ranges that needs in which we do a binary search of the src. so these ranges needs to be sorted  */
    fn parse_input(&self, mut lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let seeds_str: String = lines.by_ref().take(1).collect();
        let (_, numbers) = seeds_str.split_once(':').unwrap();
        let seeds: Vec<u64> = numbers
            .trim()
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect();

        /* Skipping the empty line after the seeds */
        let _: String = lines.by_ref().take_while(|l| !l.is_empty()).collect();

        let mut maps: Vec<Map> = Vec::new();
        while let Some(_s) = lines.next() {
            let mut val: Map = lines
                .by_ref()
                .take_while(|l| !l.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            val.sort_by_key(|m| m.range.start);
            maps.push(val);
        }

        Almanac {
            seeds: seeds,
            maps: maps,
        }
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input
            .seeds
            .iter()
            .map(|&seed| input.maps.iter().fold(seed, |n, m| get_dst_from_map(m, n)))
            .min()
            .unwrap_or_default()
    }
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        input
            .seeds
            .chunks(2)
            .flat_map(|c| {
                (c[0]..(c[0] + c[1]))
                    .map(|seed| input.maps.iter().fold(seed, |n, m| get_dst_from_map(m, n)))
                    .min()
            })
            .min()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::get_dst_from_map;
    use super::Day5;
    use super::Map;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_map() {
        let data = "50 98 2
52 50 48";
        let m: Map = data.split('\n').map(|line| line.parse().unwrap()).collect();
        let seed = 79;
        let soil = get_dst_from_map(&m, seed);
        assert_eq!(soil, 81);
    }

    #[test]
    fn test_first_seed() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day5.parse_input(lines);

        let seed: u64 = *input.seeds.first().unwrap();
        let location = input.maps.iter().fold(seed, |n, m| get_dst_from_map(m, n));

        assert_eq!(location, 82);
    }
    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day5.parse_input(lines);
        assert_eq!(Day5.first_part(&input), 35)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day5.parse_input(lines);
        assert_eq!(Day5.second_part(&input), 46)
    }
}

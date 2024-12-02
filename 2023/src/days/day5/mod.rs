use std::collections::HashMap;
use crate::common::input_reader;

mod tests;
pub struct Day;

struct Seed {
    number: i64,
    soil: i64,
    fertilizer: i64,
    water: i64,
    light: i64,
    temperature: i64,
    humidity: i64,
    location: i64
}

struct SectionLocation {
    lines: Vec<String>
}

#[derive(Clone)]
struct PositionMap {
    start: i64,
    end: i64,
    offset: i64,
    dest_start: i64,
    dest_end: i64,
    count: i64
}

struct SeedRange {
    start: i64,
    end: i64
}

fn get_position(default: i64, maps: Vec<PositionMap> ) -> i64 {
    for m in maps {
        if default >= m.start && default < m.end {
            return default + m.offset;
        }
    }
    default
}
struct Almanac {
    seeds: Vec<i64>,
    seed_ranges: Vec<SeedRange>,
    seed_to_soil: Vec<PositionMap>,
    soil_to_fertilizer: Vec<PositionMap>,
    fertilizer_to_water: Vec<PositionMap>,
    water_to_light: Vec<PositionMap>,
    light_to_temperature: Vec<PositionMap>,
    temperature_to_humidity: Vec<PositionMap>,
    humidity_to_location: Vec<PositionMap>
}

fn lines_to_map(lines: Vec<String>) -> Vec<PositionMap> {
    let mut map = Vec::new();
    for l in lines {
        if l.trim().is_empty() {
            continue;
        }
        let parts = l.trim().split(' ').collect::<Vec<&str>>();
        if parts.len() != 3 {
            println!("line: {l}");
            panic!("line had wrong format");
        }

        let src = parts[1].trim().parse::<i64>().unwrap();
        let dest = parts[0].trim().parse::<i64>().unwrap();
        let cnt = parts[2].trim().parse::<i64>().unwrap();

        map.push(PositionMap {
            start: src,
            end: src+cnt,
            offset: dest-src,
            count: cnt,
            dest_start: dest,
            dest_end: dest+cnt
        })
    }
    map
}

fn extract_map_sections(all_lines: Vec<&str>) -> HashMap<String, SectionLocation> {
    let mut sections = HashMap::new();

    let mut name = "".to_string();
    let mut lines: Vec<String> = Vec::new();

    for l in all_lines {
        if l.trim().ends_with("map:") {
            if !name.is_empty() {
                sections.insert(name,SectionLocation {
                    lines
                });
            }
            name = l.replace(" map:", "").trim().to_string();
            lines = Vec::new();
        }
        else if !l.is_empty() {
            lines.push(l.to_string());
        }
    }

    if !name.is_empty() {
        sections.insert(name, SectionLocation {
            lines
        });
    }

    sections
}

impl Almanac {
    pub fn new(lines: Vec<&str>) -> Self {
        let mut seeds = Vec::new();
        for s in lines[0].replace("seeds: ", "").trim().split(' ') {
            if s.trim().is_empty() {
                continue;
            }
            seeds.push(s.trim().parse().unwrap());
        }

        let mut n = 0;
        let mut seed_ranges = Vec::new();
        while n < seeds.len() {
            seed_ranges.push(SeedRange {
                start: seeds[n],
                end: seeds[n] + seeds[n+1]
            });
            n += 2;
        }

        let sections = extract_map_sections(lines);
        let seed_to_soil= lines_to_map(sections["seed-to-soil"].lines.clone());
        let soil_to_fertilizer = lines_to_map(sections["soil-to-fertilizer"].lines.clone());
        let fertilizer_to_water = lines_to_map(sections["fertilizer-to-water"].lines.clone());
        let water_to_light = lines_to_map(sections["water-to-light"].lines.clone());
        let light_to_temperature = lines_to_map(sections["light-to-temperature"].lines.clone());
        let temperature_to_humidity = lines_to_map(sections["temperature-to-humidity"].lines.clone());
        let humidity_to_location= lines_to_map(sections["humidity-to-location"].lines.clone());

        Self {
            seeds,
            seed_ranges,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location
        }
    }
    pub fn get_location_for_seed(&self, seed: i64) -> i64 {
        let soil = get_position(seed, self.seed_to_soil.to_vec());
        let fertilizer = get_position(soil, self.soil_to_fertilizer.to_vec());
        let water = get_position(fertilizer, self.fertilizer_to_water.to_vec());
        let light = get_position(water, self.water_to_light.to_vec());
        let temperature = get_position(light, self.light_to_temperature.to_vec());
        let humidity = get_position(temperature, self.temperature_to_humidity.to_vec());
        let location = get_position(humidity, self.humidity_to_location.to_vec());

        location
    }

    fn walk_back(&self, from: i64, maps: &Vec<PositionMap>) -> i64 {
        for m in maps {
            if from >= m.dest_start && from < m.dest_end {
                //println!("walking {0} -> {1}", from, from + -m.offset);
                return from + -m.offset;
            }
        }

        //println!("walking {0} -> {1}", from, from);
        return from;
    }

    fn seed_exists(&self, seed: i64) -> bool {
        for r in &self.seed_ranges {
            if seed >= r.start && seed < r.end {
                return true;
            }
        }
        false
    }

    fn can_walk_back_to_seed_from_location(&self, loc: i64) -> bool {
        //print!("humidity: ");
        let humidity = self.walk_back(loc, &self.humidity_to_location);
        if humidity == -1 { return false; }

        //print!("temp: ");
        let temperature = self.walk_back(humidity, &self.temperature_to_humidity);
        if temperature == -1 { return false; }

        //print!("light: ");
        let light = self.walk_back(temperature, &self.light_to_temperature);
        if light == -1 { return false; }

        //print!("water: ");
        let water = self.walk_back(light, &self.water_to_light);
        if water == -1 { return false; }

        //print!("fertilizer: ");
        let fertilizer = self.walk_back(water, &self.fertilizer_to_water);
        if fertilizer == -1 { return false; }

        let soil = self.walk_back(fertilizer, &self.soil_to_fertilizer);
        if soil == -1 { return false; }

        let seed = self.walk_back(soil, &self.seed_to_soil);
        //println!("seed: {seed}");
        self.seed_exists(seed)
    }

    pub fn get_lowest_reachable_location(&self) -> i64 {
        let mut loc = 0;
        loop {
            if loc % 1000000 == 0 {
                println!("checking {loc}");
            }
            if self.can_walk_back_to_seed_from_location(loc) {
                return loc;
            }
            loc += 1;
        }
    }
}

impl Day {
    pub fn run(&self) {
        self.run_part_two();
    }
    pub fn run_part_one(&self) {
        let reader = input_reader::InputReader;
        let lines = reader.get_lines("/git/aoc23/src/days/day5/input");
        let a = Almanac::new(lines.iter().map(|x| x.as_str()).collect());
        let mut smallest = 999999999999;
        for seed in a.seeds.clone() {
            let loc = a.get_location_for_seed(seed);
            //println!("seed {seed} -> {loc}");
            if loc < smallest {
                smallest = loc;
            }
        }

        println!("{smallest}");
    }



    pub fn run_part_two(&self) {
        let reader = input_reader::InputReader;
        let lines = reader.get_lines("/git/aoc23/src/days/day5/input");
        let a = Almanac::new(lines.iter().map(|x| x.as_str()).collect());
        let answer = a.get_lowest_reachable_location();
        println!("{answer}");
    }
}
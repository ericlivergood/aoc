mod tests;
pub struct Day;

fn get_distance(race_duration: i64, charge_time: i64) -> i64 {
    let move_speed = charge_time;
    let move_duration = race_duration - charge_time;
    move_duration * move_speed
}

struct Race {
    time: i64,
    record: i64
}

impl Day {
    pub fn run(&self) {
        self.run_part_two();
    }
    pub fn run_part_one(&self) {
        let races = vec![
            // Race { time: 7, record: 9},
            // Race { time: 15, record: 40},
            // Race { time: 30, record: 200},

            Race { time: 47, record: 400},
            Race { time: 98, record: 1213},
            Race { time: 66, record: 1011},
            Race { time: 98, record: 1540},
        ];
        let mut answer = 1;
        for race in races {
            let mut win_count = 0;
            for i in 0..race.time {
                if get_distance(race.time, i) > race.record {
                    win_count += 1;
                }
            }
            answer = answer * win_count;
        }

        println!("{answer}");
    }

    pub fn run_part_two(&self) {
        // let race = Race {
        //     time: 71530,
        //     record: 940200
        // };

        let race = Race {
            time: 47_986_698,
            record: 400121310111540
        };

        let mut win_count = 0;
        for i in 0..race.time {
            if get_distance(race.time, i) > race.record {
                win_count += 1;
            }
            if i % 1_000_000 == 0 {
                println!("{i}");
            }
        }
        println!("{win_count}");
    }
}
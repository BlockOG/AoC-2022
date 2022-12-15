use std::collections::HashSet;

use crate::days;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Pos { x, y }
    }

    fn distance(&self, other: &Pos) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn tuning_frequency(&self) -> i64 {
        self.x * 4000000 + self.y
    }
}

pub struct Sensor {
    pos: Pos,
    beacon_distance: u64,
}

impl Sensor {
    fn new(pos: Pos, closest_beacon: Pos) -> Self {
        Sensor {
            pos,
            beacon_distance: pos.distance(&closest_beacon),
        }
    }

    fn get_x_skip_enter(&self, pos: &Pos) -> i64 {
        let y_diff = self.pos.y.abs_diff(pos.y);
        if self.beacon_distance < y_diff {
            i64::MAX
        } else {
            self.pos.x - self.beacon_distance as i64 + y_diff as i64
        }
    }

    fn get_x_skip_exit(&self, pos: &Pos) -> i64 {
        self.pos.x + self.beacon_distance as i64 - self.pos.y.abs_diff(pos.y) as i64 + 1
    }

    fn is_in_distance(&self, pos: &Pos) -> bool {
        self.pos.distance(pos) <= self.beacon_distance
    }
}

pub struct Day {}

impl days::Day for Day {
    type Input = (Vec<Sensor>, i64, i64, i64, i64, u64, u32);

    fn get_num(&self) -> u8 {
        15
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> String {
        let sensors = &input.0;
        let min_x = input.1 - input.5 as i64;
        let max_x = input.3 + input.5 as i64;
        let mut isnt_in = 0;

        let mut x = min_x;
        while x <= max_x {
            let pos = Pos::new(x, 2000000);

            let mut skip_x = x + 1;
            let mut in_range = false;

            for sensor in sensors.iter() {
                if sensor.is_in_distance(&pos) {
                    skip_x = sensor.get_x_skip_exit(&pos);
                    in_range = true;
                    break;
                }
            }

            if !in_range {
                let mut enter_x = i64::MAX - 10;
                for sensor in sensors.iter() {
                    let enter = sensor.get_x_skip_enter(&pos);
                    if enter >= skip_x && enter < enter_x {
                        enter_x = enter;
                    }
                }

                skip_x = enter_x;
            }

            let dist = skip_x - x;
            if dist <= 0 {
                panic!("What did you do with the input?");
            }

            if in_range {
                isnt_in += dist;
            }
            x = skip_x;
        }

        isnt_in -= input.6 as i64;

        isnt_in.to_string()
    }

    fn part2(&mut self, input: &Self::Input) -> String {
        let sensors = &input.0;

        for (i, sensor1) in sensors.iter().enumerate() {
            for sensor2 in sensors[i + 1..].iter() {
                let empty_space = sensor1.pos.distance(&sensor2.pos) as i64
                    - (sensor1.beacon_distance + sensor2.beacon_distance) as i64;
                if empty_space > 0 && empty_space <= 2 {
                    // It should somewhere between the 2 scanners

                    let ymin = (sensor1.pos.y - sensor1.beacon_distance as i64 - 2)
                        .max(sensor2.pos.y - sensor2.beacon_distance as i64 - 2);
                    let ymax = (sensor1.pos.y + sensor1.beacon_distance as i64 + 2)
                        .min(sensor2.pos.y + sensor2.beacon_distance as i64 + 2);

                    for y in ymin..ymax {
                        let pos = Pos::new(0, y);

                        let start_x = (sensor1.get_x_skip_enter(&pos) - 1)
                            .max(sensor2.get_x_skip_enter(&pos) - 1);
                        let stop_x = sensor1
                            .get_x_skip_exit(&pos)
                            .min(sensor2.get_x_skip_exit(&pos));

                        for x in start_x..=stop_x {
                            let pos = Pos::new(x, y);
                            if !sensors.iter().any(|f| f.is_in_distance(&pos)) {
                                return pos.tuning_frequency().to_string();
                            }
                        }
                    }
                }
            }
        }

        "What did you do with the input?".to_string()
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        let mut sensors = vec![];

        let mut min_x = i64::MAX;
        let mut min_y = i64::MAX;

        let mut max_x = i64::MIN;
        let mut max_y = i64::MIN;

        let mut max_distance = u64::MIN;
        let mut beacons_on_2000000 = HashSet::new();

        for line in input.lines() {
            let mut split = line.split(": ");
            let sensor_pos;
            let beacon_pos;
            {
                let mut split = split.next().unwrap().split("=");
                split.next();
                let x = split.next().unwrap().replace(", y", "").parse().unwrap();
                let y = split.next().unwrap().parse().unwrap();
                sensor_pos = Pos::new(x, y);

                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
            {
                let mut split = split.next().unwrap().split("=");
                split.next();
                let x = split.next().unwrap().replace(", y", "").parse().unwrap();
                let y = split.next().unwrap().parse().unwrap();
                beacon_pos = Pos::new(x, y);

                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);

                if y == 2000000 {
                    beacons_on_2000000.insert(x);
                }
            }
            max_distance = max_distance.max(sensor_pos.distance(&beacon_pos));
            sensors.push(Sensor::new(sensor_pos, beacon_pos));
        }

        (
            sensors,
            min_x,
            min_y,
            max_x,
            max_y,
            max_distance,
            beacons_on_2000000.len() as u32,
        )
    }
}

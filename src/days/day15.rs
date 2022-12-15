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
    closest_beacon: Pos,
    beacon_distance: u64,
}

impl Sensor {
    fn new(pos: Pos, closest_beacon: Pos) -> Self {
        Sensor {
            pos,
            closest_beacon,
            beacon_distance: pos.distance(&closest_beacon),
        }
    }
}

pub struct Day {}

impl days::Day for Day {
    type Input = (Vec<Sensor>, i64, i64, i64, i64, u64);

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

        for x in min_x..max_x {
            let mut broke = false;
            for sensor in sensors.iter() {
                if (sensor.closest_beacon.x == x && sensor.closest_beacon.y == 2000000)
                    || (sensor.pos.x == x && sensor.pos.y == 2000000)
                {
                    broke = true;
                    break;
                }
            }
            if broke {
                continue;
            }
            for sensor in sensors.iter() {
                if sensor.pos.distance(&Pos::new(x, 2000000)) <= sensor.beacon_distance {
                    isnt_in += 1;
                    break;
                }
            }
        }

        isnt_in.to_string()
    }

    fn part2(&mut self, input: &Self::Input) -> String {
        let sensors = &input.0;

        for sensor in sensors.iter() {
            let mut x1 = sensor.pos.x - sensor.beacon_distance as i64 - 1;
            let mut x2 = sensor.pos.x + sensor.beacon_distance as i64 + 1;
            let mut y1 = sensor.pos.y;
            let mut y2 = sensor.pos.y;

            while x1 <= sensor.pos.x {
                let mut x1y1 = true;
                let mut x1y2 = true;
                let mut x2y1 = true;
                let mut x2y2 = true;

                if x1 < 0 || x1 > 4000000 {
                    x1y1 = false;
                    x1y2 = false;
                }

                if x2 < 0 || x2 > 4000000 {
                    x2y1 = false;
                    x2y2 = false;
                }

                if y1 < 0 || y1 > 4000000 {
                    x1y1 = false;
                    x2y1 = false;
                }

                if y2 < 0 || y2 > 4000000 {
                    x1y2 = false;
                    x2y2 = false;
                }

                let pos1 = Pos::new(x1, y1);
                let pos2 = Pos::new(x1, y2);
                let pos3 = Pos::new(x2, y1);
                let pos4 = Pos::new(x2, y2);

                for sensor2 in sensors.iter() {
                    if sensor2.beacon_distance >= sensor2.pos.distance(&pos1) {
                        x1y1 = false;
                    }
                    if sensor2.beacon_distance >= sensor2.pos.distance(&pos2) {
                        x1y2 = false;
                    }
                    if sensor2.beacon_distance >= sensor2.pos.distance(&pos3) {
                        x2y1 = false;
                    }
                    if sensor2.beacon_distance >= sensor2.pos.distance(&pos4) {
                        x2y2 = false;
                    }
                    if !x1y1 && !x1y2 && !x2y1 && !x2y2 {
                        break;
                    }
                }
                if x1y1 {
                    return pos1.tuning_frequency().to_string();
                }
                if x1y2 {
                    return pos2.tuning_frequency().to_string();
                }
                if x2y1 {
                    return pos3.tuning_frequency().to_string();
                }
                if x2y2 {
                    return pos4.tuning_frequency().to_string();
                }
                x1 += 1;
                x2 -= 1;
                y1 += 1;
                y2 -= 1;
            }
        }

        "Failed".to_string()
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        let mut sensors = vec![];

        let mut min_x = i64::MAX;
        let mut min_y = i64::MAX;

        let mut max_x = i64::MIN;
        let mut max_y = i64::MIN;

        let mut max_distance = u64::MIN;

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
            }
            max_distance = max_distance.max(sensor_pos.distance(&beacon_pos));
            sensors.push(Sensor::new(sensor_pos, beacon_pos));
        }

        (sensors, min_x, min_y, max_x, max_y, max_distance)
    }
}

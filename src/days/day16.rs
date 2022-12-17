use crate::days;

fn choose_one<'a, T>(xs: &'a Vec<T>) -> impl Iterator<Item = (T, Vec<T>)> + 'a
where
    T: Clone,
{
    (0..xs.len()).map(move |i| {
        let mut ys = xs.clone();
        let element = ys.remove(i);
        (element, ys)
    })
}

use std::collections::HashMap;

fn dfs(
    flows: &Vec<u64>,
    dist: &Vec<Vec<u64>>,
    current_valve: u64,
    closed_valves: &Vec<u64>,
    time_remaining: u64,
    cache: &mut HashMap<(u64, Vec<u64>, u64), u64>,
) -> u64 {
    // Check if the result is already in the cache
    if let Some(result) = cache.get(&(current_valve, closed_valves.clone(), time_remaining)) {
        return *result;
    }

    let result = choose_one(closed_valves)
        .filter(|&(r, _)| dist[current_valve as usize][r as usize] < time_remaining)
        .map(|(r, rr)| {
            flows[r as usize] * (time_remaining - dist[current_valve as usize][r as usize] - 1)
                + dfs(
                flows,
                dist,
                r,
                &rr,
                time_remaining - dist[current_valve as usize][r as usize] - 1,
                cache,
                )
        })
        .max()
        .unwrap_or(0);

    // Add the result to the cache
    cache.insert((current_valve, closed_valves.clone(), time_remaining), result);

    result
}

fn dfs_part2(
    names: &Vec<String>,
    flows: &Vec<u64>,
    dist: &Vec<Vec<u64>>,
    current_valve: u64,
    closed_valves: &Vec<u64>,
    time_remaining: u64,
    cache: &mut HashMap<(u64, Vec<u64>, u64), u64>,
) -> u64 {
    choose_one(closed_valves)
        .filter(|&(r, _)| dist[current_valve as usize][r as usize] < time_remaining)
        .map(|(r, rr)| {
            flows[r as usize] * (time_remaining - dist[current_valve as usize][r as usize] - 1)
                + dfs_part2(
                names,
                flows,
                dist,
                r,
                &rr,
                time_remaining - dist[current_valve as usize][r as usize] - 1,
                cache,
                )
        })
        .max()
        .unwrap_or(dfs(
            flows,
            dist,
            names.iter().position(|x| x == &"AA".to_string()).unwrap() as u64,
            closed_valves,
            26,
            cache,
        ))
}

pub struct Day {
    part1_cache: HashMap<(u64, Vec<u64>, u64), u64>,
}

impl days::Day for Day {
    type Input = (Vec<String>, Vec<u64>, Vec<Vec<String>>, Vec<Vec<u64>>);

    fn get_num(&self) -> u8 {
        16
    }

    fn new() -> Self {
        Self {
            part1_cache: HashMap::new(),
        }
    }

    fn part1(&mut self, input: &Self::Input) -> String {
        dfs(
            &input.1,
            &input.3,
            input.0.iter().position(|x| x == &"AA".to_string()).unwrap() as u64,
            &input
                .1
                .iter()
                .enumerate()
                .filter(|(_, f)| **f > 0)
                .map(|(i, _)| i as u64)
                .collect(),
            30,
            &mut self.part1_cache,
        )
        .to_string()
    }

    fn part2(&mut self, input: &Self::Input) -> String {
        dfs_part2(
            &input.0,
            &input.1,
            &input.3,
            input.0.iter().position(|x| x == &"AA".to_string()).unwrap() as u64,
            &input
                .1
                .iter()
                .enumerate()
                .filter(|(_, f)| **f > 0)
                .map(|(i, _)| i as u64)
                .collect(),
            26,
            &mut self.part1_cache,
        )
        .to_string()
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        let mut names = vec![];
        let mut flow_rates = vec![];
        let mut connections = vec![];
        for line in input.lines() {
            let mut leads_to = Vec::new();
            for part in line.split(" ").skip(9) {
                if part.ends_with(",") {
                    leads_to.push(part[..part.len() - 1].to_string());
                } else {
                    leads_to.push(part.to_string());
                }
            }
            connections.push(leads_to);

            names.push(line.split(" ").nth(1).unwrap().to_string());

            flow_rates.push(
                line.split("=")
                    .nth(1)
                    .unwrap()
                    .split(";")
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap(),
            );
        }
        let mut dist = vec![vec![99; names.len()]; names.len()];
        for (i, cl) in connections.iter().enumerate() {
            for conn in cl.iter() {
                dist[i][names.iter().position(|x| x == conn).unwrap()] = 1;
            }
        }
        // Floyd-Warshall
        for k in 0..names.len() {
            for i in 0..names.len() {
                for j in 0..names.len() {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
        (names, flow_rates, connections, dist)
    }
}

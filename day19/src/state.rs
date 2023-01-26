use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Debug,
};

use crate::{BluePrint, BluePrintOpt, Clay, Obsidian, Ore};

#[derive(Eq, PartialEq, Default, Clone, PartialOrd, Ord, Hash)]
pub struct State<const TIME: usize = 24> {
    pub ore: Ore,
    pub clay: Clay,
    pub obsidian: Obsidian,
    pub geode: usize,
    pub time: usize,
    pub ore_robots: Ore,
    pub clay_robots: Clay,
    pub obsidian_robots: Obsidian,
    pub geode_robots: usize,
}

#[test]
fn state_returns_most_geodes0() {
    let bp = (&BluePrint {
        id: 1,
        ore: Ore(4),
        clay: Ore(2),
        obsidian: (Clay(14), Ore(3)),
        geode: (Ore(2), Obsidian(7)),
    })
        .into();
    let state: State<24> = State {
        time: 0,
        ore_robots: Ore(1),
        ..Default::default()
    };
    let result = dfs(state, &bp);
    assert_eq!(result, 9);
}

#[test]
fn state_returns_most_geodes14() {
    let bp = (&BluePrint {
        id: 1,
        ore: Ore(4),
        clay: Ore(2),
        obsidian: (Clay(14), Ore(3)),
        geode: (Ore(2), Obsidian(7)),
    })
        .into();
    let state: State<24> = State {
        ore: Ore(3),
        clay: Clay(15),
        obsidian: Obsidian(3),
        time: 14,
        geode: 0,
        ore_robots: Ore(1),
        clay_robots: Clay(4),
        obsidian_robots: Obsidian(1),
        ..Default::default()
    };
    let result = dfs(state, &bp);
    assert_eq!(result, 9);
}

#[test]
fn state_returns_most_geodes17() {
    let bp = (&BluePrint {
        id: 1,
        ore: Ore(4),
        clay: Ore(2),
        obsidian: (Clay(14), Ore(3)),
        geode: (Ore(2), Obsidian(7)),
    })
        .into();
    let state: State<24> = State {
        ore: Ore(3),
        clay: Clay(13),
        obsidian: Obsidian(8),
        time: 17,
        geode: 0,
        ore_robots: Ore(1),
        clay_robots: Clay(4),
        obsidian_robots: Obsidian(2),
        ..Default::default()
    };
    let result = dfs(state, &bp);
    assert_eq!(result, 9);
}

#[test]
fn state_returns_most_geodes19() {
    let bp = (&BluePrint {
        id: 1,
        ore: Ore(4),
        clay: Ore(2),
        obsidian: (Clay(14), Ore(3)),
        geode: (Ore(2), Obsidian(7)),
    })
        .into();
    let state: State<24> = State {
        ore: Ore(4),
        clay: Clay(21),
        obsidian: Obsidian(5),
        time: 19,
        geode: 6,
        ore_robots: Ore(1),
        clay_robots: Clay(4),
        obsidian_robots: Obsidian(2),
        ..Default::default()
    };
    let result = dfs(state, &bp);
    assert_eq!(result, 9);
}

#[test]
fn state_returns_most_geodes23() {
    let bp = (&BluePrint {
        id: 1,
        ore: Ore(4),
        clay: Ore(2),
        obsidian: (Clay(14), Ore(3)),
        geode: (Ore(2), Obsidian(7)),
    })
        .into();
    let state: State<24> = State {
        obsidian: Obsidian(6),
        ore: Ore(5),
        clay: Clay(41),
        time: 23,
        geode: 9,
        ore_robots: Ore(1),
        clay_robots: Clay(4),
        obsidian_robots: Obsidian(2),
        ..Default::default()
    };
    let result = dfs(state, &bp);
    assert_eq!(result, 9);
}

#[test]
fn state_returns_most_geodes32() {
    let bp = (&BluePrint {
        id: 1,
        ore: Ore(4),
        clay: Ore(2),
        obsidian: (Clay(14), Ore(3)),
        geode: (Ore(2), Obsidian(7)),
    })
        .into();
    let state: State<32> = State {
        obsidian: Obsidian(6),
        ore: Ore(5),
        clay: Clay(41),
        time: 32,
        geode: 56,
        ore_robots: Ore(1),
        clay_robots: Clay(4),
        obsidian_robots: Obsidian(2),
        ..Default::default()
    };
    let result = dfs(state, &bp);
    assert_eq!(result, 56);
}

impl<const T: usize> Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("ore", &self.ore.0)
            .field("clay", &self.clay.0)
            .field("obsidian", &self.obsidian.0)
            .field("geode", &self.geode)
            .field("time", &self.time)
            .field("ore_robots", &self.ore_robots.0)
            .field("clay_robots", &self.clay_robots.0)
            .field("obsidian_robots", &self.obsidian_robots.0)
            .field("geode_robots", &self.geode_robots)
            .finish()
    }
}

impl<const TIME: usize> State<TIME> {
    const TIME: usize = TIME;

    pub fn time_left(&self) -> usize {
        Self::TIME.saturating_sub(self.time)
    }

    pub fn could_build_ore(&self, bp: &BluePrintOpt) -> Option<usize> {
        let below_max = self.ore_robots < bp.max_ore;
        let below_limit =
            (self.time_left() * self.ore_robots.0 + self.ore.0) < self.time_left() * bp.max_ore.0;
        (below_max && below_limit).then(|| {
            let need = bp.ore.0.saturating_sub(self.ore.0);
            next_time(need, self.ore_robots.0, self.time_left())
        })?
    }

    pub fn could_build_clay(&self, bp: &BluePrintOpt) -> Option<usize> {
        let below_max = self.clay_robots < bp.max_clay;
        let below_limit = (self.time_left() * self.clay_robots.0 + self.clay.0)
            < self.time_left() * bp.max_clay.0;
        (below_max && below_limit).then(|| {
            let need = bp.clay.0.saturating_sub(self.ore.0);
            next_time(need, self.ore_robots.0, self.time_left())
        })?
    }

    pub fn could_build_obsidian(&self, bp: &BluePrintOpt) -> Option<usize> {
        let below_max = self.obsidian_robots < bp.geode.1;
        let below_limit = (self.time_left() * self.obsidian_robots.0 + self.obsidian.0)
            < self.time_left() * bp.max_obs.0;
        (below_max && below_limit).then(|| {
            let need_ore = bp.obsidian.1 .0.saturating_sub(self.ore.0);
            let need_clay = bp.obsidian.0 .0.saturating_sub(self.clay.0);
            let Some(ore_time) = next_time(need_ore, self.ore_robots.0, self.time_left()) else { return None};
            let Some(clay_time) = next_time(need_clay, self.clay_robots.0, self.time_left()) else { return None};
            Some(ore_time.max(clay_time))
        })?
    }

    pub fn could_build_geode(&self, bp: &BluePrintOpt) -> Option<usize> {
        (self.obsidian_robots > Obsidian(0)).then(|| {
            let need_ore = bp.geode.0 .0.saturating_sub(self.ore.0);
            let need_obsidian = bp.geode.1 .0.saturating_sub(self.obsidian.0);
            let Some(ore_time) = next_time(need_ore, self.ore_robots.0, self.time_left()) else { return None };
            let Some(obs_time) = next_time(need_obsidian, self.obsidian_robots.0, self.time_left()) else { return None };
            Some(ore_time.max(obs_time))
        })?
    }

    pub fn build_ore(mut self, bp: &BluePrintOpt) -> Self {
        self.ore -= bp.ore;
        self.ore_robots += Ore(1);
        self
    }

    pub fn build_clay(mut self, bp: &BluePrintOpt) -> Self {
        self.ore -= bp.clay;
        self.clay_robots += Clay(1);
        self
    }

    pub fn build_obsidian(mut self, bp: &BluePrintOpt) -> Self {
        self.ore -= bp.obsidian.1;
        self.clay -= bp.obsidian.0;
        self.obsidian_robots += Obsidian(1);
        self
    }

    pub fn build_geode(mut self, bp: &BluePrintOpt) -> Self {
        self.ore -= bp.geode.0;
        self.obsidian -= bp.geode.1;
        self.geode_robots += 1;
        self.geode += self.time_left();
        self
    }

    pub fn fork_at(mut self, time: usize) -> Self {
        self.time += time; // tick first for end of the time
        self.ore += self.ore_robots * time;
        self.clay += self.clay_robots * time;
        self.obsidian += self.obsidian_robots * time;
        self
    }

    pub fn optimise_skipping(self, bp: &BluePrintOpt) -> impl Iterator<Item = Self> {
        [
            self.could_build_geode(bp)
                .map(|time| self.clone().fork_at(time).build_geode(bp)),
            self.could_build_ore(bp)
                .map(|time| self.clone().fork_at(time).build_ore(bp)),
            self.could_build_clay(bp)
                .map(|time| self.clone().fork_at(time).build_clay(bp)),
            self.could_build_obsidian(bp)
                .map(|time| self.clone().fork_at(time).build_obsidian(bp)),
        ]
        .into_iter()
        .flatten()
    }
}

pub fn dfs<const N: usize>(state: State<N>, bp: &BluePrintOpt) -> usize {
    let mut work = BinaryHeap::from([state]);
    let mut seen = HashSet::new();
    let mut max = 0;

    while let Some(state) = work.pop() {
        if seen.insert(state.clone()) {
            max = max.max(state.geode);
            if state.time_left() <= 1 {
                continue;
            } // takes 1 time to make a robot so any robot won't be ready
            work.extend(state.optimise_skipping(bp));
        } else {
            continue;
        }
    }

    max
}

fn next_time(need: usize, robots: usize, time_left: usize) -> Option<usize> {
    if robots == 0 {
        return None;
    }
    let time = ((need.next_multiple_of(robots)) / (robots)) + 1;
    // can only create robot after
    // resources are collected

    (time_left >= time).then_some(time)
}

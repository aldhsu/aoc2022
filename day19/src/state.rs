use crate::{Ore, Obsidian, Clay, BluePrintOpt, BluePrint};

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub enum RobotKind {
    Obsidian,
    #[default]
    Ore,
    Geode,
    Clay,
}

#[derive(Debug, Eq, PartialEq, Default, Clone)]
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
    pub next_robot: Option<RobotKind>,
}

#[test]
fn state_returns_most_geodes() {
    let bp = (&BluePrint {
        id: 1,
        ore: Ore(1),
        clay: Ore(2),
        obsidian: (Clay(1), Ore(3)),
        geode: (Ore(3), Obsidian(2)),
    })
        .into();
    let state: State<24> = State {
        time: 24,
        geode: 24,
        ..Default::default()
    };

    assert_eq!(state.optimise(&bp), 24);
}

#[test]
fn state_returns_most_geodes1() {
    let bp = BluePrint {
        id: 1,
        ore: Ore(1),
        clay: Ore(2),
        obsidian: (Clay(1), Ore(3)),
        geode: (Ore(3), Obsidian(2)),
    };
    let state: State<24> = State {
        time: 23,
        geode: 24,
        ..Default::default()
    };
    let result = state.optimise(&(&bp).into());

    assert_eq!(result, 24);
}

#[test]
fn state_returns_most_geodes2() {
    let bp = BluePrint {
        id: 1,
        ore: Ore(1),
        clay: Ore(2),
        obsidian: (Clay(1), Ore(3)),
        geode: (Ore(3), Obsidian(2)),
    };
    let state: State<24> = State {
        time: 23,
        geode: 0,
        geode_robots: 1,
        ore_robots: Ore(1),
        clay_robots: Clay(1),
        obsidian_robots: Obsidian(1),
        ..Default::default()
    };
    let result = state.optimise(&(&bp).into());
    assert_eq!(result, 1);
}

#[test]
fn state_returns_most_geodes23() {
    let bp = BluePrint {
        id: 1,
        ore: Ore(4),
        clay: Ore(2),
        obsidian: (Clay(14), Ore(3)),
        geode: (Ore(2), Obsidian(7)),
    };
    let state: State<24> = State {
        obsidian: Obsidian(6),
        ore: Ore(5),
        clay: Clay(41),
        time: 23,
        geode: 7,
        geode_robots: 2,
        ore_robots: Ore(1),
        clay_robots: Clay(4),
        obsidian_robots: Obsidian(2),
        ..Default::default()
    };
    let result = state.optimise(&(&bp).into());
    assert_eq!(result, 9);
}

#[test]
fn state_returns_most_geodes22() {
    let bp = BluePrint {
        id: 1,
        ore: Ore(4),
        clay: Ore(2),
        obsidian: (Clay(14), Ore(3)),
        geode: (Ore(2), Obsidian(7)),
    };
    let state: State<24> = State {
        obsidian: Obsidian(4),
        ore: Ore(4),
        clay: Clay(33),
        time: 22,
        geode: 5,
        geode_robots: 2,
        ore_robots: Ore(1),
        clay_robots: Clay(4),
        obsidian_robots: Obsidian(2),
        ..Default::default()
    };
    let result = state.optimise(&(&bp).into());
    assert_eq!(result, 9);
}

#[test]
fn one_option_test() {
    let bp = BluePrint {
        id: 1,
        ore: Ore(100),
        clay: Ore(100),
        obsidian: (Clay(100), Ore(100)),
        geode: (Ore(23), Obsidian(1)),
    };
    let state: State<24> = State {
        time: 0,
        obsidian: Obsidian(1),
        ore: Ore(0),
        clay: Clay(0),
        geode: 0,
        geode_robots: 0,
        ore_robots: Ore(1),
        clay_robots: Clay(0),
        obsidian_robots: Obsidian(0),
        ..Default::default()
    };
    let result = state.optimise(&(&bp).into());
    assert_eq!(result, 1);
}

#[test]
fn state_returns_most_geodes21() {
    let bp = BluePrint {
        id: 1,
        ore: Ore(4),
        clay: Ore(2),
        obsidian: (Clay(14), Ore(3)),
        geode: (Ore(2), Obsidian(7)),
    };
    let state: State<24> = State {
        time: 21,
        ore: Ore(4),
        clay: Clay(25),
        obsidian: Obsidian(7),
        geode: 2,
        ore_robots: Ore(1),
        clay_robots: Clay(4),
        obsidian_robots: Obsidian(2),
        geode_robots: 1,
        ..Default::default()
    };
    let result = state.optimise(&(&bp).into());
    assert_eq!(result, 9);
}

impl<const TIME: usize> State<TIME>{
    const TIME: usize = TIME;

    pub fn time_left(&self) -> usize {
        Self::TIME - self.time
    }

    pub fn should_build_ore(&self, bp: &BluePrintOpt) -> bool {
        self.ore_robots < bp.max_ore
            && self.ore >= bp.ore
            && (self.ore_robots * self.time_left() + self.ore)
                < (bp.max_ore * self.time_left())
    }

    pub fn should_build_clay(&self, bp: &BluePrintOpt) -> bool {
        self.clay_robots < bp.max_clay
            && self.ore >= bp.clay
            && (self.clay_robots * self.time_left() + self.clay)
                < (bp.max_clay * self.time_left())
    }

    pub fn should_build_obsidian(&self, bp: &BluePrintOpt) -> bool {
        self.obsidian_robots < bp.max_obs
            && self.ore >= bp.obsidian.1
            && self.clay >= bp.obsidian.0
            && (self.obsidian_robots * self.time_left() + self.obsidian)
                < (bp.max_obs * self.time_left())
    }

    pub fn should_build_geode(&self, bp: &BluePrintOpt) -> bool {
        self.ore >= bp.geode.0 && self.obsidian >= bp.geode.1
    }

    pub fn build_ore(&self, bp: &BluePrintOpt) -> Self {
        let mut state = self.clone();
        state.ore -= bp.ore;
        state.next_robot = Some(RobotKind::Ore);
        state
    }

    pub fn build_clay(&self, bp: &BluePrintOpt) -> Self {
        let mut state = self.clone();
        state.ore -= bp.clay;
        state.next_robot = Some(RobotKind::Clay);
        state
    }

    pub fn build_obsidian(&self, bp: &BluePrintOpt) -> Self {
        let mut state = self.clone();
        state.ore -= bp.obsidian.1;
        state.clay -= bp.obsidian.0;
        state.next_robot = Some(RobotKind::Obsidian);
        state
    }

    pub fn build_geode(&self, bp: &BluePrintOpt) -> Self {
        let mut state = self.clone();
        state.ore -= bp.geode.0;
        state.obsidian -= bp.geode.1;
        state.next_robot = Some(RobotKind::Geode);
        state
    }

    pub fn move_time(&mut self, time: usize) {
        self.time += time; // tick first for end of the time
        self.ore += self.ore_robots * time;
        self.clay += self.clay_robots * time;
        self.obsidian += self.obsidian_robots * time;
        self.geode += self.geode_robots * time;
    }

    pub fn optimise(mut self, bp: &BluePrintOpt) -> usize {
        if self.time >= Self::TIME {
            return self.geode;
        }
        let mut local_max = self.geode;

        // new state with addition of all generated resources
        // keep bumping up until we have enough to do something
        // options
        // compare current max to old
        //
        self.time += 1; // tick first for end of the time
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;

        match self.next_robot.take() {
            Some(RobotKind::Ore) => self.ore_robots += Ore(1),
            Some(RobotKind::Clay) => self.clay_robots += Clay(1),
            Some(RobotKind::Obsidian) => self.obsidian_robots += Obsidian(1),
            Some(RobotKind::Geode) => self.geode_robots += 1,
            None => {}
        }

        if self.should_build_geode(bp) {
            let state = self.build_geode(bp);
            local_max = state.optimise(bp).max(local_max);

            return local_max;
        }

        if self.should_build_ore(bp) {
            let state = self.build_ore(bp);
            local_max = state.optimise(bp).max(local_max);
        }

        if self.should_build_clay(bp) {
            let state = self.build_clay(bp);
            local_max = state.optimise(bp).max(local_max);
        }

        if self.should_build_obsidian(bp) {
            let state = self.build_obsidian(bp);
            local_max = state.optimise(bp).max(local_max);
        }

        local_max = self.optimise(bp).max(local_max);

        local_max
    }
}

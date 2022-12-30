use crate::{Clay, Obsidian, Ore, State};

#[derive(Debug, Eq, PartialEq, Default)]
pub struct BluePrint {
    pub id: usize,
    pub ore: Ore,
    pub clay: Ore,
    pub obsidian: (Clay, Ore),
    pub geode: (Ore, Obsidian),
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct BluePrintOpt {
    pub id: usize,
    pub ore: Ore,
    pub clay: Ore,
    pub obsidian: (Clay, Ore),
    pub geode: (Ore, Obsidian),
    pub max_ore: Ore,
    pub max_clay: Clay,
    pub max_obs: Obsidian,
}

#[test]
fn blueprint_into_blueprint_opt_test() {
    let bp = BluePrint {
        id: 1,
        ore: Ore(1),
        clay: Ore(2),
        obsidian: (Clay(3), Ore(2)),
        geode: (Ore(4), Obsidian(3)),
    };

    let opt: BluePrintOpt = (&bp).into();
    assert_eq!(opt.max_ore, Ore(4));
    assert_eq!(opt.max_clay, Clay(3));
    assert_eq!(opt.max_obs, Obsidian(3));
}

impl From<&BluePrint> for BluePrintOpt {
    fn from(value: &BluePrint) -> Self {
        let max_ore = [value.ore, value.clay, value.obsidian.1, value.geode.0]
            .into_iter()
            .max()
            .expect("no max for ore");

        BluePrintOpt {
            max_ore,
            max_clay: value.obsidian.0,
            max_obs: value.geode.1,
            id: value.id,
            ore: value.ore,
            clay: value.clay,
            obsidian: value.obsidian,
            geode: value.geode,
        }
    }
}

#[test]
fn should_build_ore_test() {
    let bp = BluePrintOpt {
        max_ore: Ore(2),
        ore: Ore(1),
        ..Default::default()
    };
    let state: State<24> = State {
        ore: Ore(1),
        ore_robots: Ore(2),
        ..Default::default()
    };

    assert_eq!(state.should_build_ore(&bp), false);
    let state: State<24> = State {
        ore: Ore(1),
        ore_robots: Ore(1),
        ..Default::default()
    };

    assert_eq!(state.should_build_ore(&bp), true);
    let state: State<24> = State {
        ore: Ore(0),
        ore_robots: Ore(1),
        ..Default::default()
    };

    assert_eq!(state.should_build_ore(&bp), false);
}

#[test]
fn should_build_clay_test() {
    let bp = BluePrintOpt {
        max_clay: Clay(2),
        clay: Ore(1),
        ..Default::default()
    };
    let state: State<24> = State {
        ore: Ore(1),
        clay_robots: Clay(2),
        ..Default::default()
    };

    assert_eq!(state.should_build_clay(&bp), false);
    let state: State<24> = State {
        ore: Ore(1),
        clay_robots: Clay(1),
        ..Default::default()
    };

    assert_eq!(state.should_build_clay(&bp), true);
    let state: State<24> = State {
        ore: Ore(0),
        clay_robots: Clay(1),
        ..Default::default()
    };

    assert_eq!(state.should_build_clay(&bp), false);
}

#[test]
fn should_build_obsidian_test() {
    let bp = BluePrintOpt {
        max_obs: Obsidian(2),
        obsidian: (Clay(1), Ore(1)),
        ..Default::default()
    };
    let state: State<24> = State {
        ore: Ore(1),
        clay: Clay(1),
        obsidian_robots: Obsidian(2),
        ..Default::default()
    };

    assert_eq!(state.should_build_obsidian(&bp), false);
    let state: State<24> = State {
        ore: Ore(1),
        clay: Clay(1),
        ..Default::default()
    };

    assert_eq!(state.should_build_obsidian(&bp), true);
    let state: State<24> = State {
        ore: Ore(1),
        ..Default::default()
    };

    assert_eq!(state.should_build_obsidian(&bp), false);
}

#[test]
fn should_build_geode_test() {
    let bp = BluePrintOpt {
        geode: (Ore(1), Obsidian(1)),
        ..Default::default()
    };
    let state: State<24> = State {
        ore: Ore(1),
        obsidian: Obsidian(1),
        geode_robots: 2,
        ..Default::default()
    };

    assert_eq!(state.should_build_geode(&bp), true);
    let state: State<24> = State {
        ore: Ore(1),
        ..Default::default()
    };

    assert_eq!(state.should_build_obsidian(&bp), false);
}

impl BluePrintOpt {}

#[test]
fn blueprint_can_optimise() {
    let bp = BluePrint {
        id: 1,
        ore: Ore(4),
        clay: Ore(2),
        obsidian: (Clay(14), Ore(3)),
        geode: (Ore(2), Obsidian(7)),
    };

    assert_eq!(bp.optimise::<24>(), 9);
}

impl BluePrint {
    pub fn optimise<const N: usize>(&self) -> usize {
        let state: State<N> = State {
            ore_robots: Ore(1),
            ..Default::default()
        };
        let result = state.optimise(&(self.into()));
        result * self.id
    }

    pub fn optimise2<const N: usize>(&self) -> usize {
        let state: State<N> = State {
            ore_robots: Ore(1),
            ..Default::default()
        };
        let result = state.optimise(&(self.into()));
        result
    }
}

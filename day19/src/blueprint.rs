#![allow(clippy::bool_assert_comparison)]
use crate::{Clay, Obsidian, Ore, State, state::dfs};

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
impl BluePrintOpt {}

impl BluePrint {
    pub fn optimise<const N: usize>(&self) -> usize {
        let state: State<N> = State {
            ore_robots: Ore(1),
            ..Default::default()
        };
        let result = dfs(state, &(self.into()));
        result * self.id
    }

    pub fn optimise2<const N: usize>(&self) -> usize {
        let state: State<N> = State {
            ore_robots: Ore(1),
            ..Default::default()
        };
        dfs(state, &(self.into()))
    }
}

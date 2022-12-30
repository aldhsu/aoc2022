use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::combinator::complete;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

use crate::{BluePrint, Ore, Clay, Obsidian};

pub fn parse_blueprints(s: &str) -> IResult<&str, Vec<BluePrint>> {
    let (s, blueprints) = complete(separated_list0(newline, parse_blueprint))(s)?;
    Ok((s, blueprints))
}

#[test]
fn parsing_blueprint_works() {
    let input = "Blueprint 1: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 17 obsidian.";
    let (_, bp) = parse_blueprint(input).unwrap();
    assert_eq!(
        bp,
        BluePrint {
            id: 1,
            ore: Ore(2),
            clay: Ore(4),
            obsidian: (Clay(20), Ore(3)),
            geode: (Ore(2), Obsidian(17))
        }
    )
}
fn parse_blueprint(s: &str) -> IResult<&str, BluePrint> {
    let (
        s,
        (_, id, _, ore_ore, _, clay_ore, _, obs_ore, _, obs_clay, _, geode_ore, _, geode_obs, _),
    ) = tuple((
        tag("Blueprint "),
        digit1,
        tag(": Each ore robot costs "),
        digit1,
        tag(" ore. Each clay robot costs "),
        digit1,
        tag(" ore. Each obsidian robot costs "),
        digit1,
        tag(" ore and "),
        digit1,
        tag(" clay. Each geode robot costs "),
        digit1,
        tag(" ore and "),
        digit1,
        tag(" obsidian."),
    ))(s)?;

    let bp = BluePrint {
        id: id.parse().expect("can't parse ore ore"),
        ore: Ore(ore_ore.parse().expect("can't parse ore ore")),
        clay: Ore(clay_ore.parse().expect("can't parse clay ore")),
        obsidian: (
            Clay(obs_clay.parse().expect("can't parse obsidian clay")),
            Ore(obs_ore.parse().expect("can't parse obsidian ore")),
        ),
        geode: (
            Ore(geode_ore.parse().expect("can't parse geode ore")),
            Obsidian(geode_obs.parse().expect("can't parse obsidian ore")),
        ),
    };

    Ok((s, bp))
}

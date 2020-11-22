
use std::str::FromStr;
use std::collections::HashMap;

use defaultmap::DefaultHashMap;

mod formula;
use formula::ReactionFormula;
use formula::ReactionParseError;

mod order;

type Element = String;

pub struct ReactionSet {
    reactions: HashMap<Element, ReactionFormula>,
}

impl FromStr for ReactionSet {
    type Err = ReactionParseError;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let mut result = ReactionSet{reactions: HashMap::new() };
        for reaction in input.lines().map(|l| ReactionFormula::from_str(l)) {
            match reaction {
                Ok(reaction) => { result.reactions.insert(reaction.output.clone(), reaction); }
                Err(e) => return Err(e),
            }
        }
        Ok(result)
    }
}

pub fn day14_test_main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
    let ore_costs = compute_ore_costs_str(input)?;
    println!("The ore cost is {}", ore_costs);
    Ok(())
}

pub fn day14_part1_main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input");
    let ore_costs = compute_ore_costs_str(input)?;
    println!("The ore cost is {}", ore_costs);
    Ok(())
}

fn compute_ore_costs_str(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let reaction_set = ReactionSet::from_str(input)?;
    let order = order::sort_elements(&reaction_set);
    println!("The order is {:?}", order);
    let ore_costs = compute_ore_costs(&reaction_set, &order);
    Ok(ore_costs)
}
fn compute_ore_costs(reactions: &ReactionSet, order: &Vec<Element>) -> i32 {
    let mut costs: DefaultHashMap<Element, i32> = DefaultHashMap::new(0);
    costs["FUEL".to_string()] = 1;

    for element in order.iter().rev() {
        if element == "ORE" {
            continue;
        }
        let element_required = costs[element];
        if element_required == 0 {
            println!("Element {} seems unnecessary", element);
            continue;
        }

        println!("Creating {} of {}", element_required, element);
        let reaction = &reactions.reactions[element];
        let reactions_necessary = (element_required + reaction.amount - 1) / reaction.amount; // round up 
        println!("That needs {} reactions", reactions_necessary);
        for (input, input_amount) in reaction.inputs.iter() {
            let existing_costs = costs[input];
            let additional_costs = input_amount * reactions_necessary;
            println!("Registering input: {} of {}", input, existing_costs + additional_costs);
            costs.insert(input.to_string(), existing_costs + additional_costs);
        }
    }
    return costs["ORE".to_string()];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL";
        let ore_cost = compute_ore_costs_str(input).unwrap();
        assert_eq!(ore_cost, 165);
    }

    #[test]
    fn test_example_2() {
        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let ore_cost = compute_ore_costs_str(input).unwrap();
        assert_eq!(ore_cost, 13312);
    }

    #[test]
    fn test_example_3() {
        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        let ore_cost = compute_ore_costs_str(input).unwrap();
        assert_eq!(ore_cost, 180697);
    }

    #[test]
    fn test_example_4() {
        let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        let ore_cost = compute_ore_costs_str(input).unwrap();
        assert_eq!(ore_cost, 2210736);
    }
}
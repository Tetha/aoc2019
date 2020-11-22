
use defaultmap::DefaultHashMap;

use super::Element;
use super::ReactionSet;

#[derive(Debug,Clone)]
enum Mark {
    None,
    Temporary,
    Permanent
}

pub fn sort_elements(reactions: &ReactionSet) -> Vec<Element> {
    let mut marks = DefaultHashMap::new(Mark::None);
    let mut output = Vec::new();
    visit(&"FUEL".to_string(), reactions, &mut marks, &mut output);
    return output;
}

fn visit(node: &Element, reactions: &ReactionSet, marks: &mut DefaultHashMap<Element, Mark>, output: &mut Vec<Element>) {
    match marks[node] {
        Mark::Permanent => return,
        Mark::Temporary => panic!("cycle detected"),
        Mark::None => {
            marks.insert(node.to_string(), Mark::Temporary);
            if node != "ORE" {
                let reaction = &reactions.reactions[node];
                for input in reaction.inputs.keys() {
                    visit(input, reactions, marks, output);
                }
            }
            marks.insert(node.to_string(), Mark::Permanent);
            output.push(node.to_string());
        }
    }
}
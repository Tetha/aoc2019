
use std::collections::HashMap;
#[derive(Debug)]
struct OrbitDefinition {
    orbits: HashMap<String, String>,
}

impl From<&str> for OrbitDefinition {
    fn from(source: &str) -> OrbitDefinition {
        let mut result = OrbitDefinition{
            orbits: HashMap::new(),
        };

        for orbit in source.lines() {
            if !orbit.contains(')') {
                continue;
            }
            let mut parts = orbit.split(')');
            let center = parts.next().unwrap();
            let orbiter = parts.next().unwrap();

            result.orbits.insert(orbiter.to_string(), center.to_string());
        }
        
        return result;
    }
}

struct OrbitIterator<'a> {
    orbits: &'a OrbitDefinition,
    object: Option<String>
}

impl<'a> Iterator for OrbitIterator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.object {
            None => None,
            Some(object) => {
                let next = self.orbits.orbits.get(object);
                match next {
                    None => {
                        self.object = None;
                        return None;
                    }
                    Some(next_center) => {
                        self.object = Some(next_center.to_string());
                        return Some(next_center.to_string());
                    }
                }
            }
        }
    }
}
impl OrbitDefinition {
    fn depth(&self, node: &str) -> Option<i32> {
        if !self.orbits.contains_key(node) {
            return None;
        }

        let mut depth = 0;
        let mut current_node = node;
        loop {
            match self.orbits.get(current_node) {
                None => {
                    return Some(depth);
                }
                Some(center) => {
                    depth = depth + 1;
                    current_node = center;
                }
            }
        }
    }

    fn count_orbits(&self) -> i32 {
        let mut orbits = 0;

        for object in self.orbits.keys() {
            orbits = orbits + self.depth(object).unwrap();
        }

        return orbits;
    }

    fn iter_centers(&self, object: &str) -> OrbitIterator {
        return OrbitIterator{orbits: &self, object: Some(String::from(object))}
    }

    fn iter_common_centers(&self, first: &str, second: &str) -> Vec<String> {
        let mut first_centers = self.iter_centers(first).collect::<Vec<String>>();
        first_centers.reverse();
        let mut second_centers = self.iter_centers(second).collect::<Vec<String>>();
        second_centers.reverse();
        first_centers.iter().zip(second_centers.iter()).filter(|(a,b)| a == b).map(|(a,_)| a.to_string()).collect::<Vec<String>>()
    }

    fn compute_orbit_changes(&self, first: &str, second: &str) -> i32 {
        let common_centers = self.iter_common_centers(first, second);
        let first_depth = self.depth(first).unwrap();
        let second_depth = self.depth(second).unwrap();

        println!("{:?}", common_centers);
        println!("{}, {:?}", first, self.iter_centers(first).collect::<String>());
        println!("{}, {:?}", second, self.iter_centers(second).collect::<String>());
        return (first_depth - common_centers.len() as i32) + (second_depth - common_centers.len() as i32);
    }
}

#[cfg(test)]
mod test {
    use super::OrbitDefinition;
    #[test]
    fn test_orbit_from_string() {
        let input = "
A)B
B)C
C)D
A)F
        ";

        let subject = OrbitDefinition::from(input);
        assert_eq!(subject.orbits["B"], "A");
        assert_eq!(subject.orbits["C"], "B");
        assert_eq!(subject.orbits["D"], "C");
        assert_eq!(subject.orbits["F"], "A");
    }

    #[test]
    fn test_depth() {
        let input = "
A)B
B)C
C)D
A)F
        ";

        let subject = OrbitDefinition::from(input);
        assert_eq!(subject.depth("D"), Some(3));
        assert_eq!(subject.depth("F"), Some(1));
        assert_eq!(subject.depth("Z"), None);
    }

    #[test]
    fn test_common_centers() {
        let input = "
A)B
B)C
B)D
        ";

        let subject = OrbitDefinition::from(input);
        let common_centers: Vec<String> = subject.iter_common_centers("D", "C");
        assert_eq!(common_centers, vec!["A", "B"]);
    }
    #[test]
    fn test_center_iter() {
        let input = "
A)B
B)C
C)D
A)F
        ";

        let subject = OrbitDefinition::from(input);
        let centers: Vec<String> = subject.iter_centers("D").collect();
        assert_eq!(centers, vec!["C", "B", "A"]);
    }
    #[test]
    fn test_given_example_1() {
        let input = "
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L     
        ";

        let subject = OrbitDefinition::from(input);
        assert_eq!(subject.count_orbits(), 42);
    }
}

pub fn day6_main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input");
    let orbits = OrbitDefinition::from(input);
    println!("There are {} orbits", orbits.count_orbits());
    Ok(())
}

pub fn day6_part2_main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input");
    let orbits = OrbitDefinition::from(input);
    println!("There are {} orbit changes", orbits.compute_orbit_changes("YOU", "SAN"));
    Ok(())
}
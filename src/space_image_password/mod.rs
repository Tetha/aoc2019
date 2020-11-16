
pub fn day8_part1_main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input");
    let width = 25;
    let height = 6;

    let layers = split_image_to_layers(input, width, height);
    let most_zero_layers = find_layer_with_least_zeros(&layers).unwrap();
    let checksum = count_digits('1', &most_zero_layers) * count_digits('2', &most_zero_layers);
    println!("The checksum is {}", checksum);
    Ok(())
}

pub fn day8_part2_main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input");
    let width = 25;
    let height = 6;

    let layers = split_image_to_layers(input, width, height);
    let final_layer = stack_layers(&layers);
    print_layer(&final_layer, width);
    Ok(())
}

fn print_layer(layer: &Vec<char>, width: i32) {
    for (index, value) in layer.iter().enumerate() {
        if index % (width as usize) == 0 {
            println!("");
        }
        if *value == '1' {
            print!("*");
        } else {
            print!(" ");
        }
    }
    println!("");
}

fn stack_layers(layers: &Vec<Vec<char>>) -> Vec<char> {
    let mut result = Vec::new();
    for layer in layers.iter().rev() {
        for (index, value) in layer.iter().enumerate() {
            if result.len() <= index {
                result.push(*value);
            } else {
                if *value != '2' {
                    result[index] = *value;
                }
            }
        }
    }
    return result;
}

fn split_image_to_layers(data: &str, width: i32, height: i32) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for layer in data.chars().collect::<Vec<char>>().chunks((width * height) as usize) {
        if layer.len() == 1 && layer[0] == '\n' {
            continue
        }
        result.push(Vec::from(layer));
    }
    return result;
}

fn find_layer_with_least_zeros(layers: &Vec<Vec<char>>) -> Option<&Vec<char>> {
    layers.iter().min_by_key(|l| count_digits('0', l))
}

fn count_digits(digit: char, layer: &Vec<char>) -> usize {
    layer.iter().filter(|c| **c == digit).count()
}
fn main() {
    let input = 277678;
    println!("Next Square: {}", get_next_square(input));
    println!("Side Length: {}", get_side_length(input));
    println!("Layer Number: {}", get_layer_number(input));
    println!("Side: {}", get_side(input));
    println!("Distance to center of side: {}", get_distance_to_center_of_side(input));
    println!("Manhattan Distance: {}", get_layer_number(input) + get_distance_to_center_of_side(input))
}

fn get_distance_to_center_of_side(num: u32) -> u32 {
    //centers are side_length apart
    //calculate from next_square - (side_length * side)
    let next_square = get_next_square(num);
    let side_length = get_side_length(num);
    let side = get_side(num);
    let layer_number = get_layer_number(num);

    let bottom_center = next_square - layer_number;
    let side_center = bottom_center - (side_length * side);
    if num > side_center {
        return num - side_center
    } else {
        return side_center - num
    }
}

fn get_side(num: u32) -> u32 {
    // returns side number
    // bottom: 0, left: 1, top: 2, right: 3
    let next_square = get_next_square(num);
    let mut difference = next_square - num;
    let side_length = get_side_length(num);

    let mut side = 0;
    while difference > side_length {
        difference = difference - side_length;
        side = side + 1
    }

    return side
}


fn get_layer_number(num: u32) -> u32 {
    let mut square = get_side_length(num);
    let mut layer = 1;
    loop {
        if square > 1 {
            layer = layer + 1;
            square = square - 2
        } else {
            break
        }
    }

    return layer - 1
}

fn get_side_length(num: u32) -> u32 {
    let square = get_next_square(num);
    return (square as f64).sqrt() as u32 - 1
}

fn get_next_square(num: u32) -> u32 {
    let mut i: u32 = 1;
    loop {
        let square = i.pow(2);
        if square >= num {
            return square
        } else {
            i = i + 2; //only need odd squares
        }
    }
}

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("{:?}", slice);

    let mut colors = ["red", "green", "blue", "pink"];
    println!("{:?}", colors);
    update_colors(&mut colors[2..4]);
    println!("{:?}", colors);
}

fn update_colors(colors_slice: &mut [&str]) {
    colors_slice[0] = "yellow";
    colors_slice[1] = "orange";
    // colors_slice[2] = "brown";
}
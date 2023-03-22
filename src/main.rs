use std::io;
mod llamas;
fn main() {
    println!("Quick llama population calculator");

    let mut current_pop = String::new();
    let mut goal_pop = String::new();

    println!("How many llamas you currently have?");
    io::stdin()
        .read_line(&mut current_pop)
        .expect("Failed to read the line");

    let current_pop: usize = current_pop.trim().parse().expect("Please type a number!");

    println!("... And how many you want to have?");
    io::stdin()
        .read_line(&mut goal_pop)
        .expect("Failed to read the line");
    let goal_pop: usize = goal_pop.trim().parse().expect("Please type a number!");

    println!("Just to make sure, {goal_pop} is the goal, {current_pop} is current population");
    let how_long = llamas::years_in_llamas(current_pop, goal_pop);

    println!("\nBada Bing Bada Boom.");
    println!("{how_long} Llamayrs. That's how long it will take.");
}

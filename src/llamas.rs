pub fn years_in_llamas(input_pop: usize, goal_pop: usize) -> usize {
    let mut years: usize = 0;
    let mut current_pop = input_pop;

    loop {
        if current_pop < goal_pop {
            current_pop = current_pop + current_pop / 3 - current_pop / 4;
            years += 1;
        }
        if current_pop >= goal_pop {
            return years;
        }
    }
}

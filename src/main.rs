fn main() {
    println!("Hello, world!");

    let hey_there_string = String::from("hey there");

    println!("\nExercise 1:");
    println!("\"{}\" reversed is \"{}\"", &hey_there_string, reverse_string(&hey_there_string));

    let number_list= vec![0, 1, 3, 0, -2, 1, 5, 0];
    let number_to_find = 1;
    let number_of_occurrences = count_occurrences(&number_list, &number_to_find);

    println!("\nExercise 2:");
    println!("The number of occurrences of {} in the vector is {}",
             number_to_find,
             number_of_occurrences);

    let random_numbers = vec![-250, -4, -2000, -375];
    let max_value = get_maximum_value(&random_numbers);

    println!("\nExercise 3:");
    match max_value {
        Some(value) => println!("The max value in the vector is {}", value),
        None => println!("The vector was empty"),
    }

    // todo practice testing borrow violations
}

fn reverse_string(s: &String) -> String {
    s.chars()
        .rev()
        .collect::<String>()
}

fn count_occurrences(vector: &[i32], number: &i32) -> usize {
    vector.iter().filter(|&x| x == number).count()
    /*
    let mut count = 0;

    for item in vector {
        if number == item {
            count += 1;
        }
    }

    count*/
}

fn get_maximum_value(vector: &[i32]) -> Option<i32> {
    vector.iter().copied().max()
    /*
    if vector.is_empty() {
        return None
    }

    let mut maximum_value = &vector[0];

    for item in vector {
        if maximum_value < item {
            maximum_value = item;
        }
    }

    Some(maximum_value)*/
}
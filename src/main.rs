/*
By: <Asen Doiron>
Date: 2026-11-02
Program Details: <The purpose of this program is to allow a teacher to see the marks of their students on multiple assignments and be able to see each students average mark and the class average.>
*/
use std::io;
fn main() {
    let grid: [[f64; 4]; 5] = [
        [80.0, 75.0, 90.0, 85.0],
        [65.0, 70.0, 85.0, 75.0],
        [90.0, 95.0, 88.0, 80.0],
        [70.0, 80.0, 75.0, 94.0],
        [78.0, 82.0, 80.0, 84.0],
    ];

    let student_names = ["Joe", "Bob", "Bill", "Steve", "Kachann"];
    print!("Student:  Task 1:  Task 2:  Task 3:  Task 4:\n");
    for (student_index, row) in grid.iter().enumerate() {
        print!("{}: ", student_names[student_index]);
        for value in row {
            print!("{} ", value);
        }
        println!();
    }

    print!("\nJoe Average: ");
    let mut avg1 = 0.0;
    for student1mean in 0..4 {
        avg1 += grid[0][student1mean];
    }
    println!("{:.2}", avg1 / 4.0);

    print!("\nBob Average: ");
    let mut avg2 = 0.0;
    for student2mean in 0..4 {
        avg2 += grid[1][student2mean];
    }
    println!("{:.2}", avg2 / 4.0);

    print!("\nBill Average: ");
    let mut avg3 = 0.0;
    for student3mean in 0..4 {
        avg3 += grid[2][student3mean];
    }
    println!("{:.2}", avg3 / 4.0);

    print!("\nSteve Average: ");
    let mut avg4 = 0.0;
    for student4mean in 0..4 {
        avg4 += grid[3][student4mean];
    }
    println!("{:.2}", avg4 / 4.0);

    print!("\nKachann Average: ");
    let mut avg5 = 0.0;
    for student5mean in 0..4 {
        avg5 += grid[4][student5mean];
    }
    println!("{:.2}", avg5 / 4.0);

    print!("\nClass Average: ");
    let mut class_avg = 0.0;
    for row in &grid {
        for value in row {
            class_avg += value;
        }
    }
    println!("{:.2}", class_avg / 20.0);

    print!("\nHighest Mark: ");
    let mut highest = 0.0;
    for row in &grid {
        for value in row {
            if *value > highest {
                highest = *value;
            }
        }
    }
    println!("{:.2}", highest);

    print!("\nLowest Mark: ");
    let mut lowest = f64::MAX;
    for row in &grid {
        for value in row {
            if *value < lowest {
                lowest = *value;
            }
        }
    }
    println!("{:.2}", lowest);

    loop {
    let mut input1 = String::new();
    println!("\nWould you like to search for a student's marks? (yes/no)");
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    if input1.trim().to_lowercase() == "yes" {
        let mut input2 = String::new();
    println!("\nSearch for a student's marks by typing their name:");
    io::stdin().read_line(&mut input2).expect("Failed to read line");


    if input2.trim().to_lowercase() == student_names[0].trim().to_lowercase() {
        println!("\n{}'s marks: {:?}", student_names[0], grid[0]);
    } else if input2.trim().to_lowercase() == student_names[1].trim().to_lowercase() {
        println!("\n{}'s marks: {:?}", student_names[1], grid[1]);
    } else if input2.trim().to_lowercase() == student_names[2].trim().to_lowercase() {
        println!("\n{}'s marks: {:?}", student_names[2], grid[2]);
    } else if input2.trim().to_lowercase() == student_names[3].trim().to_lowercase() {
        println!("\n{}'s marks: {:?}", student_names[3], grid[3]);
    } else if input2.trim().to_lowercase() == student_names[4].trim().to_lowercase() {
        println!("\n{}'s marks: {:?}", student_names[4], grid[4]);
    } else {
        println!("\nStudent not found.");
    }
    }
    else if input1.trim().to_lowercase() == "no" {
        println!("\nExiting the program.");
        break;
    } else {
        println!("\nInvalid input. Please enter 'yes' or 'no'.");
}
}
}

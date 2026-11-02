/*
By: <Asen Doiron>
Date: 2026-11-02
Program Details: <The purpose of this program is to allow a teacher to see the marks of their students on multiple assignments and be able to see each students average mark and the class average.>
*/

fn main() {
    let grid: [[f64; 3]; 3] = [
        [80.0, 75.0, 90.0],
        [65.0, 70.0, 85.0],
        [70.0, 80.0, 75.0],
    ];

    let nums: [f64; 3] = [1.0, 2.0, 3.0];
    print!("Student:  Task 1:  Task 2:  Task 3:\n");
    for (students, row) in grid.iter().enumerate() {
        print!("Student {}: ", nums[students]);
        for value in row {
            print!("{} ", value);
        }
        println!();
    }
    print!("\nStudent 1 Average: ");
    let mut sum1 = 0.0;
    for student1mean in 0..3 {
            sum1 += grid[0][student1mean] / 3.0;
        }
        println!("{:.2}", sum1);

    print!("\nStudent 2 Average: ");
    let mut sum2 = 0.0;
    for student2mean in 0..3 {
            sum2 += grid[1][student2mean] / 3.0;
        }
        println!("{:.2}", sum2);

        print!("\nStudent 3 Average: ");
    let mut sum3 = 0.0;
    for student3mean in 0..3 {
            sum3 += grid[2][student3mean] / 3.0;
        }
        println!("{:.2}", sum3);
    }

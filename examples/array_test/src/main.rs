extern crate array2d as Array2d;

fn main() {

    let mut array: [&str; 4] = ["0"; 4];

    for (i, ele) in array.iter().enumerate() {
        print!("{ele}");
    }
    println!();

    let mut array_2d: [[&str; 4]; 4] = [["0"; 4]; 4];
    
    for (i, row) in array_2d.iter().enumerate(){
        for (j, col) in row.iter().enumerate() {
            print!("{col}");
        }
        println!();
    }
    println!();


    /*
    let array = Array2d::filled_with(4, 4, 4);    // create prefilled 2d array

    for element in array.column_iter(0) {
        println!("{}", element)
    }
    println!();
    */
}

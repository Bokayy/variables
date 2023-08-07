fn main() {
    /* let mut x = 5;
    println!("The value of x is: {x}");
    x=6;
    println!("The value of x is: {x}"); */

//shadowing, and instanced variables in loops
    /* let x = 5;
    let x = x+1;

    {
        let x= x+2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}"); */

    /* let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}"); */

    /* let number_of_letters = "aSgfgAStgerzjh<gfDFGHDFVY";
    let number_of_letters = number_of_letters.len();
    println!("{number_of_letters}"); */

//codewars fake binary
   /*  fn fake_bin(s:&str) -> String{
        let mut final_string = String::new();
        for ch in s.chars()
        {
            let numeric_value = ch.to_digit(10).unwrap();
            if numeric_value < 5 {
                final_string.push('0');
            }
            else
            {
                final_string.push('1');
            }

        }
        print!("{final_string}");
        return final_string;
    } 

    fake_bin("45385593107843568"); */
//codewars fake binary

//tuples and destructuring
/* let tup: (i32, char, f32) = (8,'i',3.2);
let eight = tup.0;
let letter_i = tup.1;
let three_point_two = tup.2;
println!("{},{},{}", eight, letter_i, three_point_two); */

//arrays
/* let a = [1,2,3,4,5];
let months = ["January", "February","March",
"April","May","June","July","August","September","October",
"November","December"];
for month in months{
    println!("{month}");
}
//initialize an array of 5 i32s
let a: [i32;5] = [1,2,3,4,5];
//initialize an array of 5 equal values
let c= ['a';5];

let stubbed_toe = || for character in c {println!("{character}")};

stubbed_toe();
 */
//arrays

//mixed tuple array

/*let t = ([1;2],[3;4]);
let (a,_) = t;
println!("{}", a[0] + t.1[0]);*/
}

// codewars

fn main() {
    // Commenting like that is possible in Rust lang
    /* 
        Also I have this way to comment something
    */

    println!("{:?}", is_square(18));
    println!("{:?}", is_square(16));
    println!("{:?}", is_square(11));
    println!("{:?}", is_square(9));

}

fn is_square(n: i64) -> bool {
    // returns true if integer is square
    f64::sqrt(n as f64) % 1_f64 == 0.0
}

fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    // returns vector of divisors
    let mut v = Vec::new();

    for i in 2..integer-1 {
        if integer % i == 0 {
            v.push(i);
        }
    }

    let length = v.len();

    let mut result = Ok(v);

    if length == 0 {
        result = Err(format!("{} is prime", integer));
    }

    result
}

#[test]
fn divisors_tests() {
  assert_eq!(divisors(15), Ok(vec![3,5]));
  assert_eq!(divisors(12), Ok(vec![2,3,4,6]));
  assert_eq!(divisors(13), Err("13 is prime".to_string()));
}

#[test]
fn fixed_tests() {
    assert_eq!(is_square(-1), false, "\nYour answer (left) is not the expected answer (right).");
    assert_eq!(is_square(0), true, "\nYour answer (left) is not the expected answer (right).");
    assert_eq!(is_square(3), false, "\nYour answer (left) is not the expected answer (right).");
    assert_eq!(is_square(4), true, "\nYour answer (left) is not the expected answer (right).");
    assert_eq!(is_square(25), true, "\nYour answer (left) is not the expected answer (right).");
    assert_eq!(is_square(26), false, "\nYour answer (left) is not the expected answer (right).");
}
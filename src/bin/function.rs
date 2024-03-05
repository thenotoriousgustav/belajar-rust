fn main(){
    my_function("gustam", 20);

    let sum = sum_function(50, 50);
    println!("{sum}");
}

fn my_function(name: &str, age: i8){
    println!("my name is {name} and im {age} years old!");
}

fn sum_function(a: i32, b:i32) -> i32 {
    return a + b;
    // a, b; //! we can do like this instead
}
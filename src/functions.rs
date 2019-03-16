pub fn main(){
    println!("Hello World");
    another_function();
}

fn another_function(x: i32) {
    let x = 10;
    let y = {
        let x = 3;
        x + 12
    };
    println!("The value of x is {}", x);
}
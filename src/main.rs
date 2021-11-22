

fn main() {
    tuples()
}


fn tuples() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true  ) ;

    // get value of a tuple's index
    println!("long tuple 8's value: {}", long_tuple.8);
    //get value of the tuple
    println!(" tuples: {:?}", long_tuple);


    // destructuring tuple
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("get value of a {:?}", a);

}


fn arrays() {
}






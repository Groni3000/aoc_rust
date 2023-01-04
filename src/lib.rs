#[allow(non_snake_case)]
#[allow(unused_variables)]
pub mod days;

#[allow(dead_code)]
fn print_objects<T>(x: &Vec<T>)
where T:std::fmt::Debug{
    for el in x{
    println!("{:?}", el);
    }
}
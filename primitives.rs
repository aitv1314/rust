fn main(){
    let logical: bool = true;

    let a_float: f64 = 1.2;
    let an_integer = 5i32;

    let default_float = 3.0; // f64
    let default_integer = 5; // i32

    let mut inferred_type = 12;
    inferred_type = 15454545555i64;

    let mut mutable = 12; // i32
    mutable = 66;

    // mutable = true;

    let mutable = true; // overwritten with shadowing
}
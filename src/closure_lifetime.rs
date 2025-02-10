fn fn_lifetime(x: &i32) -> &i32 { 
    x
}

fn closure_lifetime(x: &i32) -> &i32 {
    let closure = |x: &i32| -> &i32 { x };
    closure(x)
}
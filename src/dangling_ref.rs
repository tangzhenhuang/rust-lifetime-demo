fn dangling_ref() -> &i32 {
    let local_var = 888_i32;
    &local_var
}
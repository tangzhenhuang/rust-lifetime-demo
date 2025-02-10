#[derive(Debug)]
struct SelfRef<'a> {
    value: String,
    pointer_to_value: &'a str,
}

pub fn new_self_ref() {
    let s = "aaa".to_string();
    let v = SelfRef {
        value: s,
        pointer_to_value: &s
    };
}
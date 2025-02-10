#[derive(Debug)]
struct SelfRef<'a> {
    data: RefData,
    pointer_to_data: &'a RefData,
}
#[derive(Debug)]
struct RefData {
    data: String
}

pub fn new_self_ref() {
    let data = RefData{
        data: "Rust".to_string()
    };
    let v = SelfRef {
        data,
        pointer_to_data: &data
    };
}
struct StaticLifeTime {
    static_ref: &'static str,
}

fn static_lifetime() {
    let static_ref = "static_ref".to_string();
    let static_lifetime = StaticLifeTime {
        static_ref: &static_ref,
    };
}
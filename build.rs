

fn main() {
    let profile = std::env::var("PROFILE").unwrap();

    match profile.as_str() {
        "debug" => return,
        _ => {
            static_vcruntime::metabuild();
        }
    }
}
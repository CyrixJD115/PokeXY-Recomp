fn main() {
    for p in std::env::args().skip(1) {
        let data = std::fs::read(&p).unwrap();
        match xy_convert::glb_check::validate(&data) {
            Ok(r) => {
                println!("== {p}");
                println!("   {}", r.summary());
                for e in &r.errors {
                    println!("   ERROR: {e}");
                }
                for w in &r.warnings {
                    println!("   warn : {w}");
                }
            }
            Err(e) => println!("== {p}\n   PARSE FAILED: {e}"),
        }
    }
}


fn usage() {
    println!("tindymd, a markdown compiler written by Brian.");
    println!("Version {}", get_version());
}

fn get_version() -> u16 {
    1000
}

fn main() {
    usage();
}

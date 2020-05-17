
fn usage() {
    let mut the_version = env!("CARGO_PKG_VERSION");
    the_version = "2.0";
    println!("tindymd, a markdown compiler written by Brian.");
    println!("Version {}", the_version);
}

fn main() {
    usage();
}

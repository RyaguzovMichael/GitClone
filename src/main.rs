use gclone::clone;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(git_url) = args.get(1) {
        clone(git_url).expect("Internal error");
        return;
    };
    println!("Do not enter git_url as argument!");
}

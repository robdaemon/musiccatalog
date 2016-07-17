use std::fs;
use std::path;

pub fn walkdir(dir: &path::Path) {
    trace!("Commencing yak shaving");
    match fs::read_dir(dir) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }
}

#[test]
fn test_walkdir() {
    walkdir(&path::Path::new("."));
}

use std::{fs, path::{Path, PathBuf}};
use regex::Regex;

pub fn resolve(path: &Path) -> Option<Vec<PathBuf>> {
    if !path.is_absolute() { panic!("For now path must be absolute"); }

    dbg!(path);

    let mut fin: Vec<PathBuf> = Vec::new();

    match path.iter().nth(0) {
        Some(e) => { let mut t = PathBuf::new(); t.push(e); fin.push(t) },
        None => { return Some(fin); }
    }

    for pe in path.iter().skip(1) {
        if fin.is_empty() { return Some(fin); }

        if pe.to_str().unwrap().contains("*") {
            let re = Regex::new(&pe.to_str().unwrap().replace("*", ".+")).unwrap();
            let mut new_paths: Vec<PathBuf> = Vec::new();

            for p in &fin {
                let regex_filter = |x: PathBuf| -> Option<PathBuf> {
                    if re.is_match(x.to_str().unwrap()) {
                        dbg!(&x);
                        return Some(p.join(x))
                    } else {
                        return None
                    };
                };
                let u: Vec<PathBuf> = fs::read_dir(p).unwrap().map(|x| x.unwrap().path()).collect();
                dbg!(u);

                let mut items: Vec<PathBuf> = fs::read_dir(p).unwrap().map(|x| x.unwrap().path()).filter_map(regex_filter).collect();
                dbg!(&items);
                new_paths.append(&mut items);
                
            }
            fin = new_paths;

        } else {
            fin = fin.into_iter().filter_map(
                |x| if x.join(pe).try_exists().unwrap() {
                    Some(x.join(pe))
                } else {
                    None
                }
            ).collect();
        }
    }

    Some(fin)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::{Alphanumeric, DistString};

    fn test_setup() -> PathBuf {
        let test_dir = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        let mut tdr = std::env::temp_dir();
        tdr.push(test_dir);
        _ = std::fs::create_dir(&tdr);

        tdr
    }

    #[test]
    fn ending_asterisk() {
        let tdr = test_setup();

        let test_paths: Vec<PathBuf> = vec![
            tdr.join("A"),
            tdr.join("B"),
        ];
        // _ = test_paths.iter().map(|x| std::fs::create_dir(x));
        for p in &test_paths {
            _ = std::fs::create_dir(p);
        }
        
        let test_path = tdr.clone().join("*");

        assert_eq!(
            resolve(&test_path).unwrap(),
            test_paths
        );
    }
}

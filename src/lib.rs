use std::{fs, path::{Path, PathBuf}};
use regex::Regex;

pub fn resolve(path: &Path) -> Option<Vec<PathBuf>> {
    if !path.is_absolute() { panic!("For now path must be absolute"); }
    // VALIDATE CHARACTERS IN PATH. INVALID PATH CHARACTERS CAN MESS UP THE REGEX
    // Specifically, replace the . in paths with a regex literal .
    // For symlinks, try to canonicalize each component?

    dbg!(path);

    let mut fin: Vec<PathBuf> = Vec::new();

    match path.iter().nth(0) {
        Some(e) => { let mut t = PathBuf::new(); t.push(e); fin.push(t) },
        None => { return Some(fin); }
    }

    for pe in path.iter().skip(1) {
        if fin.is_empty() { return Some(fin); }

        if pe.to_str().unwrap().contains("*") {
            let re_string = format!("^{}$", &pe.to_str().unwrap().replace("*", ".*"));
            dbg!(&re_string);
            let re = Regex::new(&re_string).unwrap();
            let mut new_paths: Vec<PathBuf> = Vec::new();

            dbg!(pe);
            dbg!(&fin);
            // let u: Vec<PathBuf> = read_dir(&fin[0]).unwrap().map(|x| x.unwrap().path()).collect();
            // dbg!(u);


            // could collapse the * and non-* into the same code
            // if statement creates a list of all valid new items (for each thing already in fin)
            // in the non-* case, this will just be one entry
            // then shared code does appending for every item in that list
            for p in &fin {
                let regex_filter = |x: PathBuf| -> Option<PathBuf> {
                    if re.is_match(x.iter().last().unwrap().to_str().unwrap()) {
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
                
                // MAYBE REPLACE THIS WITH A SINGLE FIN = FIN.FILTER LIKE IN THE OTHER CASE
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

    fin.sort_by_key(
        |k| k.as_os_str().to_owned()
    );
    
    Some(fin)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::{Alphanumeric, DistString};
    use std::fs;

    fn test_setup() -> PathBuf {
        let test_dir = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        let mut tdr = std::env::temp_dir(); // THIS DOESN'T DELETE AFTER RUNNING????
        tdr.push(test_dir);
        _ = fs::create_dir(&tdr);

        tdr
    }

    #[test]
    fn ending_asterisk() {
        let tdr = test_setup();
        let test_path = tdr.clone().join("*");

        let mut solution: Vec<PathBuf> = vec![
            tdr.join("A"),
            tdr.join("B"),
        ];
        solution.sort_by_key(
            |k| k.as_os_str().to_owned()
        );

        for p in &solution {
            _ = fs::create_dir(p);
        }
        
        assert_eq!(
            resolve(&test_path).unwrap(),
            solution
        );
    }

    #[test]
    fn double_asterisk() {
        let tdr = test_setup();
        let test_input = tdr.clone().join("*").join("*");

        _ = fs::create_dir(tdr.join("A"));
        _ = fs::create_dir(tdr.join("B"));

        let mut solution: Vec<PathBuf> = vec![
            tdr.join("A").join("C"),
            tdr.join("B").join("D"),
        ];
        solution.sort_by_key(
            |k| k.as_os_str().to_owned()
        );

        for p in &solution {
            _ = fs::create_dir(p);
        }
        
        assert_eq!(
            resolve(&test_input).unwrap(),
            solution
        );
    }

    #[test]
    fn not_all_items() {
        let tdr = test_setup();
        let test_input = tdr.clone().join("X*");

        let mut paths: Vec<PathBuf> = vec![
            tdr.join("XA"),
            tdr.join("X")
        ];

        let mut solution = paths.clone();
        solution.sort_by_key(
            |k| k.as_os_str().to_owned()
        );

        paths.push(tdr.join("YB"));
        paths.push(tdr.join("YX"));
        for p in paths {
            _ = fs::create_dir(p);
        }

        dbg!(&solution);

        assert_eq!(
            resolve(&test_input).unwrap(),
            solution
        );
    }

    #[test]
    fn double_compund() {
        let tdr = test_setup();
        let test_input = tdr.clone().join("X*").join("Y").join("*Z");

        let first_layer: Vec<PathBuf> = vec![
            tdr.join("X"),
            tdr.join("XA"),
            tdr.join("YB"),
            tdr.join("YX"),
        ];
        for p in &first_layer { _ = fs::create_dir(p); }

        let second_layer: Vec<PathBuf> = vec![
            tdr.join("X").join("Y"),
            tdr.join("X").join("TY"),
            tdr.join("XA").join("Y"),
            tdr.join("YB").join("Y")
        ];
        for p in &second_layer { _ = fs::create_dir(p); }

        let mut third_layer: Vec<PathBuf> = vec![
            tdr.join("X").join("Y").join("Z"),
            tdr.join("XA").join("Y").join("Z"),
            tdr.join("XA").join("Y").join("TZ"),
            tdr.join("XA").join("Y").join("ZAZ"),
            tdr.join("XA").join("Y").join("ZZZ"),
        ];

        let mut solution = third_layer.clone();
        solution.sort_by_key(
            |k| k.as_os_str().to_owned()
        );

        third_layer.push(tdr.join("X").join("Y").join("z"));
        third_layer.push(tdr.join("X").join("TY").join("Z"));
        third_layer.push(tdr.join("XA").join("Y").join("ZA"));

        for p in &third_layer { _ = fs::create_dir(p); }

        assert_eq!(
            resolve(&test_input).unwrap(),
            solution
        );


        // match OpenOptions::new().create(true).write(true).open(path) {
    }
}

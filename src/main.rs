use std::collections::HashMap;
use std::env::{args, Args};
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};
use std::str::Split;

fn abrev_to_full() -> HashMap<String, String> {
    HashMap::from([
        (String::from("al"), String::from("alabama")),
        (String::from("ak"), String::from("alaska")),
        (String::from("az"), String::from("arizona")),
        (String::from("ar"), String::from("arkansas")),
        (String::from("ca"), String::from("california")),
        (String::from("co"), String::from("colorado")),
        (String::from("ct"), String::from("connecticut")),
        (String::from("de"), String::from("delaware")),
        (String::from("fl"), String::from("florida")),
        (String::from("ga"), String::from("georgia")),
        (String::from("hi"), String::from("hawaii")),
        (String::from("id"), String::from("idaho")),
        (String::from("il"), String::from("illinois")),
        (String::from("in"), String::from("indiana")),
        (String::from("ia"), String::from("iowa")),
        (String::from("ks"), String::from("kansas")),
        (String::from("ky"), String::from("kentucky")),
        (String::from("la"), String::from("louisiana")),
        (String::from("me"), String::from("maine")),
        (String::from("md"), String::from("maryland")),
        (String::from("ma"), String::from("massachusetts")),
        (String::from("mi"), String::from("michigan")),
        (String::from("mn"), String::from("minnesota")),
        (String::from("ms"), String::from("mississippi")),
        (String::from("mo"), String::from("missouri")),
        (String::from("mt"), String::from("montana")),
        (String::from("ne"), String::from("nebraska")),
        (String::from("nv"), String::from("nevada")),
        (String::from("nh"), String::from("new hampshire")),
        (String::from("nj"), String::from("new jersey")),
        (String::from("nm"), String::from("new mexico")),
        (String::from("ny"), String::from("new york")),
        (String::from("nc"), String::from("north carolina")),
        (String::from("nd"), String::from("north dakota")),
        (String::from("oh"), String::from("ohio")),
        (String::from("ok"), String::from("oklahoma")),
        (String::from("or"), String::from("oregon")),
        (String::from("pa"), String::from("pennsylvania")),
        (String::from("ri"), String::from("rhode island")),
        (String::from("sc"), String::from("south carolina")),
        (String::from("sd"), String::from("south dakota")),
        (String::from("tn"), String::from("tennessee")),
        (String::from("tx"), String::from("texas")),
        (String::from("ut"), String::from("utah")),
        (String::from("vt"), String::from("vermont")),
        (String::from("va"), String::from("virginia")),
        (String::from("wa"), String::from("washington")),
        (String::from("wv"), String::from("west virginia")),
        (String::from("wi"), String::from("wisconsin")),
        (String::from("wy"), String::from("wyoming")),
        //////////////////////////////////////////////////
        (String::from("dc"), String::from("washington district of columbia")),
        (String::from("gu"), String::from("guam")),
        (String::from("vg"), String::from("british virgin islands")),
    ])
}

fn main() {
    // removed:
    // new york, new york
    let short_long = abrev_to_full();
    let read_file = "US.txt".to_string();

    let write_file = "locations.txt".to_string();

    let read_file = File::open(read_file).unwrap();
    let reader: BufReader<File> = BufReader::new(read_file);

    let mut locations: HashMap<String, (String, String)> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap().to_lowercase();

        let mut line= line.split("\t");

        let _id_number = line.next().unwrap().trim();
        let city = line.next().unwrap().trim();
        let _city_2 = line.next().unwrap().trim();
        let _city_3 = line.next().unwrap().trim();
        let latitude = line.next().unwrap().trim();
        let longitude = line.next().unwrap().trim();
        let feature_class = line.next().unwrap().trim();
        let _feature_code = line.next().unwrap().trim();
        let _country_code = line.next().unwrap().trim();
        let mut cc2 = line.next().unwrap().trim();
        let cc3 = line.next().unwrap().trim();

        if feature_class == "p" || feature_class == "a" {

            if cc2 == "" {
                cc2 = cc3;
            }
    
            let location = match short_long.get(cc2) {
                Some(state ) => {
                    format!("{}, {}", city, state)
                },
                _ => {
                    println!("couldn't find: {}, {}", city, cc2);
                    format!("{}, {}", city, cc2)
                }
            };
    
            if !locations.contains_key(&location) {
                locations.insert(location.to_string(), (latitude.to_string(), longitude.to_string()));
            }
        }
    }
    let mut writer = File::create(write_file).unwrap();

    let mut keys: Vec<&String> = locations.keys().collect();
    keys.sort();

    for key in keys.iter() {
        let (lat, long) = locations.get(*key).unwrap();
        writeln!(&mut writer, "{}\t{}\t{}", key, lat, long).unwrap();
    }
}
use std::fmt::Display;

pub mod val;

fn main() {
    println!("Hello, world!");

    println!("{} days", 31);

    println!("{0}, this is {1}, {1},this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}", subject = "the lazy dog", object="the quick brown fox", verb = "jump over");


    println!("{} of {:b} people go ", 1, 2);

    let name = "Peter";
    let age = 33;
    let peter = Person{name, age};
    println!("{:?}", peter);
    println!("{:#?}", peter);


    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

}


#[allow(dead_code)]
struct Structure(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age:u8
}

struct City {
    name:&'static str,
    lat:f32,
    lon:f32,
}

impl Display for City {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};
        write!(f, "{}:{:.3} {} {:.3} {}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}
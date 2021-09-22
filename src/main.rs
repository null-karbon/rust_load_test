extern crate reqwest;
extern crate clap;
extern crate rand;

use clap::{Arg, App};
use std::{thread, time};
use std::error::Error;
use rand::{Rng, thread_rng};

fn main() -> Result<(), Box<dyn Error>> {

    let arguments = App::new("Rust load test")
        .version("0.1.0")
        .author("Nick K. <ngkdev@gmail.com>")
        .about("Runs load tests")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .help("Url to test")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("number")
            .short("n")
            .long("number")
            .help("Number of requests to make")
            .takes_value(true))
        .arg(Arg::with_name("delay")
            .short("d")
            .long("delay")
            .help("Delay between requests in millis")
            .takes_value(true))
        .arg(Arg::with_name("verbosity")
            .short("v")
            .long("verbose")
            .help("Run in verbose mode"))
        .get_matches();


    let url = arguments.value_of("url").unwrap();
    let number: u32 = if arguments.is_present("number") {
        match arguments.value_of("number") {
            Some(x) if x.parse::<u32>().is_ok() => {
                if x.parse::<u32>()? > 10000000 {
                    println!("Over 10m requests, that's going to take a while. Get a real load tester.");
                    println!("Defaulting to 1k.");
                    1000
                }else if x.parse::<u32>()? == 0{
                    println!("You can't try 0 times.");
                    println!("Defaulting to 1k.");
                    1000
                }
                else{
                    println!("Number flag present and set.");
                    x.parse::<u32>()?
                }
            },
            _ => {
                println!("Error parsing number of requests value, defaulting to 1k.");
                1000
            }
        }
    } else {
        println!("Number of requests not provided, defaulting to 1k.");
        1000
    };

    let delay_arg: u64 = if arguments.is_present("delay") {
        match arguments.value_of("delay") {
            Some(x) if x.parse::<u64>().is_ok() => {
                println!("Delay flag present and set.");
                x.parse::<u64>()?
            },
            _ => {
                println!("Error parsing delay value, defaulting to 250ms.");
                250
            }
        }
    } else {
        println!("Delay in ms not provided, defaulting to 250ms.");
        250
    };


    let delay_millis = time::Duration::from_millis(delay_arg);

    println!("Waiting random interval to start...");
    let rng: u64 = thread_rng().gen_range(1..11);
    thread::sleep(time::Duration::from_secs(rng));

    if arguments.is_present("verbosity") {
        println!("Running in verbose mode");
        println!("Load testing {} {} times", url, number);
        println!("Waiting {:?} between each call", delay_millis);
    }

    let mut i = 0;

    while i < number {
        i += 1;
        println!("Request #: {}", i);
        let res = reqwest::blocking::get(url)?;
        println!("Status: {}\n", res.status());
        //println!("Headers:\n{:?}", res.headers());
        // copy the response body directly to stdout
        //res.copy_to(&mut std::io::stdout())?;
        thread::sleep(delay_millis);
    }

    println!("\n\nTesting Complete.");
    Ok(())
}
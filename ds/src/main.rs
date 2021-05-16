use std::thread::spawn;
use std::{
    any::Any,
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

use rand::prelude::*;

fn main() -> Result<(), Box<dyn Any + Send + 'static>> {
    //map from anonymous vector of tuples
    let m: HashMap<&str, isize> = [("ceva", 10 as isize)].iter().cloned().collect();

    // set from map
    let n: HashSet<_> = m.keys().into_iter().collect();

    n.iter().for_each(|v| println!("{}", v));

    // Vector from hashset
    let v: Vec<_> = n.into_iter().collect();
    v.iter().for_each(|v| println!("{}", v));

    let vals = Arc::new(RwLock::new(Vec::<u8>::new()));

    let mut rnd = rand::thread_rng();

    (0..100).for_each({
        let v = vals.clone();
        let mut rnd = rnd.clone();
        move |_| {
            v.write()
                .map_or_else(|err| println!("{}", err), |mut smth| smth.push(rnd.gen()));
        }
    });

    let mut threads = Vec::new();

    (1..10).for_each(|_| {
        threads.push(spawn({
            let v = vals.clone();
            move || (100..200).for_each(|i| v.write().map_or((), |mut locked| locked.push(i)))
        }))
    });

    threads.into_iter().for_each(|t| t.join().unwrap_or(()));

    vals.as_ref().read().map_or_else(
        |err| println!("{}", err),
        |locked| locked.iter().for_each(|v| println!("{}", v)),
    );

    let mut matrix: Vec<Vec<isize>> = vec![vec![0; 10]; 10];

    matrix.iter_mut().for_each(|line| {
        line.iter_mut().for_each(|val| *val = rnd.gen::<isize>());
        println!();
    });

    matrix.iter().for_each(|line| {
        line.iter().for_each(|val| print!(" {} ", val));
        println!();
    });

    Ok(())
}

#![feature(test)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

extern crate petgraph;
extern crate rand;
extern crate test;

// pub mod mentu;
pub mod yaku;

pub mod structs;

pub mod agari;
pub mod util;

mod algos;
mod yaku_rules;

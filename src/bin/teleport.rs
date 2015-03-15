#![feature(test)]

extern crate test;

#[macro_use]
extern crate teleport;

use std::fmt::Write;
use std::mem::replace;
use test::Bencher;

use teleport::{push_attribute, AttributeValue, Attribute};

// bench: find the `BENCH_SIZE` first terms of the fibonacci sequence
static BENCH_SIZE: usize = 20;

// recursive fibonacci
fn fibonacci(n: usize) -> u32 {
   if n < 2 {
      1
   } else {
      fibonacci(n - 1) + fibonacci(n - 2)
   }
}

// iterative fibonacci
struct Fibonacci {
   curr: u32,
   next: u32,
}

impl Iterator for Fibonacci {
   type Item = u32;
   fn next(&mut self) -> Option<u32> {
      let new_next = self.curr + self.next;
      let new_curr = replace(&mut self.next, new_next);

      Some(replace(&mut self.curr, new_curr))
   }
}

fn fibonacci_sequence() -> Fibonacci {
   Fibonacci { curr: 1, next: 1 }
}


// function to benchmark must be annotated with `#[bench]`
#[bench]
fn recursive_fibonacci(b: &mut Bencher) {
   // exact code to benchmark must be passed as a closure to the iter
   // method of Bencher
   b.iter(|| {
      (0..BENCH_SIZE).map(fibonacci).collect::<Vec<u32>>()
   })
}


#[bench]
fn iterative_fibonacci(b: &mut Bencher) {
   b.iter(|| {
      fibonacci_sequence().take(BENCH_SIZE).collect::<Vec<u32>>()
   })
}


pub fn main() {
   let class = AttributeValue::List(vec![
      AttributeValue::StaticString("first"),
      AttributeValue::StaticString("second")
   ]);

   let first_attr = Attribute {
      name: "class",
      value: class,
   };

   let mut rendered = String::new();

   push_attribute(&first_attr, &mut rendered);

   println!("{}", rendered);

   let list = attr!["id", "location"; "class", "identical"];

   println!("{:?}", list);
}

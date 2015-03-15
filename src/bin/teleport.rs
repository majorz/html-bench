#![feature(test)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate test;

#[macro_use]
extern crate teleport;

use std::fmt::Write;
use std::mem::replace;
use test::Bencher;
use std::iter::{repeat, FromIterator, IntoIterator};


use teleport::{push_attribute, AttributeValue, Attribute};



#[bench]
fn string_concat(b: &mut Bencher) {

   let mut table_contents: Vec<Vec<u32>> = Vec::new();
   //let mut table_contents: Vec<[u32; 10]> = Vec::new();
   for i in 0..10000 {
      let k = i*10;
      table_contents.push(Vec::from_iter((k..k+10)))
      //table_contents.push([k, k+1, k+2, k+3, k+4, k+5, k+6, k+7, k+8, k+9]);
   }

   b.iter(|| {
      let mut result = String::new();
      result.push_str("<table>\n");

      for index in 0..10000 {//table_contents.iter() {
         result.push_str("  <tr>\n");

         for item in table_contents[index].iter() {
            result.push_str("    <td>");
            result.push_str(&item.to_string());
            result.push_str("<td>\n");
         }

         result.push_str("  </tr>\n");
      }

      result.push_str("</table>");
   });
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

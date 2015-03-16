#![feature(test)]
#![feature(plugin)]

#![plugin(maud_macros)]

#![allow(unused_imports)]
#![allow(dead_code)]

extern crate test;
extern crate maud;

#[macro_use]
extern crate teleport;

use std::fmt::Write;
use std::mem::replace;
use test::Bencher;
use std::iter::{repeat, FromIterator, IntoIterator};


use teleport::{push_attribute, AttributeValue, Attribute};


#[bench]
fn concat_string_source(b: &mut Bencher) {
   let mut table_contents: Vec<[&'static str; 10]> = Vec::new();
   for _ in 0..1000 {
      table_contents.push(["aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj"])
   }

   b.iter(|| {
      let mut result = String::with_capacity(200000);
      result.push_str("<table>\n");

      for row in table_contents.iter() {
         result.push_str("  <tr>\n");

         for item in row {
            result.push_str("    <td>");
            result.push_str(&item.to_string());
            result.push_str("<td>\n");
         }

         result.push_str("  </tr>\n");
      }

      result.push_str("</table>");

      result
   });
}


#[bench]
fn concat_vector_source(b: &mut Bencher) {
   let mut table_contents: Vec<[&'static str; 10]> = Vec::new();
   for _ in 0..1000 {
      table_contents.push(["aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj"])
   }

   b.iter(|| {
      let mut result = Vec::new();
      result.push("<table>\n");

      for row in table_contents.iter() {
         result.push("  <tr>\n");

         for item in row {
            result.push("    <td>");
            result.push(item);
            result.push("<td>\n");
         }

         result.push("  </tr>\n");
      }

      result.push("</table>");
   });
}


#[bench]
fn maud_render(b: &mut Bencher) {
   let mut table_contents: Vec<[&'static str; 10]> = Vec::new();
   for _ in 0..1000 {
      table_contents.push(["aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj"])
   }

   b.iter(|| {
      let markup = html! {
         table {
            $for row in table_contents.iter() {
               tr {
                  $for item in row {
                     td $item
                  }
               }
            }
         }
      };

      markup.to_string()
   });
}


fn render_table() {
   let mut table_contents: Vec<[&'static str; 10]> = Vec::new();
   for _ in 0..1000 {
      table_contents.push(["aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj"])
   }

   let markup = html! {
      table {
         $for row in table_contents.iter() {
            tr {
               $for item in row {
                  td $item
               }
            }
         }
      }
   };

   println!("{}", markup.to_string())
}


pub fn main() {
   render_table();

   // let class = AttributeValue::List(vec![
   //    AttributeValue::StaticString("first"),
   //    AttributeValue::StaticString("second")
   // ]);
   //
   // let first_attr = Attribute {
   //    name: "class",
   //    value: class,
   // };
   //
   // let mut rendered = String::new();
   //
   // push_attribute(&first_attr, &mut rendered);
   //
   // println!("{}", rendered);
   //
   // let list = attr!["id", "location"; "class", "identical"];
   //
   // println!("{:?}", list);
}

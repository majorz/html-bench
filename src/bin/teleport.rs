#[macro_use]
extern crate teleport;

use std::fmt::Write;

use teleport::{push_attribute, AttributeValue, Attribute};


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


   let mut out = String::new();

   let html_title = format!("HTML Title {}", 100).to_string();

   write_html!(&mut out,
      html[
           head[title[html_title]]
           body[h1["Macros are the best!"]]
      ]
   );

   println!("{}", out);
}

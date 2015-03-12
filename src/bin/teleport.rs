extern crate teleport;

use std::collections::BTreeMap;
use teleport::{render_attribute, Attr};

pub fn main() {
   let mut map: BTreeMap<&str, Attr> = BTreeMap::new();

   map.insert("id", Attr::Str("hello-world"));
   map.insert("class", Attr::Vec(vec![Attr::Str("first"), Attr::Str("second")]));

   let class = Attr::Vec(vec![Attr::Str("first"), Attr::Str("second")]);

   let mut render_list = vec![];

   render_attribute(&class, &mut render_list);

   for item in render_list {
      println!("{}", item);
   }
}

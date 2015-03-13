extern crate teleport;


use teleport::{push_attribute_value, AttributeValue};

pub fn main() {
   let class = AttributeValue::List(vec![
      AttributeValue::StaticString("first"),
      AttributeValue::StaticString("second")
   ]);

   let mut render_list = vec![];

   push_attribute_value(&class, &mut render_list);

   for item in render_list {
      print!("{}", item);
   }

   println!("");
}

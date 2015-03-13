extern crate teleport;


use teleport::{push_attribute_value, AttributeValue};


pub fn main() {
   let class = AttributeValue::List(vec![
      AttributeValue::StaticString("first"),
      AttributeValue::StaticString("second")
   ]);

   let mut rendered = String::new();

   push_attribute_value(&class, &mut rendered);

   println!("{}", rendered);
}

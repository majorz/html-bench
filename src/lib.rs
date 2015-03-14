#[derive(Debug)]
pub struct Attribute {
   pub name: &'static str,
   pub value: AttributeValue,
}


#[derive(Debug)]
pub enum AttributeValue {
   StaticString(&'static str),
   List(Vec<AttributeValue>),
   Boolean(bool),
}


pub fn push_attribute(attribute: &Attribute, rendered: &mut String) {
   match attribute.value {
      AttributeValue::Boolean(true) => {
         rendered.push_str(" ");
         rendered.push_str(attribute.name);
      },
      AttributeValue::Boolean(false) => {},
      _ => {
         rendered.push_str(" ");
         rendered.push_str(attribute.name);
         rendered.push_str("=");
         rendered.push_str("\"");
         push_attribute_value(&attribute.value, rendered);
         rendered.push_str("\"");
      },
   }
}


pub fn push_attribute_value(value: &AttributeValue, rendered: &mut String) {
   match *value {
      AttributeValue::StaticString(s) => {
         rendered.push_str(&s)
      },
      AttributeValue::List(ref list) => {
         for inner in list {
            push_attribute_value(&inner, rendered)
         }
      },
      _ => {},
   }
}


#[macro_export]
macro_rules! attr {
   ( $( $name:expr, $value:expr );* ) => {
      {
         let mut temp_vec = Vec::new();
         $(
            let attribute = Attribute {
               name: $name,
               value: AttributeValue::StaticString($value),
            };
            temp_vec.push(attribute);
         )*
         temp_vec
      }
   };
}

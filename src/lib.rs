
pub struct Attribute {
   name: &'static str,
   value: AttributeValue,
}

pub enum AttributeValue {
   StaticString(&'static str),
   List(Vec<AttributeValue>),
   Boolean(bool),
}

pub fn push_attribute(attr: &Attribute, rendered: &mut String) {
   match attr.value {
      AttributeValue::Boolean(true) => {

      },
      _ => {},
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

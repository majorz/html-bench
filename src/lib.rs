
pub struct Attribute {
   pub name: &'static str,
   pub value: AttributeValue,
}

pub enum AttributeValue {
   StaticString(&'static str),
   List(Vec<AttributeValue>),
   Boolean(bool),
}

pub fn push_attribute(attr: &Attribute, rendered: &mut String) {
   match attr.value {
      AttributeValue::Boolean(true) => {
         rendered.push_str(" ");
         rendered.push_str(attr.name);
      },
      AttributeValue::Boolean(false) => {},
      _ => {
         rendered.push_str(" ");
         rendered.push_str(attr.name);
         rendered.push_str("=");
         rendered.push_str("\"");
         push_attribute_value(&attr.value, rendered);
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


pub enum AttributeValue {
   StaticString(&'static str),
   List(Vec<AttributeValue>)
}

pub fn push_attribute_value(attr: &AttributeValue, render_list: &mut Vec<String>) {
   match *attr {
      AttributeValue::StaticString(val) => {
         render_list.push(val.to_string())
      },
      AttributeValue::List(ref attrs) => {
         for inner in attrs {
            push_attribute_value(&inner, render_list)
         }
      }
   }
}

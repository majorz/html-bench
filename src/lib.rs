pub enum Attr {
   Str(&'static str),
   Vec(Vec<Attr>)
}

pub fn render_attribute(attr: &Attr, render_list: &mut Vec<String>) {
   match *attr {
      Attr::Str(val) => {
         render_list.push(val.to_string())
      },
      Attr::Vec(ref attrs) => {
         for inner in attrs {
            render_attribute(&inner, render_list)
         }
      }
   }
}

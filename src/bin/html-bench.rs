#![feature(test)]
#![feature(plugin)]

#![plugin(maud_macros)]

extern crate test;
extern crate maud;
extern crate handlebars;
extern crate "rustc-serialize" as serialize;

#[macro_use]
extern crate "html-bench" as htmlbench;


fn render_table() {
   let mut table_contents: Vec<[&'static str; 10]> = Vec::new();
   for _ in 0..1000 {
      table_contents.push(["aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj"])
   }

   let mut list = Vec::new();
   list.push("<table>\n");

   for row in table_contents.iter() {
      list.push("  <tr>\n");

      for item in row {
         list.push("    <td>");
         list.push(item);
         list.push("<td>\n");
      }

      list.push("  </tr>\n");
   }

   list.push("</table>");

   let result: String = list.concat();

   println!("{}", result);
}


pub fn main() {
   render_table();
}


#[cfg(test)]
mod bench {
   use std::fmt::Write;
   use std::collections::BTreeMap;
   use test::Bencher;

   use serialize::json::{Json, ToJson};

   use handlebars::Handlebars;

   #[bench]
   fn concat_string(b: &mut Bencher) {
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
   fn concat_vector_string(b: &mut Bencher) {
      let mut table_contents: Vec<[&'static str; 10]> = Vec::new();
      for _ in 0..1000 {
         table_contents.push(["aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj"])
      }

      b.iter(|| {
         let mut list = Vec::new();
         list.push("<table>\n");

         for row in table_contents.iter() {
            list.push("  <tr>\n");

            for item in row {
               list.push("    <td>");
               list.push(item);
               list.push("<td>\n");
            }

            list.push("  </tr>\n");
         }

         list.push("</table>");

         let result: String = list.concat();

         result
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


   #[bench]
   fn handlebars_render(b: &mut Bencher) {
      let mut table_vec = Vec::new();
      for _ in 0..1000 {
         let mut row = BTreeMap::new();
         let array = Json::Array(vec![
            Json::String("aa".to_string()),
            Json::String("bb".to_string()),
            Json::String("cc".to_string()),
            Json::String("dd".to_string()),
            Json::String("ee".to_string()),
            Json::String("ff".to_string()),
            Json::String("gg".to_string()),
            Json::String("hh".to_string()),
            Json::String("ii".to_string()),
            Json::String("jj".to_string())
         ]);
         row.insert("row".to_string(), array.to_json());
         table_vec.push(row);
      }
      //let table_contents = Json::Array(table_vec);

      let mut data = BTreeMap::new();
      data.insert("table".to_string(), table_vec.to_json());

      let mut handlebars = Handlebars::new();
      let source = " \
      <table> \n\
      {{#each table}} \
         <tr> \n\
            {{#each row}} \
               <td>{{this}}</td> \n\
            {{/each}} \
         </tr> \n\
      {{/each}} \
      </table> \
      ";

      handlebars.register_template_string("table", source.to_string())
         .ok().expect("template created");

      b.iter(|| {
         handlebars.render("table", &data).ok().unwrap()
      });
   }
}

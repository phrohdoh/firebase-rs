extern crate firebase;
extern crate hyper;

use firebase::Firebase;

fn main() {
    let fb = match Firebase::new("https://your-db-name.firebaseio.com/") {
        Ok(f) => f,
        Err(e) => panic!(e),
    };

    let node1 = fb.at("toplevel_node1").unwrap();
    let set1 = node1.set("[ \"foo\", \"bar\", \"baz\", \"qux\" ]").unwrap();
    println!("{:?}", set1);

    let node2_text = "{ \"glossary\": { \"title\": \"example glossary\", \"GlossDiv\": { \"title\": \"S\", \"GlossList\": { \"GlossEntry\": { \"ID\": \"SGML\", \"SortAs\": \"SGML\", \"GlossTerm\": \"Standard Generalized Markup Language\", \"Acronym\": \"SGML\", \"Abbrev\": \"ISO 8879:1986\", \"GlossDef\": { \"para\": \"A meta-markup language, used to create markup languages such as DocBook.\", \"GlossSeeAlso\": [ \"GML\", \"XML\" ] }, \"GlossSee\": \"markup\" } } } } }";
    println!("{:?}", fb.at("toplevel_node2").unwrap().set(node2_text).unwrap());

    println!("{:?}", node1.push("[ \"push!\" ]").unwrap());
}

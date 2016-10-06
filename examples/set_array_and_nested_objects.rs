extern crate firebase;
extern crate hyper;

use firebase::Firebase;
use hyper::status::StatusCode;

fn main() {
    let fb = match Firebase::authed("https://your-db-name.firebaseio.com/", "gear-icon -> project-settings -> database -> click 'show' just under 'add secret', paste that key here") {
        Ok(f) => f,
        Err(e) => panic!(e),
    };

    let node1 = fb.at("toplevel_node1").unwrap();
    let set1 = node1.set("[ \"foo\", \"bar\", \"baz\", \"qux\" ]").unwrap();

    if set1.code == StatusCode::NotFound {
        println!("You need to setup the database connection / auth strings for this example!");
        return;
    }

    println!("{:?}", set1);

    let node2_text = "{ \"glossary\": { \"title\": \"example glossary\", \"GlossDiv\": { \"title\": \"S\", \"GlossList\": { \"GlossEntry\": { \"ID\": \"SGML\", \"SortAs\": \"SGML\", \"GlossTerm\": \"Standard Generalized Markup Language\", \"Acronym\": \"SGML\", \"Abbrev\": \"ISO 8879:1986\", \"GlossDef\": { \"para\": \"A meta-markup language, used to create markup languages such as DocBook.\", \"GlossSeeAlso\": [ \"GML\", \"XML\" ] }, \"GlossSee\": \"markup\" } } } } }";
    println!("{:?}", fb.at("toplevel_node2").unwrap().set(node2_text).unwrap());

    println!("{:?}", node1.push("[ \"push!\" ]").unwrap());
}

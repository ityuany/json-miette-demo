use biome_json_parser::parse_json;
use biome_json_syntax::{JsonParserOptions, JsonSyntax};

fn main() {
    let json = r#"
    {
        "name":"demo"
    }
    "#;

    let parse = parse_json(json, JsonParserOptions::default());

    println!("-->{:#?}", parse)
}

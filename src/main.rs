use biome_json_parser::{JsonParserOptions, parse_json};
use miette::{LabeledSpan, SourceSpan, miette};

fn main() {
    let json = r#"
    {
        "name": "@shined/doctor",
	"version": "0.0.9",
	"main": "index.js",
	"types": "index.d.ts",
	"napi": {
		"binaryName": "doctor",
		"packageName": "@shined/doctor",
		"targets": [
			"x86_64-apple-darwin",
			"aarch64-apple-darwin",
			"x86_64-pc-windows-msvc",
			"aarch64-pc-windows-msvc",
			"x86_64-unknown-linux-gnu",
			"x86_64-unknown-linux-musl",
			"aarch64-unknown-linux-gnu",
			"aarch64-unknown-linux-musl"
		]
	},
	"bin": "./bin/index.mjs"
    }
    "#;

    let parse = parse_json(json, JsonParserOptions::default());

    let root = parse.tree();

    if let Some(obj_node) = root.value().unwrap().as_json_object_value() {
        for member in obj_node.json_member_list() {
            let member = member.ok().unwrap();
            let name = member.name().ok().unwrap();
            let value = member.value().ok().unwrap();

            let name = name.inner_string_text().unwrap().to_string();

            // let value = value.as_json_string_value().unwrap().to_string();
            if name == "name" {
                let n = member.value().unwrap();
                let a = n.as_json_string_value().unwrap();

                let token = a.value_token().unwrap();

                let range = token.text_trimmed_range();

                let start_byte: usize = range.start().into();
                let end_byte: usize = range.end().into();

                let span = SourceSpan::new(start_byte.into(), end_byte - start_byte);

                let source = json.to_string();
                let report = miette!(
                    labels = vec![LabeledSpan::at(span, "this should be 6"),],
                    help = "'*' has greater precedence than '+'",
                    "Wrong answer"
                )
                .with_source_code(source);
                println!("{:?}", report)
            }
        }
    }
}

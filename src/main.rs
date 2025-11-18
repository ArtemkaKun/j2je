use clap::Parser;
use serde_json::Value;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, required = true)]
    json_string: String,
}

fn main() {
    let args = Args::parse();

    let parsed: Value = serde_json::from_str(args.json_string.as_str()).expect("Failed to parse JSON");
    let output = convert_to_hcl(&parsed, 4);

    println!("jsonencode({})", output);
}

fn convert_to_hcl(value: &Value, indent_level: usize) -> String {
    match value {
        Value::Object(map) => {
            let mut lines = Vec::new();
            let indent = " ".repeat(indent_level);

            for (key, val) in map {
                let val_str = convert_to_hcl(val, indent_level + 1);
                lines.push(format!("{}\"{}\" = {}", " ".repeat(indent_level), key, val_str));
            }

            format!("{{\n{}\n{}}}", lines.join("\n"), indent)
        }

        Value::String(s) => {
            if s.starts_with("${") && s.ends_with("}") {
                s[2..s.len() - 1].to_string()
            } else {
                format!("\"{}\"", s)
            }
        }

        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),

        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(|v| convert_to_hcl(v, indent_level)).collect();
            format!("[{}]", items.join(", "))
        }
    }
}

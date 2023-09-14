use std::fs::File;
use std::io::Write;

macro_rules! generate_ts_function {
  (fn $name:ident($($arg_name:ident: $arg_type:ident),*) -> $return_type:ident $body:block) => {
    // Mapping Rust types to TypeScript types
    let rust_to_ts_types = |rust_type: &'static str| -> &str {
      match rust_type {
        "i32" => "number",
        // Add other mappings here as needed
        _ => rust_type,
      }
    };

    // Generate the argument string for TypeScript
    let args: Vec<String> = vec![
      $(
        format!(
          "{}: {}",
          stringify!($arg_name),
          rust_to_ts_types(stringify!($arg_type))
        )
      ),*
    ];

    let code = format!("function {}({}): {} {{\n    {}\n}}\n",
      stringify!($name),
      args.join(", "),
      rust_to_ts_types(stringify!($return_type)),
      stringify!($body).replace("{", "").replace("}", "")
    );

    // Write TypeScript code to file
    let mut file = File::create("output.ts").expect("Unable to create TypeScript file");
    file.write_all(code.as_bytes()).expect("Unable to write to TypeScript file");
  };

}

macro_rules! generate_dart_function {
  (fn $name:ident($($arg_name:ident: $arg_type:ident),*) -> $return_type:ident $body:block) => {{
    // Mapping Rust types to Dart types
    let rust_to_dart_types = |rust_type: &'static str| -> &str {
      match rust_type {
        "i32" => "int",
        // Add other mappings here as needed
        _ => rust_type,
      }
    };

    // Generate the argument string
    let args: Vec<String> = vec![
      $(
        format!(
          "{} {}",
          rust_to_dart_types(stringify!($arg_type)),
          stringify!($arg_name)
        )
      ),*
    ];

    let code = format!("{} {}({}) {{\n    {}\n}}\n",
      rust_to_dart_types(stringify!($return_type)),
      stringify!($name),
      args.join(", "), // Join each argument with a comma
      stringify!($body).replace("{", "").replace("}", "")
    );

    // Open a file to write to
    let mut file = File::create("output.dart").expect("Unable to create file");

    // Write the code to the file
    file.write_all(code.as_bytes()).expect("Unable to write to file");
  }};
}

macro_rules! code_generate {
  ($($input:tt)*) => {
    generate_dart_function! {$($input)*}
    generate_ts_function! {$($input)*}
  };
}


pub fn generate_code() {
  code_generate! {
    fn add(x: i32, y: i32) -> i32 {
      return x + y;
    }
  }
}

use proc_macro::TokenStream;
use std::ffi::OsString;
use std::fs;

#[proc_macro]
pub fn match_solutions(_: TokenStream) -> TokenStream {
    let dir = fs::read_dir("src/solutions").expect("Couldn't read solution directory");
    let solutions = dir
        .filter(|entry| {
            entry
                .as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap_or_default()
                .ends_with(".rs")
        })
        .map(|entry| entry.expect("Couldn't read entry").file_name())
        .collect::<Vec<OsString>>();

    let mut out = String::from("return match arg {\n");
    for file in solutions {
        let file_name = match file.to_str() {
            Some(str) => {
                if str.starts_with("solution_") {
                    str
                } else {
                    continue;
                }
            }
            None => continue,
        };
        out += "    \"";
        out += &file_name[9..12];
        out += "\" => Ok(String::from(";
        out += &file_name[0..12];
        out += "::solve(&input))),\n";
    }
    out += "    _ => Err(Error::new(
                ErrorKind::NotFound,
                String::from(\"Could not find solution for \") + arg,
            ))\n";
    out += "};\n";

    out.parse().unwrap()
}

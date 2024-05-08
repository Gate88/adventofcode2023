extern crate proc_macro;
use proc_macro::TokenStream;
use std::fs;

#[proc_macro]
pub fn include_all_day_files(_item: TokenStream) -> TokenStream {
    let paths = fs::read_dir(format!(r"{}\src\", env!("CARGO_MANIFEST_DIR"))).unwrap();
    let mut sorted = paths
        .filter_map(|p| {
            let file_name = p.unwrap().file_name().to_str().unwrap().to_owned();
            if file_name.starts_with("day") && file_name.ends_with(".rs") {
                return Some(file_name.strip_suffix(".rs").unwrap().to_owned());
            }
            return None;
        })
        .collect::<Vec<String>>();
    sorted.sort_unstable();
    let arguments = sorted
        .iter()
        .map(|a| format!("{}, {}", a, a.strip_prefix("day").unwrap()))
        .collect::<Vec<String>>()
        .join(", ");
    format!("main_day!(run_day, get_default_day, {});", arguments)
        .parse()
        .unwrap()
}

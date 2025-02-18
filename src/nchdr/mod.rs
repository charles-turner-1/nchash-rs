mod attrs;
mod dims;
mod vars;

use attrs::get_attr_info;
use dims::get_dim_info;
use netcdf::{self};
use std::env;
use std::error::Error;
use std::path::Path;
use vars::get_var_info;

pub fn nchdr(fname: String) -> Result<String, Box<dyn Error>> {
    // Dynamically build back up the output of ncdump -h $fname

    let mut ncdump = String::new();

    let f_path = Path::new(fname.as_str());

    let file: netcdf::File = netcdf::open(f_path)?;

    let file_stem = extract_fname(&f_path)?;
    let preamble =  print_skeleton_opening(&file_stem);

    let variables: Vec<_> = file.variables().collect();
    let dimensions: Vec<_> = file.dimensions().collect();
    let global_attrs: Vec<_> = file.attributes().collect();

    println!("{}", "dimensions:");
    for dim in dimensions {
        get_dim_info(&file, &dim.name());
    }

    println!("{}", "variables:");
    for var in variables {
        get_var_info(&file, &var.name());
    }

    println!("{}", "\n// global attributes:");
    for attr in global_attrs {
        get_attr_info(&file, attr.name())
    }
    print_skeleton_close();

    Ok("lalala".to_string())
}

fn print_skeleton_opening(f_stem: &str) -> String {
    format!("netcdf {} {{ ", f_stem).to_string()

}

fn print_skeleton_close() -> String { 
    "}}\n".to_string()
}

fn extract_fname<'a>(f_handle: &'a std::path::Path) -> Result<&str, &str> {
    // Take the file handle, get the file stem, extract it.

    match f_handle.file_stem() {
        Some(stem) => match stem.to_str() {
            Some(stem_str) => Ok(stem_str),
            None => Err("Could not convert file stem to valid utf-8 string"),
        },
        None => Err("Could not identify filename"),
    }
}

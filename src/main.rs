use std::fs::File;
use std::io::{Write};
use std::process::{ Command };
use std::env;
use std::path::{ Path };

use structopt::StructOpt;
use serde::{ Deserialize, Serialize };
use serde_json::Result;


#[derive(Debug, Serialize, Deserialize)]
struct Arg {
    arg_name: String,
    arg_value: String
}

#[derive(Debug, Serialize, Deserialize)]
struct CodeX {
    cmd_name: String,
    args: Vec<Arg>,
}

#[derive(StructOpt)]
struct Cli {
    project_url: String
}


fn get_file(file_path: &Path) -> File {

    let default_data =
r#"{
    "cmd_name": "",
    "args": [
        {
            "arg_name": "",
            "arg_value": ""
        }
    ]
}"#;

    if !file_path.exists() {
         let mut file = match File::create(file_path) {
            Err(why) => panic!("couldn't create file {}: {}", file_path.display(), why),
            Ok(file) => file
        };

        file.write_all(default_data.as_bytes()).expect("write file failed.");
        println!("please finish config file and run again");
    }

    match File::open(file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path.display(), why),
        Ok(file) => file
    }
}

fn read_codex_from_file(file: File) -> Result<CodeX> {

    let result = match serde_json::from_reader(&file) {
        Err(why) => panic!("couldn't read from reader, {}", why),
        Ok(res) => res
    };

    Ok(result)
}

fn main() {

    let exe_path = env::current_exe().unwrap();
    let base_path = exe_path.parent().unwrap();
    let file_path = base_path.join("config.json");
    let file = get_file(&file_path);

    let config: CodeX = read_codex_from_file(file).unwrap();
    let mut cmd: String = config.cmd_name + " ";

    for s in config.args {
        cmd += &s.arg_name;
        cmd += " ";
        cmd += &s.arg_value;
        cmd += " ";
    }

    let cli_input = Cli::from_args();
    cmd += &cli_input.project_url;

    Command::new("powershell")
        .arg(cmd)
        .spawn()
        .expect("open failed");
}

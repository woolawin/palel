use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::{fs, path::Path};
use walkdir::WalkDir;

use crate::c::CSrc;
use crate::compilation_error::{
    CompilationError, FailedToReadSrcFile, FailedToWriteToFile, NoSourceFiles,
};
use crate::core::Of;
use crate::downstream_compiler_c::downstream_compile;
use crate::palel::Src;
use crate::parser::parse;
use crate::renderer_c::render;
use crate::toolkit_c::CToolKit;
use crate::transpiler_c::transpile;

pub struct BuildTaskConfig {
    pub bin_name: String,
    pub src_dir: String,
    pub dest_dir: String,
}

pub struct BuildTask {
    pub config: BuildTaskConfig,
    src_files: Vec<SrcFile>,
}

#[derive(Debug, PartialEq)]
pub struct SrcFile {
    pub file: String,
    pub content: String,
}

pub fn default_build_task_config() -> BuildTaskConfig {
    let default_bin_name = "build-artifact".to_string();
    let bin_name = match env::current_dir() {
        Err(_) => None,
        Ok(dir) => match dir.file_name() {
            Some(value) => Some(value.to_string_lossy().to_string()),
            None => None,
        },
    };
    BuildTaskConfig {
        bin_name: bin_name.unwrap_or(default_bin_name).to_string(),
        src_dir: "./src".to_string(),
        dest_dir: "./build".to_string(),
    }
}

pub fn create_build_task(config: BuildTaskConfig) -> BuildTask {
    return BuildTask {
        config: config,
        src_files: Vec::new(),
    };
}

pub fn run(task: &mut BuildTask) -> Option<Box<dyn CompilationError>> {
    if let Some(err) = load(task) {
        return Some(err);
    }
    if let Some(err) = execute(task) {
        return Some(err);
    }
    None
}

fn load(task: &mut BuildTask) -> Option<Box<dyn CompilationError>> {
    let src_dir = Path::new(task.config.src_dir.as_str());
    for entry in WalkDir::new(src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_name = entry
            .path()
            .strip_prefix(src_dir)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if !file_name.ends_with(".palel") {
            continue;
        }
        let file_contents = match fs::read_to_string(entry.path()) {
            Ok(contents) => contents,
            Err(_) => {
                return Some(Box::new(FailedToReadSrcFile {
                    file: file_name.clone(),
                }));
            }
        };
        task.src_files.push(SrcFile {
            file: file_name,
            content: file_contents,
        });
    }

    if task.src_files.is_empty() {
        Some(Box::new(NoSourceFiles {
            dir: task.config.src_dir.clone(),
        }))
    } else {
        None
    }
}

fn execute(task: &BuildTask) -> Option<Box<dyn CompilationError>> {
    let mut src = Src::default();
    for file in &task.src_files {
        if let Some(err) = parse(&mut src, &file) {
            return Some(err);
        }
    }
    let toolkit = CToolKit {};
    let result = match transpile(&src, &toolkit) {
        Of::Ok(tp) => tp,
        Of::Error(err) => return Some(err),
    };
    let output_file = format!("{}/code/main.c", task.config.dest_dir);
    if let Some(err) = write(&output_file, &result) {
        return Some(err);
    }
    downstream_compile(&output_file, &task.config.bin_name)
}

fn write(output_file: &String, src: &CSrc) -> Option<Box<dyn CompilationError>> {
    let err = FailedToWriteToFile {
        file: output_file.to_string(),
    };
    let path = Path::new(output_file.as_str());
    if let Some(parent) = path.parent() {
        if let Err(_) = fs::create_dir_all(parent) {
            return Some(Box::new(err));
        }
    }
    let file_open = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path);

    let mut file = match file_open {
        Ok(value) => value,
        Err(_) => return Some(Box::new(err)),
    };

    if let Err(_) = file.write_all(render(src).as_bytes()) {
        return Some(Box::new(err));
    } else {
        None
    }
}

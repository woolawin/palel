use std::{fs, path::Path};
use walkdir::WalkDir;

use crate::compilation_error::{CompilationError, FailedToReadSrcFile, NoSourceFiles};

#[derive(Debug, PartialEq)]
pub struct Project {
    pub src_dir: String,
    pub build_dir: String,
    pub src_files: Vec<SrcFile>,
}

#[derive(Debug, PartialEq)]
pub struct SrcFile {
    pub file: String,
    pub content: String,
}

fn default_project() -> Project {
    return Project {
        src_dir: "./src".to_string(),
        build_dir: "./build".to_string(),
        src_files: Vec::new(),
    };
}

impl Default for Project {
    fn default() -> Self {
        default_project()
    }
}

pub fn load(project: &mut Project) -> Option<Box<dyn CompilationError>> {
    let src_dir = Path::new(project.src_dir.as_str());
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
        project.src_files.push(SrcFile {
            file: file_name,
            content: file_contents,
        });
    }

    if project.src_files.is_empty() {
        Some(Box::new(NoSourceFiles {
            dir: project.src_dir.clone(),
        }))
    } else {
        None
    }
}

use crate::c::{CInclude, CSrc, CSrcPatch};

impl CSrcPatch {
    pub fn merge(&mut self, other: &CSrcPatch) {
        for include in &other.includes {
            src_patch_include(self, include);
        }
    }
}

impl CSrc {
    pub fn merge(&mut self, patch: &CSrcPatch) {
        for include in &patch.includes {
            patch_include(self, include);
        }
    }
}

fn patch_include(src: &mut CSrc, include: &CInclude) {
    for existing in &src.includes {
        if existing.file == include.file {
            return;
        }
    }
    src.includes.push(include.clone())
}

fn src_patch_include(src: &mut CSrcPatch, include: &CInclude) {
    for existing in &src.includes {
        if existing.file == include.file {
            return;
        }
    }
    src.includes.push(include.clone())
}

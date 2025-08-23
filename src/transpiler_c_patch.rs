use crate::c::{CInclude, CSrc, CSrcPatch};

pub fn patch_src(src: &mut CSrc, patch: &CSrcPatch) {
    for include in &patch.includes {
        patch_include(&mut src.includes, include);
    }
}

pub fn merge_patch(patch: &mut CSrcPatch, other: &CSrcPatch) {
    for include in &other.includes {
        patch_include(&mut patch.includes, include);
    }
}

fn patch_include(includes: &mut Vec<CInclude>, include: &CInclude) {
    for existing in includes.iter() {
        if existing.file == include.file {
            return;
        }
    }
    includes.push(include.clone())
}

use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Show current package manager using NativeScript CLI."####.into(),command: r####"ns package-manager"####.into(),tags: vec![r####"nativescript"####.into()].into_iter().collect(),description: Some(r####"NativeScript prints the value of the current package manager."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://github.com/NativeScript/nativescript-cli/blob/master/docs/man_pages/general/package-manager.md"####.into()),author: Some(r####"erodriguezh"####.into()),author_url: Some(r####"https://github.com/erodriguezh"####.into()),shells: vec![].into_iter().collect(),}
}

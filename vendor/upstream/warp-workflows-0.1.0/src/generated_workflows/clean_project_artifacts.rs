use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Clean your NativeScript project artifacts."####.into(),command: r####"ns clean"####.into(),tags: vec![r####"nativescript"####.into()].into_iter().collect(),description: Some(r####"Cleans your NativeScript project artifacts."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://github.com/NativeScript/nativescript-cli/blob/master/docs/man_pages/general/clean.md"####.into()),author: Some(r####"erodriguezh"####.into()),author_url: Some(r####"https://github.com/erodriguezh"####.into()),shells: vec![].into_iter().collect(),}
}

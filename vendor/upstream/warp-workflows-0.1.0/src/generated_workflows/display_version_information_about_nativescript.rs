use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Display version information about NativeScript."####.into(),command: r####"ns info"####.into(),tags: vec![r####"nativescript"####.into()].into_iter().collect(),description: Some(r####"Displays version information about the NativeScript CLI, core modules, and runtimes."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://github.com/NativeScript/nativescript-cli/blob/master/docs/man_pages/general/info.md"####.into()),author: Some(r####"erodriguezh"####.into()),author_url: Some(r####"https://github.com/erodriguezh"####.into()),shells: vec![].into_iter().collect(),}
}

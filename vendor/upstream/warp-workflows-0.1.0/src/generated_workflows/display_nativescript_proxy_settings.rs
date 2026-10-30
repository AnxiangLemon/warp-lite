use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Display proxy settings of the NativeScript CLI."####.into(),command: r####"ns proxy"####.into(),tags: vec![r####"nativescript"####.into()].into_iter().collect(),description: Some(r####"Displays the current proxy settings of the NativeScript CLI."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://github.com/NativeScript/nativescript-cli/blob/master/docs/man_pages/general/proxy.md"####.into()),author: Some(r####"erodriguezh"####.into()),author_url: Some(r####"https://github.com/erodriguezh"####.into()),shells: vec![].into_iter().collect(),}
}

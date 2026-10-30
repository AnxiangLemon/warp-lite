use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Lists all platforms that your NativeScript project currently targets."####.into(),command: r####"ns platform list"####.into(),tags: vec![r####"nativescript"####.into()].into_iter().collect(),description: Some(r####"Lists all platforms that your NativeScript project currently targets. You can build and deploy your project only for these target platforms."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://github.com/NativeScript/nativescript-cli/blob/master/docs/man_pages/project/configuration/platform.md"####.into()),author: Some(r####"erodriguezh"####.into()),author_url: Some(r####"https://github.com/erodriguezh"####.into()),shells: vec![].into_iter().collect(),}
}

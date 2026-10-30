use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Configure NativeScript CLI usage reporting."####.into(),command: r####"ns usage-reporting"####.into(),tags: vec![r####"nativescript"####.into()].into_iter().collect(),description: Some(r####"Configures anonymous usage reporting for the NativeScript CLI."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://github.com/NativeScript/nativescript-cli/blob/master/docs/man_pages/general/usage-reporting.md"####.into()),author: Some(r####"erodriguezh"####.into()),author_url: Some(r####"https://github.com/erodriguezh"####.into()),shells: vec![].into_iter().collect(),}
}

use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"List applications installed on a device using NativeScript CLI."####.into(),command: r####"ns device list-applications"####.into(),tags: vec![r####"nativescript"####.into()].into_iter().collect(),description: Some(r####"NativeScript lists the installed applications on all connected Android and iOS devices."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://github.com/NativeScript/nativescript-cli/blob/master/docs/man_pages/device/device-list-applications.md"####.into()),author: Some(r####"erodriguezh"####.into()),author_url: Some(r####"https://github.com/erodriguezh"####.into()),shells: vec![].into_iter().collect(),}
}

use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Open the device log stream using NativeScript CLI."####.into(),command: r####"ns device log"####.into(),tags: vec![r####"nativescript"####.into()].into_iter().collect(),description: Some(r####"NativeScript opens the device log stream for a selected connected Android or iOS device."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://github.com/NativeScript/nativescript-cli/blob/master/docs/man_pages/device/device-log.md"####.into()),author: Some(r####"erodriguezh"####.into()),author_url: Some(r####"https://github.com/erodriguezh"####.into()),shells: vec![].into_iter().collect(),}
}

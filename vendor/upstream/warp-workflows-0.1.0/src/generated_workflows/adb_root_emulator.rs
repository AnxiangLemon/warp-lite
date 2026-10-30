use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Root your emulator"####.into(),
        command: r####"adb root"####.into(),
        tags: vec![r####"android"####.into(), r####"adb"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Roots your Android emulator"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: None,
        author: Some(r####"Odin Asbjørnsen"####.into()),
        author_url: Some(r####"https://github.com/oas004"####.into()),
        shells: vec![].into_iter().collect(),
    }
}

use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Clear Xcode Derived Data"####.into(),
        command: r####"rm -rf ~/Library/Developer/Xcode/DerivedData"####.into(),
        tags: vec![r####"ios"####.into(), r####"xcode"####.into()]
            .into_iter()
            .collect(),
        description: None,
        arguments: vec![].into_iter().collect(),
        source_url: None,
        author: None,
        author_url: None,
        shells: vec![].into_iter().collect(),
    }
}

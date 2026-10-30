use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Synchronize upstream branch"####.into(),
        command: r####"git push -u origin HEAD"####.into(),
        tags: vec![r####"git"####.into()].into_iter().collect(),
        description: Some(
            r####"Sync the local branch to the remote branch with the same name."####.into(),
        ),
        arguments: vec![].into_iter().collect(),
        source_url: None,
        author: None,
        author_url: None,
        shells: vec![].into_iter().collect(),
    }
}

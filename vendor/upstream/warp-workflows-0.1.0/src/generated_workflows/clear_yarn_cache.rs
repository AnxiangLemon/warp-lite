use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Clear Yarn cache"####.into(),
        command: r####"yarn cache clean"####.into(),
        tags: vec![r####"yarn"####.into()].into_iter().collect(),
        description: Some(r####"Clears the global yarn cache directory."####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(
            r####"https://stackoverflow.com/questions/39991508/how-to-clear-cache-in-yarnyar"####
                .into(),
        ),
        author: Some(r####"KhaledMohamedP"####.into()),
        author_url: Some(r####"https://stackoverflow.com/users/3105145/khaledmohamedp"####.into()),
        shells: vec![].into_iter().collect(),
    }
}

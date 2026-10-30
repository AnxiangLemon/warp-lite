use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Graphite - Restacks the current branch to its parent"####.into(),command: r####"gt branch restack"####.into(),tags: vec![r####"graphite"####.into()].into_iter().collect(),description: Some(r####"Alias: `br`. Ensure the current branch is based on its parent, rebasing if necessary."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://graphite.dev/docs/command-reference"####.into()),author: Some(r####"graphite"####.into()),author_url: Some(r####"https://graphite.dev/"####.into()),shells: vec![].into_iter().collect(),}
}

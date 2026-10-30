use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Graphite - Pushes (force) the current branch to GitHub"####.into(),command: r####"gt branch submit"####.into(),tags: vec![r####"graphite"####.into()].into_iter().collect(),description: Some(r####"Alias: `bs`. Idempotently force push the current branch to GitHub, creating or updating a pull request."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://graphite.dev/docs/command-reference"####.into()),author: Some(r####"graphite"####.into()),author_url: Some(r####"https://graphite.dev/"####.into()),shells: vec![].into_iter().collect(),}
}

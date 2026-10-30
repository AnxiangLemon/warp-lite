use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Graphite - Rebase the parent branches"####.into(),command: r####"gt downstack restack"####.into(),tags: vec![r####"graphite"####.into()].into_iter().collect(),description: Some(r####"Alias `dsr`. From trunk to the current branch, ensure each is based on its parent, rebasing if necessary."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://graphite.dev/docs/command-reference"####.into()),author: Some(r####"graphite"####.into()),author_url: Some(r####"https://graphite.dev/"####.into()),shells: vec![].into_iter().collect(),}
}

use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Graphite - Synchronize with remote"####.into(),command: r####"gt rs -r"####.into(),tags: vec![r####"graphite"####.into()].into_iter().collect(),description: Some(r####"A quick command to pull changes from your trunk branch and subsequently restack upstack changes. Same as `gt repo sync && gt stack restack` under the hood. Also deletes any branches that have been merged."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://graphite.dev/docs/command-reference"####.into()),author: Some(r####"graphite"####.into()),author_url: Some(r####"https://graphite.dev/"####.into()),shells: vec![].into_iter().collect(),}
}

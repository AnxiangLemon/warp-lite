use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"List all installed Homebrew formulae/casks"####.into(),
        command: r####"brew list"####.into(),
        tags: vec![r####"homebrew"####.into()].into_iter().collect(),
        description: Some(r####"Lists all installed Homebrew formulae/casks"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://docs.brew.sh/Manpage"####.into()),
        author: Some(r####"Wyatt-Stanke"####.into()),
        author_url: Some(r####"https://github.com/Wyatt-Stanke"####.into()),
        shells: vec![].into_iter().collect(),
    }
}

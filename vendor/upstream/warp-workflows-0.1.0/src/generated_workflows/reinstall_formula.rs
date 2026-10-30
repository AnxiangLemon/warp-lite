use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Reinstall a Homebrew formula"####.into(),
        command: r####"brew reinstall {{formula_name}}"####.into(),
        tags: vec![r####"homebrew"####.into()].into_iter().collect(),
        description: Some(r####"Reinstall a Homebrew formula"####.into()),
        arguments: vec![Argument {
            name: r####"formula_name"####.into(),
            description: Some(r####"The formula to reinstall"####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://docs.brew.sh/Manpage"####.into()),
        author: Some(r####"Wyatt-Stanke"####.into()),
        author_url: Some(r####"https://github.com/Wyatt-Stanke"####.into()),
        shells: vec![].into_iter().collect(),
    }
}

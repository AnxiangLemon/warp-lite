use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Unpin a homebrew formula"####.into(),command: r####"brew unpin {{formula_name}}"####.into(),tags: vec![r####"homebrew"####.into()].into_iter().collect(),description: Some(r####"Unpins a version of a homebrew formula to its current version, i.e. it will not be updated when a newer version is available."####.into()),arguments: vec![Argument {name: r####"formula_name"####.into(),description: Some(r####"The formula to unpin"####.into()),default_value: None,}].into_iter().collect(),source_url: Some(r####"https://docs.brew.sh/Manpage"####.into()),author: Some(r####"Wyatt-Stanke"####.into()),author_url: Some(r####"https://github.com/Wyatt-Stanke"####.into()),shells: vec![].into_iter().collect(),}
}

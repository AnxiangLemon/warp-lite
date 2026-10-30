use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Install all dependencies from Brewfile"####.into(),command: r####"brew bundle install"####.into(),tags: vec![r####"homebrew"####.into()].into_iter().collect(),description: Some(r####"Install and upgrade (by default) all dependencies from the Brewfile. Optionally you can pass [--file={{file_location}}], Otherwise homebrew finds Brewfile from the current location."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://docs.brew.sh/Manpage"####.into()),author: Some(r####"shivamkj"####.into()),author_url: Some(r####"https://github.com/shivamkj"####.into()),shells: vec![].into_iter().collect(),}
}

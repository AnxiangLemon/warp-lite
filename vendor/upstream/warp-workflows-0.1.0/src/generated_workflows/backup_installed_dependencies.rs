use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Store / Backup all installed dependencies into Brewfile"####.into(),command: r####"brew bundle dump"####.into(),tags: vec![r####"homebrew"####.into()].into_iter().collect(),description: Some(r####"Write all installed casks/formulae/images/taps into a Brewfile in the current directory."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://docs.brew.sh/Manpage"####.into()),author: Some(r####"shivamkj"####.into()),author_url: Some(r####"https://github.com/shivamkj"####.into()),shells: vec![].into_iter().collect(),}
}

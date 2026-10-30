use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Remove all untracked files and folders"####.into(),command: r####"git clean -fd"####.into(),tags: vec![r####"git"####.into()].into_iter().collect(),description: Some(r####"Remove all local untracked files and folders"####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/61212/how-to-remove-local-untracked-files-from-the-current-git-working-tree"####.into()),author: Some(r####"Robert Berger"####.into()),author_url: Some(r####"https://stackoverflow.com/users/108743/robert-berger"####.into()),shells: vec![].into_iter().collect(),}
}

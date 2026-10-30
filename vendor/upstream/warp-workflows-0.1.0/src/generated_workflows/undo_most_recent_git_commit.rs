use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Undo most recent git commit"####.into(),command: r####"git reset HEAD~"####.into(),tags: vec![r####"git"####.into()].into_iter().collect(),description: Some(r####"Undos the last git commit while leaving the working tree (the state of the files on disk) untouched."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/927358/how-do-i-undo-the-most-recent-local-commits-in-git"####.into()),author: Some(r####"Mark Amery"####.into()),author_url: Some(r####"https://stackoverflow.com/users/1709587"####.into()),shells: vec![].into_iter().collect(),}
}

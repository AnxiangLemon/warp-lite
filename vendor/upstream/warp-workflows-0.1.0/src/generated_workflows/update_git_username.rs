use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Change username of git"####.into(),command: r####"git config --global user.name '{{username}}'"####.into(),tags: vec![r####"git"####.into()].into_iter().collect(),description: Some(r####"Sets the username for git"####.into()),arguments: vec![Argument {name: r####"username"####.into(),description: Some(r####"The new username git should use"####.into()),default_value: None,}].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/22844806/how-to-change-my-git-username-in-terminal"####.into()),author: Some(r####"Orils"####.into()),author_url: Some(r####"https://stackoverflow.com/users/2817112/oriol"####.into()),shells: vec![].into_iter().collect(),}
}

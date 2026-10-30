use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Delete newly git-ignored files from your repository"####.into(),command: r####"git rm -r --cached .
git add ."####.into(),tags: vec![r####"git"####.into()].into_iter().collect(),description: Some(r####"After adding new files to the .gitignore file, these commands will update the git index"####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/1274057/how-can-i-make-git-forget-about-a-file-that-was-tracked-but-is-now-in-gitign"####.into()),author: Some(r####"Matt Frear"####.into()),author_url: Some(r####"https://stackoverflow.com/users/32598/matt-frear"####.into()),shells: vec![].into_iter().collect(),}
}

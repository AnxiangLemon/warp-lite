use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Rebase master into feature branch"####.into(),
        command: r####"git checkout
git checkout master
git pull origin master
git checkout -
git pull origin master --rebase"####
            .into(),
        tags: vec![r####"git"####.into()].into_iter().collect(),
        description: Some(r####"Rebases master into the feature branch"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####""####.into()),
        author: Some(r####"Varun Jindal"####.into()),
        author_url: Some(r####"https://www.linkedin.com/in/varun-jindal/"####.into()),
        shells: vec![].into_iter().collect(),
    }
}

use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Yarn install with reproducible dependencies"####.into(),command: r####"yarn install --frozen-lockfile"####.into(),tags: vec![r####"yarn"####.into()].into_iter().collect(),description: Some(r####"Install yarn dependencies withuot a lockfile. This is useful if you need reproducible dependencies, which is usually the case with continuous integration systems."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/58482655/what-is-the-closest-to-npm-ci-in-yarn"####.into()),author: Some(r####"fab67"####.into()),author_url: Some(r####"https://stackoverflow.com/users/11243310/fab67"####.into()),shells: vec![].into_iter().collect(),}
}

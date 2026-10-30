use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Run Build Runner Watch"####.into(),command: r####"flutter pub run build_runner watch --delete-conflicting-outputs"####.into(),tags: vec![r####"flutter"####.into()].into_iter().collect(),description: Some(r####"Run build runner to generate code-gen for flutter project when files changes."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://pub.dev/packages/build_runner"####.into()),author: Some(r####"Birju Vachhani"####.into()),author_url: Some(r####"https://github.com/BirjuVachhani"####.into()),shells: vec![].into_iter().collect(),}
}

use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Clean Project"####.into(),command: r####"flutter clean && flutter pub get"####.into(),tags: vec![r####"flutter"####.into()].into_iter().collect(),description: Some(r####"Cleans flutter project and gets dependencies to make project ready for a fresh run."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://docs.flutter.dev/reference/flutter-cli"####.into()),author: Some(r####"Birju Vachhani"####.into()),author_url: Some(r####"https://github.com/BirjuVachhani"####.into()),shells: vec![].into_iter().collect(),}
}

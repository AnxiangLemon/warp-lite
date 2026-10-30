use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Migrate Create-React-App project into a Nx Workspace"####.into(),command: r####"npx cra-to-nx"####.into(),tags: vec![r####"nx"####.into()].into_iter().collect(),description: Some(r####"If you have an existing Create-React-App project, you can gain the benefits of Nx's computation cache and distributed task execution by running this command"####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://nx.dev/migration/migration-cra"####.into()),author: Some(r####"mandarini"####.into()),author_url: Some(r####"https://github.com/mandarini"####.into()),shells: vec![].into_iter().collect(),}
}

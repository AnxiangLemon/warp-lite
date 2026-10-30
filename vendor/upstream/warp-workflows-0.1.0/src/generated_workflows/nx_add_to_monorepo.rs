use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Add Nx to an Existing Project"####.into(),command: r####"npx add-nx-to-monorepo"####.into(),tags: vec![r####"nx"####.into()].into_iter().collect(),description: Some(r####"If you have an existing Lerna or Yarn monorepo, you can gain the benefits of Nx's computation cache and distributed task execution without modifying the file structure by running this command"####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://nx.dev/getting-started/nx-setup#add-nx-to-an-existing-project"####.into()),author: Some(r####"mandarini"####.into()),author_url: Some(r####"https://github.com/mandarini"####.into()),shells: vec![].into_iter().collect(),}
}

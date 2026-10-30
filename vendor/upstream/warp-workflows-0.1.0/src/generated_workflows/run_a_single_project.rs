use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Run a single Playwright project"####.into(),
        command: r####"npx playwright test --project={{projectName}}"####.into(),
        tags: vec![r####"playwright"####.into()].into_iter().collect(),
        description: Some(r####"Run Playwright tests in a project."####.into()),
        arguments: vec![Argument {
            name: r####"projectName"####.into(),
            description: Some(r####"The project name."####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://playwright.dev/docs/test-advanced#projects"####.into()),
        author: Some(r####"DominusKelvin"####.into()),
        author_url: None,
        shells: vec![].into_iter().collect(),
    }
}

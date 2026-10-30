use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Run Playwright test with the title"####.into(),
        command: r####"npx playwright test --g "{{testTitle}}""####.into(),
        tags: vec![r####"playwright"####.into()].into_iter().collect(),
        description: Some(r####"Runs Playwright test with the title."####.into()),
        arguments: vec![Argument {
            name: r####"testTitle"####.into(),
            description: Some(r####"The title of the test to run."####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://playwright.dev/docs/test-cli#reference"####.into()),
        author: Some(r####"DominusKelvin"####.into()),
        author_url: None,
        shells: vec![].into_iter().collect(),
    }
}

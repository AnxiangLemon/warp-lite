use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Run Playwright tests in headed mode"####.into(),
        command: r####"npx playwright test --headed"####.into(),
        tags: vec![r####"playwright"####.into()].into_iter().collect(),
        description: Some(r####"Runs Playwright tests in headed mode."####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://playwright.dev/docs/test-cli#reference"####.into()),
        author: Some(r####"DominusKelvin"####.into()),
        author_url: None,
        shells: vec![].into_iter().collect(),
    }
}

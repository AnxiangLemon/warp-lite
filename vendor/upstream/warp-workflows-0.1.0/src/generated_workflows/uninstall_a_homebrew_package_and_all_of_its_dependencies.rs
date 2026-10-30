use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Uninstall a Homebrew package and all of its dependencies"####.into(),command: r####"brew tap beeftornado/rmtree
brew rmtree {{package_name}}"####.into(),tags: vec![r####"homebrew"####.into()].into_iter().collect(),description: Some(r####"Uses the external command rmtree to remove a Homebrew package and all of its dependencies"####.into()),arguments: vec![Argument {name: r####"package_name"####.into(),description: Some(r####"The name of the package that should be removed"####.into()),default_value: None,}].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/7323261/uninstall-remove-a-homebrew-package-including-all-its-dependencies"####.into()),author: Some(r####"Ory Band"####.into()),author_url: Some(r####"https://stackoverflow.com/users/207894"####.into()),shells: vec![].into_iter().collect(),}
}

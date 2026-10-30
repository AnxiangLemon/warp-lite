use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"List all Kubernetes pods by app label"####.into(),command: r####"kubectl get pods -l app={{app}} -o jsonpath='{range .items[*]}{"\n"}{range .spec.containers[*]}{.name}{"\t"}{.image}{"\n"}{end}{end}' | sort | uniq
"####.into(),tags: vec![r####"kubernetes"####.into()].into_iter().collect(),description: Some(r####"Lists the images of pods fetched by app label"####.into()),arguments: vec![Argument {name: r####"app"####.into(),description: Some(r####"The value of the app label"####.into()),default_value: None,}].into_iter().collect(),source_url: Some(r####"https://kubernetes.io/docs/reference/kubectl/jsonpath/"####.into()),author: Some(r####"pkaramol"####.into()),author_url: Some(r####"https://www.linkedin.com/in/pkaramol/"####.into()),shells: vec![].into_iter().collect(),}
}

use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Shell for-loop"####.into(),command: r####"for {{variable}} in {{sequence}}; do
      {{command}}
done"####.into(),tags: vec![r####"shell"####.into()].into_iter().collect(),description: Some(r####"A for loop that iterates through a sequence. A common example of this is to iterate through words in in a string, for example:

for i in $( ls ); do
     echo item: $i
done"####.into()),arguments: vec![Argument {name: r####"variable"####.into(),description: Some(r####"The variable to use in the body of the for loop."####.into()),default_value: None,},Argument {name: r####"sequence"####.into(),description: Some(r####"The sequence to iterate over. This is commonly a string."####.into()),default_value: None,},Argument {name: r####"command"####.into(),description: Some(r####"The body of the for loop."####.into()),default_value: None,}].into_iter().collect(),source_url: Some(r####"https://tldp.org/HOWTO/Bash-Prog-Intro-HOWTO-7.html"####.into()),author: Some(r####"Mike G"####.into()),author_url: None,shells: vec![Shell::Zsh,Shell::Bash].into_iter().collect(),}
}

use pest::Parser;

#[derive(Parser)]
#[grammar = "peanut-script.pest"]
struct PeanutScriptParser;

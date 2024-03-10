fn main() {
    rustemo_compiler::Settings::new()
        .parser_algo(rustemo_compiler::ParserAlgo::GLR)
        .actions_in_source_tree()
        .partial_parse(false)
        .process_dir()
        .unwrap();
}

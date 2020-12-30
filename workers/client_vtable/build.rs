use spatialos_codegen::ASTBuilder;

fn main() {
    let ast = ASTBuilder::default()
        .with_directory("../../schema")
        .with_directory("../../build/dependencies/schema/standard_library")
        .build();
    ast.generate("./src/generated", "generated")
        .expect("Could not generate AST");
}

use spatialos_codegen::ASTBuilder;

fn main() {
    let ast = ASTBuilder::default().with_directory("../../schema").build();
    ast.generate("./src/generated")
        .expect("Could not generate AST");
}

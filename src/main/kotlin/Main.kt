import b2.B2InterpreterVisitor
import no.nilsmf.antlr.Basic2Lexer
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.CharStreams
import org.antlr.v4.kotlinruntime.CommonTokenStream

fun main() {
    val input = """
        LET foo: FLOAT = 10f;
        foo = foo * 10;
        PRINT("Hello, World!");
        DECL main() : INT;
        IMPL main() PRINT("HEYO!");
        LET _ = main();
        """.trimIndent()
    val lexer = Basic2Lexer(CharStreams.fromString(input))
    val tokens = CommonTokenStream(lexer)
    val parser = Basic2Parser(tokens)

    val tree = parser.program()

    val visitor = B2InterpreterVisitor()

    visitor.visit(tree)

    visitor.printSymbolTable()
}

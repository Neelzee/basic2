import b2.B2InterpreterVisitor
import no.nilsmf.antlr.Basic2Lexer
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.CharStreams
import org.antlr.v4.kotlinruntime.CommonTokenStream

fun main() {
    val input = """
        LET foo = 10f;
        foo = foo * 10;
        PRINT("Hello, World!");
        """.trimIndent()
    val lexer = Basic2Lexer(CharStreams.fromString(input))
    val tokens = CommonTokenStream(lexer)
    val parser = Basic2Parser(tokens)

    val tree = parser.program()

    B2InterpreterVisitor().visit(tree)
}

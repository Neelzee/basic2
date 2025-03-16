import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream

fun main() {
    val input = "3 + 4 * 2;"
    val lexer = Basic2Lexer(CharStreams.fromString(input))
    val tokens = CommonTokenStream(lexer)
    val parser = Basic2Parser(tokens)

    val tree = parser.program() // Start parsing

    println("Parsed tree: ${tree.toStringTree(parser)}")
}

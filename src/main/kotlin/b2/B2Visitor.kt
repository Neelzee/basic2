package b2

import no.nilsmf.antlr.Basic2Lexer
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.CharStreams
import org.antlr.v4.kotlinruntime.CommonTokenStream
import java.nio.file.Path

class B2Visitor(
    val input: String? = null,
    val path: Path? = null,
) {
    private val lexer = Basic2Lexer(
        input?.let { CharStreams.fromString(it) }
            ?: path?.let { CharStreams.fromPath(path) }
            ?: throw RuntimeException("Must specify a path or input string")
    )
    private val tokens = CommonTokenStream(lexer)
    private val parser = Basic2Parser(tokens)

    private val tree = parser.program()

    private val typeChecker = B2TypeChecker()

    private val visitor = B2Use()

    fun `type-check`() {
        typeChecker.visit(tree)
    }

    fun eval() {
        visitor.visit(tree)
    }
}
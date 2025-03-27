package b2
/*


import b2.interpreter.B2Use
import b2.typechecker.B2TypeChecker
import no.nilsmf.antlr.Basic2Lexer
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.CharStreams
import org.antlr.v4.kotlinruntime.CommonTokenStream
import java.nio.file.Path

class B2Visitor(
    val input: String? = null,
    val path: Path? = null,
) {

    private val program = {
        Basic2Parser(CommonTokenStream(Basic2Lexer(
            input?.let { CharStreams.fromString(it) }
                ?: path?.let { CharStreams.fromPath(path) }
                ?: throw RuntimeException("Must specify a path or input string")
        ))).program()
    }

    private val typeChecker = B2TypeChecker()

    private val visitor = B2Use()

    fun typeCheck() {
        typeChecker.visit(program())
    }

    fun eval() {
        visitor.visit(program())
    }

    fun print() = visitor.printSymbolTable()
    fun printTC() = typeChecker.printSymbolTable()
    fun getSymbolTableEval() = visitor.getSymbolTable()
}
 */
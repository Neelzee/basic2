import b2.interpreter.B2Interpreter
import no.nilsmf.antlr.Basic2Lexer
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.CharStreams
import org.antlr.v4.kotlinruntime.CommonTokenStream
import kotlin.io.path.Path

fun main() {
    val interpreter = B2Interpreter()
    val path = Path("compiler-frontend/src/main/resources/BasicBasedBasic2ClassExample")
    val prg = Basic2Parser(CommonTokenStream(Basic2Lexer(CharStreams.fromPath(path)))).program()
    interpreter.interpret(prg)
}
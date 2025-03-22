import b2.B2Visitor
import no.nilsmf.antlr.Basic2Lexer
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.CharStreams
import org.antlr.v4.kotlinruntime.CommonTokenStream
import java.io.ByteArrayInputStream
import java.nio.file.Paths
import kotlin.test.Test

class B2VisitorTest {
    @Test
    fun `Hello World Test`() {
        val lexer = Basic2Lexer(CharStreams.fromPath(Paths.get("src/main/resources/HelloWorld")))
        val tokens = CommonTokenStream(lexer)
        val parser = Basic2Parser(tokens)

        val tree = parser.program()

        val visitor = B2Visitor()

        visitor.visit(tree)
    }

    @Test
    fun `FizzBuzz Test`() {

        val input = "1\n2\n10\n100\n3123\nQ"

        System.setIn(ByteArrayInputStream(input.toByteArray()))

        val lexer = Basic2Lexer(CharStreams.fromPath(Paths.get("src/main/resources/FizzBuzz")))
        val tokens = CommonTokenStream(lexer)
        val parser = Basic2Parser(tokens)

        val tree = parser.program()

        val visitor = B2Visitor()

        visitor.visit(tree)
    }


    @Test
    fun `Scope Test`() {
        val lexer = Basic2Lexer(CharStreams.fromPath(Paths.get("src/main/resources/Scope")))
        val tokens = CommonTokenStream(lexer)
        val parser = Basic2Parser(tokens)

        val tree = parser.program()

        val visitor = B2Visitor()

        visitor.visit(tree)
        assert(visitor.getSymbolTable().getVar("global").value() as String == "global-inner")
        assert(
            visitor.getSymbolTable().getVar("inner").value() as String
                    == "inner"
        )
    }

    @Test
    fun `Reassign Variable Test`() {
        val lexer = Basic2Lexer(CharStreams.fromPath(Paths.get("src/main/resources/ReassignVariable")))
        val tokens = CommonTokenStream(lexer)
        val parser = Basic2Parser(tokens)

        val tree = parser.program()

        val visitor = B2Visitor()

        visitor.visit(tree)

        assert(visitor.getSymbolTable().getVar("hello").value() as String == "Hello, World!")
    }
}
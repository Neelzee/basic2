import b2.B2Exception
import b2.B2TypeChecker
import no.nilsmf.antlr.Basic2Lexer
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.CharStreams
import org.antlr.v4.kotlinruntime.CommonTokenStream
import org.junit.Test
import java.io.ByteArrayInputStream
import java.nio.file.Paths

class B2TypeCheckerTest {
    @Test
    fun `Hello World Test`() {
        val lexer = Basic2Lexer(CharStreams.fromPath(Paths.get("src/main/resources/HelloWorld")))
        val tokens = CommonTokenStream(lexer)
        val parser = Basic2Parser(tokens)

        val tree = parser.program()

        val visitor = B2TypeChecker()

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

        val visitor = B2TypeChecker()

        visitor.visit(tree)
    }


    @Test
    fun `Scope Test`() {
        val lexer = Basic2Lexer(CharStreams.fromPath(Paths.get("src/main/resources/Scope")))
        val tokens = CommonTokenStream(lexer)
        val parser = Basic2Parser(tokens)

        val tree = parser.program()

        val visitor = B2TypeChecker()

        visitor.visit(tree)
    }

    @Test
    fun `Reassign Variable Test`() {
        val lexer = Basic2Lexer(CharStreams.fromPath(Paths.get("src/main/resources/ReassignVariable")))
        val tokens = CommonTokenStream(lexer)
        val parser = Basic2Parser(tokens)

        val tree = parser.program()

        val visitor = B2TypeChecker()

        visitor.visit(tree)
    }

    @Test
    fun `Invalid Assignment Test`() {
        val lexer = Basic2Lexer(CharStreams.fromString(
            """
            BEGIN PROC TEST   
            LET foo: INT;
            foo = "invalid";
            PROC TEST END
            """.trimIndent()
        ))
        val tokens = CommonTokenStream(lexer)
        val parser = Basic2Parser(tokens)

        val tree = parser.program()

        val visitor = B2TypeChecker()

        try {
            visitor.visit(tree)
        } catch (e: B2Exception.TypeException.ReassignmentException) {
            assert(e.id == "foo")
            assert(e.newType != e.oldType)
        }
    }

    @Test
    fun `Invalid Declaration Test`() {
        val lexer = Basic2Lexer(CharStreams.fromString(
            """
            BEGIN PROC TEST   
            LET foo: INT = "invalid";
            PROC TEST END
            """.trimIndent()
        ))
        val tokens = CommonTokenStream(lexer)
        val parser = Basic2Parser(tokens)

        val tree = parser.program()

        val visitor = B2TypeChecker()

        try {
            visitor.visit(tree)
        } catch (e: B2Exception.TypeException.InvalidCastException) {
            assert(e.id == "foo")
            assert(e.valueType != e.type)
        }
    }


    @Test
    fun `Invalid Array Declaration Test`() {
        val lexer = Basic2Lexer(CharStreams.fromString(
            """
            BEGIN PROC TEST   
            LET foo: [INT] = ["Invalid"];
            PROC TEST END
            """.trimIndent()
        ))
        val tokens = CommonTokenStream(lexer)
        val parser = Basic2Parser(tokens)

        val tree = parser.program()

        val visitor = B2TypeChecker()

        try {
            visitor.visit(tree)
        } catch (e: B2Exception.TypeException.InvalidCastException) {
            assert(e.id == "foo")
            assert(e.valueType != e.type)
        }
    }

    @Test
    fun `Invalid Array Reassignment Test`() {
        val lexer = Basic2Lexer(CharStreams.fromString(
            """
            BEGIN PROC TEST   
            LET foo: [INT] = [0, 1, 2];
            foo[0] = "invalid";
            PROC TEST END
            """.trimIndent()
        ))
        val tokens = CommonTokenStream(lexer)
        val parser = Basic2Parser(tokens)

        val tree = parser.program()

        val visitor = B2TypeChecker()

        try {
            visitor.visit(tree)
        } catch (e: B2Exception.TypeException.InvalidIndexingElementException) {
            assert(e.id == "foo")
            assert(e.element != e.type)
        }
    }
}
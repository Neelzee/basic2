import b2.B2Exception
import b2.B2TypeChecker
import b2.B2Visitor
import b2.symbols.Symbol
import no.nilsmf.antlr.Basic2Lexer
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.CharStreams
import org.antlr.v4.kotlinruntime.CommonTokenStream
import java.io.ByteArrayInputStream
import java.nio.file.Paths
import kotlin.test.Test
import kotlin.test.assertEquals

class B2VisitorTest {
    @Test
    fun `Hello World Test`() {
        val visitor = B2Visitor(path = Paths.get("src/main/resources/HelloWorld"))
        visitor.eval()
    }

    @Test
    fun `FizzBuzz Test`() {

        val input = "1\n2\n10\n100\n3123\nQ"

        System.setIn(ByteArrayInputStream(input.toByteArray()))
        B2Visitor(path = Paths.get("src/main/resources/FizzBuzz")).eval()
    }


    @Test
    fun `Scope Test`() {
        val visitor = B2Visitor(path = Paths.get("src/main/resources/Scope"))

        visitor.eval()

        assert(visitor.getSymbolTableEval().getVar("global").value() as String == "global-inner")
        assert(
            visitor.getSymbolTableEval().getVar("inner").value() as String
                    == "inner"
        )
    }

    @Test
    fun `Reassign Variable Test`() {
        val visitor = B2Visitor(path = Paths.get("src/main/resources/ReassignVariable"))

        visitor.eval()

        assert(visitor.getSymbolTableEval().getVar("hello").value() as String == "Hello, World!")
    }


    @Test
    fun `Iterable Test`() {
        val visitor = B2Visitor(
            """
            BEGIN PROC iterable
            LET intIterable = (0, 10);
            LET outputInt = [];
            FOR (i IN FROM intIterable[0] TO intIterable[1]) THEN ADD(i, outputInt); END
            
            LET arrIterable = ["foo", "bar", "foobar"];
            LET outputArr = [];
            FOR (j IN arrIterable) THEN ADD(j, outputArr); END
            
            LET strIterable = "ABC";
            LET outputStr = [];
            FOR (k IN strIterable) THEN ADD(k, outputStr); END
            PROC iterable END
            """.trimIndent()
        )

        visitor.eval()

        assertEquals(
            visitor.getSymbolTableEval().getVar("outputInt").value() as List<*>,
            (0..10).map { Symbol.Var.Value.VInt(it) }.toList()
        )

        assertEquals(
            visitor.getSymbolTableEval().getVar("outputArr").value() as List<*>,
            listOf("foo", "bar", "foobar").map { Symbol.Var.Value.VString(it) }
        )

        assertEquals(
            visitor.getSymbolTableEval().getVar("outputStr").value() as List<*>,
            listOf("A", "B", "C").map { Symbol.Var.Value.VString(it) }
        )
    }
}
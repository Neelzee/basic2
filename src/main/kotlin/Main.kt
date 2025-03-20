import b2.B2Visitor
import no.nilsmf.antlr.Basic2Lexer
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.CharStreams
import org.antlr.v4.kotlinruntime.CommonTokenStream
import java.nio.file.Paths

val while_test = """
    LET count = 0;
    WHILE (count != 10) THEN
    {
    count = count + 1;
    PRINT(count);
    }
    END
""".trimIndent()

val for_loop_test = """
    FOR (i = 0; i <= 10; i += 1;) THEN
    PRINT(i);
    END
""".trimIndent()

val random_bullshit = """
        LET foo: FLOAT = 10f;
        foo = foo * 10;
        PRINT("Hello, World!");
        DECL main() : INT;
        IMPL main() PRINT("HEYO!");
        LET _ = main();
        """.trimIndent()

val function_args = """
DECL fib(INT): INT;
IMPL fib(n) IF (2 >= n) THEN
RETURN 1;
ELSE
RETURN fib(n - 2) + fib(n - 1);
FI
LET result = fib(10);
PRINT(result);
""".trimIndent()

val if_test = """
    LET foo = 2;
    IF (2 % 2 == 0) THEN
      PRINT("even");
    FI
    IF (2 % 2 == 1) THEN
      PRINT("odd");
    FI
""".trimIndent()

val factory_test = """
DECL fac(INT): INT;
IMPL fac(n) IF (1 >= n) THEN
RETURN 1;
ELSE
RETURN n * fac(n - 1);
FI
LET result = fac(30);
PRINT(result);
""".trimIndent()

val input_test = """
LET foo: STR = INPUT("> ");
PRINT(foo);
""".trimIndent()

fun main() {
    val lexer = Basic2Lexer(CharStreams.fromPath(Paths.get("src/main/resources/IsPrime")))
    val tokens = CommonTokenStream(lexer)
    val parser = Basic2Parser(tokens)

    val tree = parser.program()

    val visitor = B2Visitor()

    visitor.visit(tree)

    visitor.printSymbolTable()
}

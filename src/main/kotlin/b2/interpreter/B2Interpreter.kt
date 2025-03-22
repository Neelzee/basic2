package b2.interpreter

import arrow.core.None
import arrow.core.Option
import arrow.core.Some
import b2.B2Visitor
import no.nilsmf.antlr.Basic2Lexer
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.CharStreams
import org.antlr.v4.kotlinruntime.CommonTokenStream

sealed class Cmd {
    data object Help : Cmd()
    data object Clear : Cmd()
    data class Quit(val save: Boolean = false) : Cmd()

    companion object {
        fun from(s: String): Option<Cmd> = when (s) {
            ":h" -> Some(Help)
            ":q" -> Some(Quit())
            ":c" -> Some(Clear)
            ":wq" -> Some(Quit(true))
            ":x" -> Some(Quit(true))
            else -> None
        }
    }
}

class B2Interpreter {
    private val interpreter = B2Visitor()

    fun help(): String = """
===============================================================================
|                                                                             |
|   Help Menu                                                                 |
|  X===========X==============X============================================X  |
|  x Command   x Name         x Description                                x  |
|  X===========X==============X============================================X  |
|  x :c        x Clear        x Clears the terminal                        x  |
|  X===========X==============X============================================X  |
|  x :q        x Quit         x Exits the program                          x  |
|  X===========X==============X============================================X  |
|  x :wq/:x    x Save & Quit  x Saves the program and then exits           x  |
|  X==========================X============================================X  |
|  x :h        x Help         x Prints this menu                           x  |
|  X==========================X============================================X  |
|                                                                             |
===============================================================================
    """.trimIndent()

    fun interpret(): Result<Unit> {
        var cont = true;
        while (cont) {
            val inp = readln()
            cont = when {
                inp.startsWith(":") -> parse(inp)
                else -> {
                    //println(interpreter.visit(Basic2Parser(CommonTokenStream(Basic2Lexer(CharStreams.fromString(inp)))).program()))
                    true
                }
            }
        }

        return Result.success(Unit)
    }

    private fun parse(s: String): Boolean = when (val optCmd = Cmd.from(s)) {
        is None -> TODO()
        is Some<*> -> when (val cmd = optCmd.value as Cmd) {
            is Cmd.Help -> {
                println(help())
                true
            }
            is Cmd.Quit if (cmd.save) -> TODO("Implement saving")
            is Cmd.Quit -> false
            is Cmd.Clear -> {
                clearTerm()
                true
            }
        }
    }

    private fun clearTerm() = print("\u001b[2J")
}
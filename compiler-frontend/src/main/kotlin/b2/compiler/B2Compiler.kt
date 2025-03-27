package b2.compiler

import no.nilsmf.antlr.Basic2BaseVisitor
import no.nilsmf.antlr.Basic2Parser

data class Program(
    val name: String,
)

sealed class Statement {

}

class B2Compiler : Basic2BaseVisitor<Any?>() {
    override fun defaultResult(): Any? {
        TODO("Not yet implemented")
    }

}
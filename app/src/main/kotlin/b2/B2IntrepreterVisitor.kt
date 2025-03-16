package b2

import Basic2BaseVisitor
import Basic2Visitor
import org.antlr.v4.runtime.tree.ErrorNode
import org.antlr.v4.runtime.tree.ParseTree
import org.antlr.v4.runtime.tree.RuleNode
import org.antlr.v4.runtime.tree.TerminalNode

class B2IntrepreterVisitor<T> : Basic2BaseVisitor<T>() {

    companion object {
        val tbl = SymbolTable()
    }

    override fun visitFloat(ctx: Basic2Parser.FloatContext?): T {
        return super.visitFloat(ctx)
    }
}
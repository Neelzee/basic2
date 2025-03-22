package b2

import b2.symbols.Symbol
import no.nilsmf.antlr.Basic2Lexer
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.CharStreams
import org.antlr.v4.kotlinruntime.CommonTokenStream
import java.nio.file.Path
import java.nio.file.Paths

open class B2Use : B2Stmt() {
    override fun stmtCtx(ctx: Basic2Parser.StmtContext): Symbol.Var.Value = when (ctx) {
        else -> TODO(ctx.text)
    }

    fun importModule(filePath: Path): Symbol.Var.Value {

        val lexer = Basic2Lexer(CharStreams.fromPath(filePath))
        val tokens = CommonTokenStream(lexer)
        val parser = Basic2Parser(tokens)

        val tree = parser.program()

        return when (val res = visit(tree)) {
            is Symbol.Var.Value -> res
            else -> Symbol.Var.Value.VUnit
        }
    }

    override fun visitUseAll(ctx: Basic2Parser.UseAllContext): Symbol.Var.Value.VUnit {
        val module = ctx.IDENTIFIER().text
        if (!getSymbolTable().hasModule(module)) {
            getSymbolTable().declModule(module)
            val filePath = Paths.get("src/main/resources/$module")
            if (!filePath.toFile().exists()) throw B2Exception.MissingModuleException(module, ctx.position)
            importModule(filePath)
        }
        return Symbol.Var.Value.VUnit
    }

    override fun visitUseSpecific(ctx: Basic2Parser.UseSpecificContext): Symbol {
        return super.visitUseSpecific(ctx)
    }

    override fun visitRenaming(ctx: Basic2Parser.RenamingContext): Symbol {
        return super.visitRenaming(ctx)
    }

    override fun visitImport_items(ctx: Basic2Parser.Import_itemsContext): Symbol {
        return super.visitImport_items(ctx)
    }
}
package b2

import b2.symbols.Symbol
import no.nilsmf.antlr.Basic2Lexer
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.CharStreams
import org.antlr.v4.kotlinruntime.CommonTokenStream
import org.antlr.v4.kotlinruntime.ast.Position
import java.nio.file.Paths

open class B2Use : B2Stmt() {
    fun importModule(
        module: String,
        pos: Position?,
        imports: List<Symbol.Var.ImportItem>? = null,
        useAll: Boolean = false,
    ): Symbol.Var.Value {
        val filePath = Paths.get("src/main/resources/$module")
        if (!filePath.toFile().exists()) throw B2Exception.MissingModuleException(module, pos)

        val moduleProgram = {
            Basic2Parser(CommonTokenStream(Basic2Lexer(CharStreams.fromPath(filePath)))).program()
        }

        val tc = B2TypeChecker()

        try {
            tc.visit(moduleProgram())
        } catch (e: B2Exception.TypeException) {
            throw B2Exception.TypeException.ImportException(module, e, pos)
        }

        val visitor = B2Use()

        visitor.visit(moduleProgram())
        val moduleSymbolTable = visitor.getSymbolTable()

        getSymbolTable().declModule(module)

        if (useAll) {
            val current = getSymbolTable()
            val rename = imports?.first()?.newName ?: ""
            val newId = { id: String -> if (rename.isEmpty()) { id } else { "${rename}_$id" } }
            val variables = moduleSymbolTable.iterVariable().toList()
            val fnDecls = moduleSymbolTable.iterFnDecl().toList()
            val fnImpls = moduleSymbolTable.iterFnImpl().toList()

            variables.forEach { current.declAssVar(newId(it.key), it.value as Symbol.Var.Value) }
            fnDecls.forEach { current.addFnDecl(newId(it.key), it.value.params.map { t -> t.type }, it.value.resultType) }
            fnImpls.forEach { current.addFnImpl(newId(it.key), it.value.args, it.value.body) }

            return Symbol.Var.Value.VUnit
        }

        imports?.forEachIndexed { i, it ->
            val rename = imports.getOrNull(i)?.newName ?: ""
            val newId = { id: String -> if (rename.isEmpty()) { id } else { rename } }
            when (it.type) {
                is Symbol.FnImpl -> {
                    val (args, body) = moduleSymbolTable.getImpl(it.id)
                    getSymbolTable().addFnImpl(newId(it.id), args, body)
                }
                is Symbol.FnDecl -> {
                    val (_, params, result) = moduleSymbolTable.getDecl(it.id)
                    getSymbolTable().addFnDecl(newId(it.id), params.map { t -> t.type }, result)
                }
                is Symbol.Var.Value.VUnit -> {
                    val (args, body) = moduleSymbolTable.getImpl(it.id)
                    getSymbolTable().addFnImpl(newId(it.id), args, body)
                    val (_, params, result) = moduleSymbolTable.getDecl(it.id)
                    getSymbolTable().addFnDecl(newId(it.id), params.map { t -> t.type }, result)
                }
                is Symbol.Var.Variable -> {
                    val variable = moduleSymbolTable.getVar(it.id)
                    getSymbolTable().declAssVar(newId(it.id), variable)
                }
                else -> throw RuntimeException("Cannot import $it")
            }
        }

        return Symbol.Var.Value.VUnit
    }

    override fun visitUseAll(ctx: Basic2Parser.UseAllContext): Symbol.Var.Value.VUnit {
        val module = ctx.IDENTIFIER().text
        val renaming = ctx.renaming()?.let {
            listOf(Symbol.Var.ImportItem("*", Symbol.Var.Value.VUnit, visitRenaming(it).value))
        }
        if (!getSymbolTable().hasModule(module)) {
            importModule(module, ctx.position, renaming, true)
        }
        return Symbol.Var.Value.VUnit
    }

    override fun visitUseSpecific(ctx: Basic2Parser.UseSpecificContext): Symbol.Var.Value.VUnit {
        val module = ctx.IDENTIFIER().text
        val renaming: List<String> = ctx.renaming().map { visitRenaming(it).value }
        val imports: List<Symbol.Var.ImportItem> = ctx.import_items().map {
            Symbol.Var.ImportItem("", TODO())
        }
        if (!getSymbolTable().hasModule(module)) {
            importModule(module, ctx.position, imports)
        }
        return Symbol.Var.Value.VUnit
    }

    override fun visitRenaming(ctx: Basic2Parser.RenamingContext) = Symbol.Var.Value.VString(ctx.IDENTIFIER().text)

    override fun visitFnDecl(ctx: Basic2Parser.FnDeclContext): Symbol.Var.ImportItem {
        val id = ctx.IDENTIFIER().text
        return Symbol.Var.ImportItem(id, Symbol.FnDecl())
    }

    override fun visitFnImpl(ctx: Basic2Parser.FnImplContext): Symbol.Var.ImportItem {
        return Symbol.Var.ImportItem(ctx.IDENTIFIER().text, Symbol.FnImpl())
    }

    override fun visitFnDeclImpl(ctx: Basic2Parser.FnDeclImplContext): Symbol.Var.ImportItem
        = Symbol.Var.ImportItem(ctx.IDENTIFIER().text, Symbol.Var.Value.VUnit)

    override fun visitVar(ctx: Basic2Parser.VarContext): Symbol.Var.ImportItem
        = Symbol.Var.ImportItem(ctx.IDENTIFIER().text, Symbol.Var.Variable())
}
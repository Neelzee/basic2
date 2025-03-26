package b2.visitor

import b2.symbols.B2Ast
import b2.symbols.B2Ast.Statement
import b2.symbols.B2Ast.Type
import no.nilsmf.antlr.Basic2BaseVisitor
import no.nilsmf.antlr.Basic2Parser
import kotlin.streams.toList

open class B2 : Basic2BaseVisitor<B2Ast>() {

    override fun defaultResult() = Statement.NoOp

    fun stmtCtx(ctx: Basic2Parser.StmtContext): Statement = when (ctx) {
        else -> TODO()
    }

    fun exprCtx(ctx: Basic2Parser.ExprContext): B2Ast.Expression = when (ctx) {
        else -> TODO()
    }

    override fun visitProgram(ctx: Basic2Parser.ProgramContext): B2Ast {
        return B2Ast.Program(
            ctx.IDENTIFIER().first().text,
            ctx.stmt().map { stmtCtx(it) }
        )
    }

    override fun visitIfElse(ctx: Basic2Parser.IfElseContext) = visitIfElseStmt(ctx.ifElseStmt())

    override fun visitIfElseStmt(ctx: Basic2Parser.IfElseStmtContext) = Statement.If(
        condition = exprCtx(ctx.expr()),
        thenBranch = listOf(stmtCtx(ctx.stmt(0)!!)),
        elseBranch = listOf(stmtCtx(ctx.stmt(0)!!))
    )

    override fun visitElifBlock(ctx: Basic2Parser.ElifBlockContext) = visitIfElifStmtBlock(ctx.ifElifStmtBlock())

    override fun visitIfElifStmtBlock(ctx: Basic2Parser.IfElifStmtBlockContext) = Statement.If(
        condition = exprCtx(ctx.expr()),
        thenBranch = ctx.thenBlock.map { stmtCtx(it) },
        elif = ctx.elifStmtBlock().map { Pair(exprCtx(it.expr()), it.stmt().map { s -> stmtCtx(s) }) },
        elseBranch = ctx.elseBlock.map { stmtCtx(it) }
    )

    override fun visitElif(ctx: Basic2Parser.ElifContext) = visitIfElifStmt(ctx.ifElifStmt())


    override fun visitIfElifStmt(ctx: Basic2Parser.IfElifStmtContext)= Statement.If(
        condition = exprCtx(ctx.expr(0)!!),
        thenBranch = listOf(stmtCtx(ctx.stmt(0)!!)),
        elif = ctx.stmt().subList(1, ctx.stmt().size - 1).zip(ctx.expr().subList(1, ctx.stmt().size))
            .map { (stmt, expr) -> Pair(exprCtx(expr), listOf(stmtCtx(stmt))) },
        elseBranch = listOf(stmtCtx(ctx.stmt(ctx.stmt().size - 1)!!))
    )

    override fun visitIfBlock(ctx: Basic2Parser.IfBlockContext) = visitIfStmtBlock(ctx.ifStmtBlock())

    override fun visitIfElseStmtBlock(ctx: Basic2Parser.IfElseStmtBlockContext) = Statement.If(
        condition = exprCtx(ctx.expr()),
        thenBranch = ctx.ifThenBlock().stmt().map { stmtCtx(it) },
        elseBranch = ctx.ifElseBlock().stmt().map { stmtCtx(it) }
    )

    override fun visitIfThenBlock(ctx: Basic2Parser.IfThenBlockContext): B2Ast {
        return super.visitIfThenBlock(ctx)
    }

    override fun visitIfElseBlock(ctx: Basic2Parser.IfElseBlockContext): B2Ast {
        return super.visitIfElseBlock(ctx)
    }

    override fun visitIfStmtBlock(ctx: Basic2Parser.IfStmtBlockContext): B2Ast {
        return super.visitIfStmtBlock(ctx)
    }

    override fun visitIf(ctx: Basic2Parser.IfContext) = visitIfStmt(ctx.ifStmt())

    override fun visitIfStmt(ctx: Basic2Parser.IfStmtContext) = Statement.If(
        condition = exprCtx(ctx.expr()),
        thenBranch = listOf(stmtCtx(ctx.stmt()))
    )

    // LOOP STATEMENTS =========================================================

    override fun visitWhile(ctx: Basic2Parser.WhileContext) = visitWhileStmt(ctx.whileStmt())

    override fun visitWhileStmt(ctx: Basic2Parser.WhileStmtContext) = Statement.While(
        condition = exprCtx(ctx.expr()),
        body = listOf(stmtCtx(ctx.stmt()))
    )

    override fun visitWhileBlock(ctx: Basic2Parser.WhileBlockContext) = visitWhileStmtBlock(ctx.whileStmtBlock())

    override fun visitWhileStmtBlock(ctx: Basic2Parser.WhileStmtBlockContext)= Statement.While(
        condition = exprCtx(ctx.expr()),
        body = ctx.blockStmt().stmt().map { stmtCtx(it) }
    )

    override fun visitFor(ctx: Basic2Parser.ForContext) = visitForStmt(ctx.forStmt())

    override fun visitForStmt(ctx: Basic2Parser.ForStmtContext) = Statement.For(
        i = B2Ast.Expression.Var(ctx.IDENTIFIER(0)?.text!!),
        init = exprCtx(ctx.expr(0)!!),
        cond = B2Ast.Expression.BinOp(
            left = B2Ast.Expression.Var(ctx.IDENTIFIER(1)?.text!!),
            operator = visitComp(ctx.comp()),
            right = exprCtx(ctx.expr(1)!!)
        ),
        incr = ctx.expr(2)?.let {
            B2Ast.Expression.BinOp(
                left = B2Ast.Expression.Var(ctx.IDENTIFIER(2)?.text!!),
                operator = visitIncr(ctx.incr()!!),
                right = exprCtx(it)
            )
        } ?: B2Ast.Expression.UniOp(
            value = B2Ast.Expression.Var(ctx.IDENTIFIER(2)?.text!!),
            operator = visitIncrUni(ctx.incrUni()!!),
        ),
        body = listOf(stmtCtx(ctx.stmt())),
    )

    override fun visitForR(ctx: Basic2Parser.ForRContext) = visitForRange(ctx.forRange())

    override fun visitForRange(ctx: Basic2Parser.ForRangeContext) = Statement.ForIter(
        ident = B2Ast.Expression.Var(ctx.IDENTIFIER().text),
        iter = visitIterable(ctx.iterable()),
        body = listOf(stmtCtx(ctx.stmt()))
    )

    // BREAK STATEMENTS ========================================================

    override fun visitRet(ctx: Basic2Parser.RetContext) = visitReturnStmt(ctx.returnStmt())

    override fun visitReturnStmt(ctx: Basic2Parser.ReturnStmtContext)
        = Statement.Return(ctx.expr()?.let { exprCtx(it) })

    override fun visitBrk(ctx: Basic2Parser.BrkContext) = visitBreakStmt(ctx.breakStmt())

    override fun visitBreakStmt(ctx: Basic2Parser.BreakStmtContext) = Statement.Break

    override fun visitCont(ctx: Basic2Parser.ContContext) = visitContinueStmt(ctx.continueStmt())

    override fun visitContinueStmt(ctx: Basic2Parser.ContinueStmtContext) = Statement.Continue

    // BUILTINS ================================================================

    override fun visitPrnt(ctx: Basic2Parser.PrntContext) = visitPrintStmt(ctx.printStmt())

    override fun visitPrintStmt(ctx: Basic2Parser.PrintStmtContext) = Statement.Print(exprCtx(ctx.expr()))

    override fun visitInputExpr(ctx: Basic2Parser.InputExprContext) = B2Ast.Expression.Input(
        ctx.expr()?.let { exprCtx(it) }
    )

    override fun visitLenExpr(ctx: Basic2Parser.LenExprContext) = B2Ast.Expression.Len(exprCtx(ctx.expr()))

    override fun visitInput(ctx: Basic2Parser.InputContext) = visitInputExpr(ctx.inputExpr())

    override fun visitArrReAss(ctx: Basic2Parser.ArrReAssContext) = visitArrReStmt(ctx.arrReStmt())

    override fun visitArrReStmt(ctx: Basic2Parser.ArrReStmtContext) = Statement.ArrIndReAssign(
        ident = ctx.IDENTIFIER().text,
        ind = exprCtx(ctx.expr(0)!!),
        newValue = exprCtx(ctx.expr(1)!!),
    )

    override fun visitVarDecl(ctx: Basic2Parser.VarDeclContext) = visitVarDeclStmt(ctx.varDeclStmt())

    override fun visitVarAs(ctx: Basic2Parser.VarAsContext) = visitVarDeclAssignStmt(ctx.varDeclAssignStmt())

    override fun visitVarReAs(ctx: Basic2Parser.VarReAsContext) = visitVarReStmt(ctx.varReStmt())

    override fun visitBlock(ctx: Basic2Parser.BlockContext)
        = Statement.Block(ctx.blockStmt().stmt().map { stmtCtx(it) })

    override fun visitDecl(ctx: Basic2Parser.DeclContext) = visitFnDeclStmt(ctx.fnDeclStmt())

    override fun visitImpl(ctx: Basic2Parser.ImplContext) = visitFnImplStmt(ctx.fnImplStmt())

    override fun visitPreIncr(ctx: Basic2Parser.PreIncrContext) = B2Ast.Expression.UniOp(
        value = exprCtx(ctx.expr()),
        operator = visitIncrUni(ctx.incrUni())
    )

    override fun visitBinopIncr(ctx: Basic2Parser.BinopIncrContext) = B2Ast.Expression.BinOp(
        left = B2Ast.Expression.Var(ctx.IDENTIFIER().text),
        operator = visitIncr(ctx.incr()),
        right = exprCtx(ctx.expr()),
    )

    // IMPORTS =================================================================

    override fun visitUseAll(ctx: Basic2Parser.UseAllContext) = Statement.Import(
        moduleIdent = ctx.IDENTIFIER().text,
        rename = ctx.renaming()?.IDENTIFIER()?.text
    )

    override fun visitUseSpecific(ctx: Basic2Parser.UseSpecificContext) = Statement.ImportSpecific(
        moduleIdent = ctx.IDENTIFIER().text,
        imports = ctx.importItems().map {
            when (it) {
                is Basic2Parser.VarContext -> Pair(visitVar(it), it.renaming()?.let { r -> visitRenaming(r).value })
                is Basic2Parser.FnDeclContext -> Pair(visitFnDecl(it), it.renaming()?.let { r -> visitRenaming(r).value })
                is Basic2Parser.FnImplContext -> Pair(visitFnImpl(it), it.renaming()?.let { r -> visitRenaming(r).value })
                is Basic2Parser.FnDeclImplContext -> Pair(visitFnDeclImpl(it), it.renaming()?.let { r -> visitRenaming(r).value })
                else -> TODO()
            }
        }
    )

    override fun visitRenaming(ctx: Basic2Parser.RenamingContext) = B2Ast.Expression.Str(ctx.IDENTIFIER().text)

    override fun visitFnDecl(ctx: Basic2Parser.FnDeclContext) = Statement.ImportItem.FnDecl(
        ident = ctx.IDENTIFIER().text
    )

    override fun visitFnImpl(ctx: Basic2Parser.FnImplContext) = Statement.ImportItem.FnImpl(
        ident = ctx.IDENTIFIER().text
    )

    override fun visitFnDeclImpl(ctx: Basic2Parser.FnDeclImplContext) = Statement.ImportItem.FnDeclImpl(
        ident = ctx.IDENTIFIER().text,
    )

    override fun visitVar(ctx: Basic2Parser.VarContext)
        = Statement.ImportItem.Var(ctx.IDENTIFIER().text)

    // VARIABLES ===============================================================

    override fun visitVarDeclStmt(ctx: Basic2Parser.VarDeclStmtContext) = Statement.VarDecl(
        ident = ctx.IDENTIFIER().text,
        type = visitTyping(ctx.typing())
    )

    override fun visitVarDeclAssignStmt(ctx: Basic2Parser.VarDeclAssignStmtContext): Statement.VarAssDecl {
        val value = exprCtx(ctx.expr())
        val type = ctx.typing()?.let { visitTyping(it) } ?: value.type()
        return Statement.VarAssDecl(
            ident = ctx.IDENTIFIER().text,
            type,
            value,
        )
    }

    override fun visitVarReStmt(ctx: Basic2Parser.VarReStmtContext) = Statement.VarReAssign(
        ident = ctx.IDENTIFIER().text,
        newValue = exprCtx(ctx.expr())
    )

    override fun visitTyping(ctx: Basic2Parser.TypingContext) = visitType(ctx.type())

    override fun visitBlockStmt(ctx: Basic2Parser.BlockStmtContext) = Statement.Block(ctx.stmt().map { stmtCtx(it) })

    override fun visitFnDeclStmt(ctx: Basic2Parser.FnDeclStmtContext) = Statement.FnDecl(
        ident = ctx.IDENTIFIER().text,
        params = ctx.type().subList(0, ctx.type().size - 1).map { visitType(it) },
        resultType = visitType(ctx.type().last())
    )

    override fun visitFnImplStmt(ctx: Basic2Parser.FnImplStmtContext) = Statement.FnImpl(
        ident = ctx.IDENTIFIER().text,
        args = ctx.fnParam().map { Pair(it.IDENTIFIER().text, it.expr()?.let { e -> exprCtx(e) }) },
    )

    override fun visitComp(ctx: Basic2Parser.CompContext): B2Ast.Operator.Bi = when (val op = ctx.text) {
        "==" -> B2Ast.Operator.Bi.Eq
        else -> TODO(op)
    }

    override fun visitIncr(ctx: Basic2Parser.IncrContext): B2Ast.Operator.Bi = when (val op = ctx.text) {
        "+=" -> B2Ast.Operator.Bi.AddMut
        else -> TODO(op)
    }

    override fun visitIncrUni(ctx: Basic2Parser.IncrUniContext): B2Ast.Operator.Uni = when (val op = ctx.text) {
        "++" -> B2Ast.Operator.Uni.Incr
        else -> TODO(op)
    }

    override fun visitType(ctx: Basic2Parser.TypeContext): Type = when {
        ctx.PRIM_TYPES() != null -> Type.infer(ctx.PRIM_TYPES())
        ctx.ARRAY_STRT() != null -> Type.Lst(visitType(ctx.type(0)!!))
        ctx.TUPLE_STRT() != null -> Type.Tup(visitType(ctx.type(0)!!), visitType(ctx.type(1)!!))
        else -> TODO()
    }

    override fun visitIterable(ctx: Basic2Parser.IterableContext): B2Ast.Expression.Arr
    = when (val expr = exprCtx(ctx.expr(0)!!)) {
        is B2Ast.Expression.Arr -> expr
        is B2Ast.Expression.Str -> {
            val value: Array<B2Ast.Expression>
                = expr.value.chars().toList().map { B2Ast.Expression.Str(it.toChar().toString()) }.toTypedArray()
            B2Ast.Expression.Arr(
                value,
                Type.Str,
                value.size
                )
        }
        else -> {
            val len: Int = exprCtx(ctx.expr(1)!!).value() as Int
            B2Ast.Expression.Arr(
                (expr.value() as Int..len)
                    .map { B2Ast.Expression.Int(it) }.toTypedArray(),
                Type.Int,
                len
            )
        }
    }

    override fun visitBinOp(ctx: Basic2Parser.BinOpContext) = when (val op = ctx.text) {
        "+" -> B2Ast.Operator.Bi.Add
        else -> TODO(op)
    }

    override fun visitIdent(ctx: Basic2Parser.IdentContext) = B2Ast.Expression.Var(ctx.IDENTIFIER().text)

    override fun visitIntLit(ctx: Basic2Parser.IntLitContext) = B2Ast.Expression.Int(ctx.NUM_LIT().text.toInt())

    override fun visitTuple(ctx: Basic2Parser.TupleContext)
        = B2Ast.Expression.Tup(exprCtx(ctx.expr(0)!!), exprCtx(ctx.expr(1)!!))

    override fun visitCast(ctx: Basic2Parser.CastContext)
        = B2Ast.Expression.Cast(exprCtx(ctx.expr()), visitType(ctx.type()))

    override fun visitStrLit(ctx: Basic2Parser.StrLitContext) = B2Ast.Expression.Str(ctx.STR_LIT().text)

    override fun visitFloatLit(ctx: Basic2Parser.FloatLitContext) = B2Ast.Expression.Flt(ctx.FLOAT_LIT().text.toFloat())

    override fun visitTrim(ctx: Basic2Parser.TrimContext)
        = B2Ast.Expression.Trim(exprCtx(ctx.expr()) as B2Ast.Expression.Str)

    override fun visitLen(ctx: Basic2Parser.LenContext) = visitLenExpr(ctx.lenExpr())

    override fun visitBoolLit(ctx: Basic2Parser.BoolLitContext) = B2Ast.Expression.Bol(ctx.BOOL_LIT().text == "TRUE")

    override fun visitArrLit(ctx: Basic2Parser.ArrLitContext): B2Ast.Expression.Arr {
        val size = ctx.NUM_LIT().text.toInt()
        val elementType = visitType(ctx.type())
        return B2Ast.Expression.Arr(
            value = (0..size).map { elementType.default() }.toTypedArray(),
            elementType,
            size
        )
    }

    override fun visitFnCall(ctx: Basic2Parser.FnCallContext) = B2Ast.Expression.FnCall(
        ident = ctx.IDENTIFIER().text,
        args = ctx.expr().map { exprCtx(it) }
    )

    override fun visitArrExpLit(ctx: Basic2Parser.ArrExpLitContext): B2Ast.Expression.Arr {
        val value = ctx.expr().map { exprCtx(it) }
        return B2Ast.Expression.Arr(
            value.toTypedArray(),
            elementType = value.first().type(),
            size = value.size
        )
    }

    override fun visitBinop(ctx: Basic2Parser.BinopContext) = B2Ast.Expression.BinOp(
        left = exprCtx(ctx.expr(0)!!),
        operator = visitBinOp(ctx.binOp()),
        right = exprCtx(ctx.expr(1)!!)
    )

    override fun visitGroup(ctx: Basic2Parser.GroupContext) = B2Ast.Expression.Group(exprCtx(ctx.expr()))

    override fun visitArrInd(ctx: Basic2Parser.ArrIndContext) = B2Ast.Expression.ArrInd(
        arr = exprCtx(ctx.expr(0)!!),
        ind = exprCtx(ctx.expr(1)!!)
    )

    override fun visitBinopComp(ctx: Basic2Parser.BinopCompContext) = B2Ast.Expression.BinOp(
        left = exprCtx(ctx.expr(0)!!),
        operator = visitComp(ctx.comp()),
        right = exprCtx(ctx.expr(1)!!)
    )
}
package b2.visitor

import b2.symbols.B2Ast
import b2.symbols.B2Ast.Statement
import b2.symbols.B2Ast.Type
import b2.symbols.Position
import no.nilsmf.antlr.Basic2BaseVisitor
import no.nilsmf.antlr.Basic2Parser
import kotlin.streams.toList

open class B2 : Basic2BaseVisitor<B2Ast>() {

    override fun defaultResult() = Statement.NoOp()

    fun stmtCtx(ctx: Basic2Parser.StmtContext): Statement = when (ctx) {
        is Basic2Parser.PrntContext -> visitPrintStmt(ctx.printStmt())
        is Basic2Parser.VarDeclContext -> visitVarDeclStmt(ctx.varDeclStmt())
        is Basic2Parser.VarAsContext -> visitVarAs(ctx)
        is Basic2Parser.BlockContext -> visitBlock(ctx)
        is Basic2Parser.TypeAliasContext -> visitTypeAlias(ctx)
        is Basic2Parser.BinopIncrContext -> Statement.Expr(visitBinopIncr(ctx), Position.from(ctx.position))
        is Basic2Parser.UnpackContext -> visitUnpack(ctx)
        is Basic2Parser.DeclContext -> visitDecl(ctx)
        is Basic2Parser.ImplContext -> visitImpl(ctx)
        is Basic2Parser.RetContext -> visitRet(ctx)
        is Basic2Parser.VarReAsContext -> visitVarReStmt(ctx.varReStmt())
        else -> TODO(ctx.text)
    }

    fun exprCtx(ctx: Basic2Parser.ExprContext): B2Ast.Expression = when (ctx) {
        is Basic2Parser.StrLitContext -> visitStrLit(ctx)
        is Basic2Parser.BinopContext -> visitBinop(ctx)
        is Basic2Parser.IdentContext -> visitIdent(ctx)
        is Basic2Parser.TupleContext -> visitTuple(ctx)
        is Basic2Parser.IntLitContext -> visitIntLit(ctx)
        is Basic2Parser.ArrIndContext -> visitArrInd(ctx)
        is Basic2Parser.FnCallContext -> visitFnCall(ctx)
        is Basic2Parser.InputContext -> visitInput(ctx)
        else -> TODO(ctx.text)
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
        elseBranch = listOf(stmtCtx(ctx.stmt(0)!!)),
        position = Position.from(ctx.position)
    )

    override fun visitElifBlock(ctx: Basic2Parser.ElifBlockContext) = visitIfElifStmtBlock(ctx.ifElifStmtBlock())

    override fun visitIfElifStmtBlock(ctx: Basic2Parser.IfElifStmtBlockContext) = Statement.If(
        condition = exprCtx(ctx.expr()),
        thenBranch = ctx.thenBlock.map { stmtCtx(it) },
        elif = ctx.elifStmtBlock().map { Pair(exprCtx(it.expr()), it.stmt().map { s -> stmtCtx(s) }) },
        elseBranch = ctx.elseBlock.map { stmtCtx(it) },
        position = Position.from(ctx.position)
    )

    override fun visitElif(ctx: Basic2Parser.ElifContext) = visitIfElifStmt(ctx.ifElifStmt())


    override fun visitIfElifStmt(ctx: Basic2Parser.IfElifStmtContext) = Statement.If(
        condition = exprCtx(ctx.expr(0)!!),
        thenBranch = listOf(stmtCtx(ctx.stmt(0)!!)),
        elif = ctx.stmt().subList(1, ctx.stmt().size - 1).zip(ctx.expr().subList(1, ctx.stmt().size))
            .map { (stmt, expr) -> Pair(exprCtx(expr), listOf(stmtCtx(stmt))) },
        elseBranch = listOf(stmtCtx(ctx.stmt(ctx.stmt().size - 1)!!)),
        position = Position.from(ctx.position)
    )

    override fun visitIfBlock(ctx: Basic2Parser.IfBlockContext) = visitIfStmtBlock(ctx.ifStmtBlock())

    override fun visitIfElseStmtBlock(ctx: Basic2Parser.IfElseStmtBlockContext) = Statement.If(
        condition = exprCtx(ctx.expr()),
        thenBranch = ctx.ifThenBlock().stmt().map { stmtCtx(it) },
        elseBranch = ctx.ifElseBlock().stmt().map { stmtCtx(it) },
        position = Position.from(ctx.position)
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
        thenBranch = listOf(stmtCtx(ctx.stmt())),
        position = Position.from(ctx.position)
    )

    // LOOP STATEMENTS =========================================================

    override fun visitWhile(ctx: Basic2Parser.WhileContext) = visitWhileStmt(ctx.whileStmt())

    override fun visitWhileStmt(ctx: Basic2Parser.WhileStmtContext) = Statement.While(
        condition = exprCtx(ctx.expr()),
        body = listOf(stmtCtx(ctx.stmt())),
        position = Position.from(ctx.position)
    )

    override fun visitWhileBlock(ctx: Basic2Parser.WhileBlockContext) = visitWhileStmtBlock(ctx.whileStmtBlock())

    override fun visitWhileStmtBlock(ctx: Basic2Parser.WhileStmtBlockContext) = Statement.While(
        condition = exprCtx(ctx.expr()),
        body = ctx.blockStmt().stmt().map { stmtCtx(it) },
        position = Position.from(ctx.position)
    )

    override fun visitFor(ctx: Basic2Parser.ForContext) = visitForStmt(ctx.forStmt())

    override fun visitForStmt(ctx: Basic2Parser.ForStmtContext) = Statement.For(
        i = ctx.IDENTIFIER(0)?.text!!,
        h = ctx.IDENTIFIER(1)?.text!!,
        init = exprCtx(ctx.expr(0)!!),
        cond = B2Ast.Expression.BinOp(
            left = B2Ast.Expression.Var(ctx.IDENTIFIER(1)?.text!!, null),
            operator = visitComp(ctx.comp()),
            right = exprCtx(ctx.expr(1)!!),
            null
        ),
        incr = ctx.expr(2)?.let {
            B2Ast.Expression.BinOp(
                left = B2Ast.Expression.Var(ctx.IDENTIFIER(2)?.text!!, null),
                operator = visitIncr(ctx.incr()!!),
                right = exprCtx(it),
                null
            )
        } ?: B2Ast.Expression.UniOp(
            value = B2Ast.Expression.Var(ctx.IDENTIFIER(2)?.text!!, null),
            operator = visitIncrUni(ctx.incrUni()!!),
            null
            ),
        body = listOf(stmtCtx(ctx.stmt())),
        position = Position.from(ctx.position)
    )

    override fun visitForR(ctx: Basic2Parser.ForRContext) = visitForRange(ctx.forRange())

    override fun visitForRange(ctx: Basic2Parser.ForRangeContext) = Statement.ForIter(
        ident = ctx.IDENTIFIER().text,
        iter = visitIterable(ctx.iterable()),
        body = listOf(stmtCtx(ctx.stmt())),
        position = Position.from(ctx.position)
    )

    // BREAK STATEMENTS ========================================================

    override fun visitRet(ctx: Basic2Parser.RetContext) = visitReturnStmt(ctx.returnStmt())

    override fun visitReturnStmt(ctx: Basic2Parser.ReturnStmtContext) =
        Statement.Return(ctx.expr()?.let { exprCtx(it) }, Position.from(ctx.position))

    override fun visitBrk(ctx: Basic2Parser.BrkContext) = visitBreakStmt(ctx.breakStmt())

    override fun visitBreakStmt(ctx: Basic2Parser.BreakStmtContext) = Statement.Break(Position.from(ctx.position))

    override fun visitCont(ctx: Basic2Parser.ContContext) = visitContinueStmt(ctx.continueStmt())

    override fun visitContinueStmt(ctx: Basic2Parser.ContinueStmtContext)
        = Statement.Continue(Position.from(ctx.position))

    // BUILTINS ================================================================

    override fun visitPrnt(ctx: Basic2Parser.PrntContext) = visitPrintStmt(ctx.printStmt())

    override fun visitPrintStmt(ctx: Basic2Parser.PrintStmtContext)
        = Statement.Print(exprCtx(ctx.expr()), Position.from(ctx.position))

    override fun visitInputExpr(ctx: Basic2Parser.InputExprContext) = B2Ast.Expression.Input(
        ctx.expr()?.let { exprCtx(it) },
        Position.from(ctx.position)
    )

    override fun visitLenExpr(ctx: Basic2Parser.LenExprContext)
        = B2Ast.Expression.Len(exprCtx(ctx.expr()), Position.from(ctx.position))

    override fun visitInput(ctx: Basic2Parser.InputContext) = visitInputExpr(ctx.inputExpr())

    override fun visitArrReAss(ctx: Basic2Parser.ArrReAssContext) = visitArrReStmt(ctx.arrReStmt())

    override fun visitArrReStmt(ctx: Basic2Parser.ArrReStmtContext) = Statement.ArrIndReAssign(
        ident = ctx.IDENTIFIER().text,
        ind = exprCtx(ctx.expr(0)!!),
        newValue = exprCtx(ctx.expr(1)!!),
        position = Position.from(ctx.position)
    )

    override fun visitVarDecl(ctx: Basic2Parser.VarDeclContext) = visitVarDeclStmt(ctx.varDeclStmt())

    override fun visitVarAs(ctx: Basic2Parser.VarAsContext) = visitVarDeclAssignStmt(ctx.varDeclAssignStmt())

    override fun visitVarReAs(ctx: Basic2Parser.VarReAsContext) = visitVarReStmt(ctx.varReStmt())

    override fun visitBlock(ctx: Basic2Parser.BlockContext) =
        Statement.Block(ctx.blockStmt().stmt().map { stmtCtx(it) }, Position.from(ctx.position))

    override fun visitDecl(ctx: Basic2Parser.DeclContext) = visitFnDeclStmt(ctx.fnDeclStmt())

    override fun visitImpl(ctx: Basic2Parser.ImplContext) = visitFnImplStmt(ctx.fnImplStmt())

    override fun visitPreIncr(ctx: Basic2Parser.PreIncrContext) = B2Ast.Expression.UniOp(
        value = exprCtx(ctx.expr()),
        operator = visitIncrUni(ctx.incrUni()),
        Position.from(ctx.position)
    )

    override fun visitBinopIncr(ctx: Basic2Parser.BinopIncrContext) = B2Ast.Expression.BinOp(
        left = B2Ast.Expression.Var(ctx.IDENTIFIER().text, null),
        operator = visitIncr(ctx.incr()),
        right = exprCtx(ctx.expr()),
        Position.from(ctx.position)
    )

    // IMPORTS =================================================================

    override fun visitUseAll(ctx: Basic2Parser.UseAllContext) = Statement.Import(
        moduleIdent = ctx.IDENTIFIER().text,
        rename = ctx.renaming()?.IDENTIFIER()?.text,
        Position.from(ctx.position)
    )

    override fun visitUseSpecific(ctx: Basic2Parser.UseSpecificContext) = Statement.ImportSpecific(
        moduleIdent = ctx.IDENTIFIER().text,
        imports = ctx.importItems().map {
            when (it) {
                is Basic2Parser.VarContext -> Pair(visitVar(it), it.renaming()?.let { r -> visitRenaming(r).value })
                is Basic2Parser.FnDeclContext -> Pair(
                    visitFnDecl(it),
                    it.renaming()?.let { r -> visitRenaming(r).value })

                is Basic2Parser.FnImplContext -> Pair(
                    visitFnImpl(it),
                    it.renaming()?.let { r -> visitRenaming(r).value })

                is Basic2Parser.FnDeclImplContext -> Pair(
                    visitFnDeclImpl(it),
                    it.renaming()?.let { r -> visitRenaming(r).value })

                else -> TODO()
            }
        },
        Position.from(ctx.position)
    )

    override fun visitRenaming(ctx: Basic2Parser.RenamingContext)
        = B2Ast.Expression.Str(ctx.IDENTIFIER().text, Position.from(ctx.position))

    override fun visitFnDecl(ctx: Basic2Parser.FnDeclContext) = Statement.ImportItem.FnDecl(
        ident = ctx.IDENTIFIER().text,
        Position.from(ctx.position)
    )

    override fun visitFnImpl(ctx: Basic2Parser.FnImplContext) = Statement.ImportItem.FnImpl(
        ident = ctx.IDENTIFIER().text,
        Position.from(ctx.position)
    )

    override fun visitFnDeclImpl(ctx: Basic2Parser.FnDeclImplContext) = Statement.ImportItem.FnDeclImpl(
        ident = ctx.IDENTIFIER().text,
        Position.from(ctx.position)
    )

    override fun visitVar(ctx: Basic2Parser.VarContext)
        = Statement.ImportItem.Var(ctx.IDENTIFIER().text, Position.from(ctx.position))

    // VARIABLES ===============================================================

    override fun visitVarDeclStmt(ctx: Basic2Parser.VarDeclStmtContext) = Statement.VarDecl(
        ident = ctx.IDENTIFIER().text,
        type = visitTyping(ctx.typing()),
        Position.from(ctx.position)
    )

    override fun visitVarDeclAssignStmt(ctx: Basic2Parser.VarDeclAssignStmtContext): Statement.VarAssDecl {
        val value = exprCtx(ctx.expr())
        val type = ctx.typing()?.let { visitTyping(it) } ?: value.type()
        return Statement.VarAssDecl(
            ident = ctx.IDENTIFIER().text,
            type,
            value,
            Position.from(ctx.position)
        )
    }

    override fun visitVarReStmt(ctx: Basic2Parser.VarReStmtContext) = Statement.VarReAssign(
        ident = ctx.IDENTIFIER().text,
        newValue = exprCtx(ctx.expr()),
        Position.from(ctx.position)
    )

    override fun visitTyping(ctx: Basic2Parser.TypingContext) = visitType(ctx.type())

    override fun visitBlockStmt(ctx: Basic2Parser.BlockStmtContext)
        = Statement.Block(ctx.stmt().map { stmtCtx(it) }, Position.from(ctx.position))

    override fun visitFnDeclStmt(ctx: Basic2Parser.FnDeclStmtContext) = Statement.FnDecl(
        ident = ctx.IDENTIFIER().text,
        params = ctx.type().subList(0, ctx.type().size - 1).map { visitType(it) },
        resultType = visitType(ctx.type().last()),
        Position.from(ctx.position)
    )

    override fun visitFnImplStmt(ctx: Basic2Parser.FnImplStmtContext) = Statement.FnImpl(
        ident = ctx.IDENTIFIER().text,
        args = ctx.fnParam().map { Pair(it.IDENTIFIER().text, it.expr()?.let { e -> exprCtx(e) }) },
        body = listOf(stmtCtx(ctx.stmt())),
        Position.from(ctx.position)
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
        ctx.IDENTIFIER() != null -> Type.TypeAlias(ctx.IDENTIFIER()?.text!!)
        ctx.PRIM_TYPES() != null -> Type.from(ctx.PRIM_TYPES()!!.text)
        ctx.ARRAY_STRT() != null -> Type.Lst(visitType(ctx.type(0)!!))
        ctx.TUPLE_STRT() != null -> Type.Tup(visitType(ctx.type(0)!!), visitType(ctx.type(1)!!))
        else -> TODO()
    }

    override fun visitIterable(ctx: Basic2Parser.IterableContext): B2Ast.Expression.Arr =
        when (val expr = exprCtx(ctx.expr(0)!!)) {
            is B2Ast.Expression.Arr -> expr
            is B2Ast.Expression.Str -> {
                val value: Array<B2Ast.Expression> =
                    expr.value.chars()
                        .toList().map { B2Ast.Expression.Str(it.toChar().toString(), null) }.toTypedArray()
                B2Ast.Expression.Arr(
                    value,
                    Type.Str,
                    value.size,
                    Position.from(ctx.position)
                )
            }

            else -> {
                val len: Int = exprCtx(ctx.expr(1)!!).value() as Int
                B2Ast.Expression.Arr(
                    (expr.value() as Int..len)
                        .map { B2Ast.Expression.Int(it, null) }.toTypedArray(),
                    Type.Int,
                    len,
                    Position.from(ctx.position)
                )
            }
        }

    override fun visitBinOp(ctx: Basic2Parser.BinOpContext) = when (val op = ctx.text) {
        "+" -> B2Ast.Operator.Bi.Add
        else -> TODO(op)
    }

    override fun visitIdent(ctx: Basic2Parser.IdentContext)
        = B2Ast.Expression.Var(ctx.IDENTIFIER().text, Position.from(ctx.position))

    override fun visitIntLit(ctx: Basic2Parser.IntLitContext)
        = B2Ast.Expression.Int(ctx.NUM_LIT().text.toInt(), Position.from(ctx.position))

    override fun visitTuple(ctx: Basic2Parser.TupleContext) =
        B2Ast.Expression.Tup(exprCtx(ctx.expr(0)!!), exprCtx(ctx.expr(1)!!), Position.from(ctx.position))

    override fun visitCast(ctx: Basic2Parser.CastContext) =
        B2Ast.Expression.Cast(exprCtx(ctx.expr()), visitType(ctx.type()), Position.from(ctx.position))

    override fun visitStrLit(ctx: Basic2Parser.StrLitContext)
        = B2Ast.Expression.Str(ctx.STR_LIT().text, Position.from(ctx.position))

    override fun visitFloatLit(ctx: Basic2Parser.FloatLitContext)
        = B2Ast.Expression.Flt(ctx.FLOAT_LIT().text.toFloat(), Position.from(ctx.position))

    override fun visitTrim(ctx: Basic2Parser.TrimContext) =
        B2Ast.Expression.Trim(exprCtx(ctx.expr()) as B2Ast.Expression.Str, Position.from(ctx.position))

    override fun visitLen(ctx: Basic2Parser.LenContext) = visitLenExpr(ctx.lenExpr())

    override fun visitBoolLit(ctx: Basic2Parser.BoolLitContext)
        = B2Ast.Expression.Bol(ctx.BOOL_LIT().text == "TRUE", Position.from(ctx.position))

    override fun visitArrLit(ctx: Basic2Parser.ArrLitContext): B2Ast.Expression.Arr {
        val size = ctx.NUM_LIT().text.toInt()
        val elementType = visitType(ctx.type())
        return B2Ast.Expression.Arr(
            value = (0..size).map { elementType.default() }.toTypedArray(),
            elementType,
            size,
            Position.from(ctx.position)
        )
    }

    override fun visitFnCall(ctx: Basic2Parser.FnCallContext) = B2Ast.Expression.FnCall(
        ident = ctx.IDENTIFIER().text,
        args = ctx.expr().map { exprCtx(it) },
        Position.from(ctx.position)
    )

    override fun visitArrExpLit(ctx: Basic2Parser.ArrExpLitContext): B2Ast.Expression.Arr {
        val value = ctx.expr().map { exprCtx(it) }
        return B2Ast.Expression.Arr(
            value.toTypedArray(),
            elementType = value.first().type(),
            size = value.size,
            Position.from(ctx.position)
        )
    }

    override fun visitBinop(ctx: Basic2Parser.BinopContext) = B2Ast.Expression.BinOp(
        left = exprCtx(ctx.expr(0)!!),
        operator = visitBinOp(ctx.binOp()),
        right = exprCtx(ctx.expr(1)!!),
        Position.from(ctx.position)
    )

    override fun visitGroup(ctx: Basic2Parser.GroupContext)
        = B2Ast.Expression.Group(exprCtx(ctx.expr()), Position.from(ctx.position))

    override fun visitArrInd(ctx: Basic2Parser.ArrIndContext) = B2Ast.Expression.ArrInd(
        arr = exprCtx(ctx.expr(0)!!),
        ind = exprCtx(ctx.expr(1)!!),
        Position.from(ctx.position)
    )

    override fun visitBinopComp(ctx: Basic2Parser.BinopCompContext) = B2Ast.Expression.BinOp(
        left = exprCtx(ctx.expr(0)!!),
        operator = visitComp(ctx.comp()),
        right = exprCtx(ctx.expr(1)!!),
        Position.from(ctx.position)
    )

    override fun visitUnpack(ctx: Basic2Parser.UnpackContext) = visitUnpack_stmt(ctx.unpack_stmt())

    override fun visitUnpack_stmt(ctx: Basic2Parser.Unpack_stmtContext) = Statement.Unpack(
        ctx.IDENTIFIER().map { it.text }.toTypedArray(),
        exprCtx(ctx.expr()),
        Position.from(ctx.position)
    )

    override fun visitTypeAlias(ctx: Basic2Parser.TypeAliasContext) = visitTypeAliasStmt(ctx.typeAliasStmt())

    override fun visitTypeAliasStmt(ctx: Basic2Parser.TypeAliasStmtContext) = Statement.TypeAlias(
        ctx.IDENTIFIER().text,
        visitType(ctx.type())
    )
}
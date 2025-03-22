package b2

import b2.symbols.Symbol
import no.nilsmf.antlr.Basic2Parser

open class B2TypeChecker() : B2() {

    override fun defaultResult() = Symbol.Var.Type.TUnit

    // EXPRESSIONS =============================================================

    fun exprTypeCtx(ctx: Basic2Parser.ExprContext): Symbol.Var.Type = when (ctx) {
        is Basic2Parser.NumContext -> visitNum(ctx)
        is Basic2Parser.FloatContext -> visitFloat(ctx)
        is Basic2Parser.BoolContext -> visitBool(ctx)
        is Basic2Parser.TupleContext -> visitTuple(ctx)
        is Basic2Parser.FnCallContext -> visitFnCall(ctx)
        is Basic2Parser.BinopContext -> visitBinop(ctx)
        is Basic2Parser.IdentContext -> visitIdent(ctx)
        is Basic2Parser.StrContext -> visitStr(ctx)
        is Basic2Parser.TrimContext -> visitTrim(ctx)
        is Basic2Parser.ArrayContext -> visitArray(ctx)
        is Basic2Parser.ArrIndContext -> visitArrInd(ctx)
        is Basic2Parser.GroupContext -> visitGroup(ctx)
        is Basic2Parser.BinopCompContext -> visitBinopComp(ctx)
        is Basic2Parser.CastContext -> visitCast(ctx)
        else -> TODO("Not implemented for: ${ctx.text}")
    }

    override fun visitGroup(ctx: Basic2Parser.GroupContext) = exprTypeCtx(ctx.expr())

    override fun visitIdent(ctx: Basic2Parser.IdentContext): Symbol.Var.Type
            = getSymbolTable().getVar(ctx.IDENTIFIER().text).type()

    override fun visitFloat(ctx: Basic2Parser.FloatContext): Symbol.Var.Type.TFloat = try {
        ctx.FLOAT_LIT().text.replace("f", "").toFloat()
        Symbol.Var.Type.TFloat
    } catch (_: NumberFormatException) {
        throw B2Exception.TypeException.NumberFormatException(ctx.text, ctx.position)
    }

    override fun visitNum(ctx: Basic2Parser.NumContext): Symbol.Var.Type.TInt = try {
        ctx.text.toInt()
        Symbol.Var.Type.TInt
    } catch (_: NumberFormatException) {
        throw B2Exception.TypeException.NumberFormatException(ctx.text, ctx.position)
    }

    override fun visitStr(ctx: Basic2Parser.StrContext) = Symbol.Var.Type.TStr

    override fun visitBool(ctx: Basic2Parser.BoolContext): Symbol.Var.Type.TBool = when (ctx.text) {
            "TRUE" -> Symbol.Var.Type.TBool
            "FALSE" -> Symbol.Var.Type.TBool
            else -> throw B2Exception.TypeException.NotBoolException(ctx.text, ctx.position)
        }

    override fun visitTernary(ctx: Basic2Parser.TernaryContext): Symbol.Var.Type.Tuple {
        val left = exprTypeCtx(ctx.expr(1)!!)
        val right = exprTypeCtx(ctx.expr(2)!!)
        return Symbol.Var.Type.Tuple(left, right)
    }

    override fun visitTuple(ctx: Basic2Parser.TupleContext): Symbol.Var.Type.Tuple {
        val fst = exprTypeCtx(ctx.expr(0)!!)
        val snd = exprTypeCtx(ctx.expr(1)!!)
        return Symbol.Var.Type.Tuple(fst, snd)
    }

    override fun visitArray(ctx: Basic2Parser.ArrayContext): Symbol.Var.Type.TList
            = ctx.expr()
        .map { exprTypeCtx(it) }
        .let { Symbol.Var.Type.TList(Symbol.Var.Type.TUnit) }

    override fun visitArrInd(ctx: Basic2Parser.ArrIndContext): Symbol.Var.Type {
        val arr = when (val arr = exprTypeCtx(ctx.expr(0)!!)) {
            is Symbol.Var.Type.TList -> arr
            else -> throw RuntimeException("Cannot infer type from indexing on type: $arr")
        }

        return arr.t
    }

    override fun visitFnCall(ctx: Basic2Parser.FnCallContext)
        = getSymbolTable().getDecl(ctx.IDENTIFIER().text).resultType

    override fun visitBinop(ctx: Basic2Parser.BinopContext): Symbol.Var.Type {
        val left = exprTypeCtx(ctx.expr(0)!!)
        val right = exprTypeCtx(ctx.expr(1)!!)
        return when (ctx.bin_op().text) {
            "+" -> left + right
            "-" -> left - right
            "*" -> left * right
            "%" -> left % right
            else -> TODO("Missing operator: ${ctx.bin_op().text}")
        }
    }

    override fun visitTrim(ctx: Basic2Parser.TrimContext) = Symbol.Var.Type.TStr

    override fun visitBinopComp(ctx: Basic2Parser.BinopCompContext) = Symbol.Var.Type.TBool

    override fun visitCast(ctx: Basic2Parser.CastContext)
        = Symbol.Var.Value.withType(exprTypeCtx(ctx.expr()).default(), visitType(ctx.type())).type()

    // STATEMENTS ==============================================================

    open fun stmtTypeCtx(ctx: Basic2Parser.StmtContext): Symbol.Var.Type = when (ctx) {
        is Basic2Parser.Var_declContext -> visitVar_decl(ctx)
        is Basic2Parser.Var_re_assContext -> visitVar_re_ass(ctx)
        is Basic2Parser.Var_assContext -> visitVar_ass(ctx)
        is Basic2Parser.AppendContext -> {
            super.visitAppend(ctx)
            Symbol.Var.Type.TUnit
        }
        is Basic2Parser.If_blockContext -> visitIf_block(ctx)
        is Basic2Parser.While_blockContext -> visitWhile_block(ctx)
        is Basic2Parser.ArrReAssContext -> {
            super.visitArr_re_ass_stmt(ctx.arr_re_ass_stmt())
            Symbol.Var.Type.TUnit
        }
        is Basic2Parser.IfContext -> visitIf(ctx)
        is Basic2Parser.PrintContext -> {
            visitPrint(ctx)
            Symbol.Var.Type.TUnit
        }
        is Basic2Parser.BlockContext -> visitBlock(ctx)
        is Basic2Parser.InputContext -> visitInput(ctx)
        is Basic2Parser.RetContext -> visitRet(ctx)
        is Basic2Parser.BreakContext -> visitBreak(ctx)
        is Basic2Parser.BinopIncrContext -> visitBinopIncr(ctx)
        else -> TODO(ctx.text)
    }


    // ============================= ARR-STATEMENTS ============================

    override fun visitArr_re_ass_stmt(ctx: Basic2Parser.Arr_re_ass_stmtContext): Symbol.Var.Type.TUnit {
        val id = ctx.IDENTIFIER().text
        val arr = getSymbolTable().getVar(id)
        if (arr.type() !is Symbol.Var.Type.TList)
            throw B2Exception.TypeException.InvalidIndexingException(id, arr.type(), ctx.position)
        val ind = exprTypeCtx(ctx.expr(0)!!)
        if (ind !is Symbol.Var.Type.TInt)
            throw B2Exception.TypeException.InvalidIndexingTypeException(id, ind, ctx.position)
        val element = exprTypeCtx(ctx.expr(1)!!)
        if (Symbol.Var.Type.TList(element) != arr.type())
            throw B2Exception.TypeException.InvalidIndexingElementException(id, arr.type(), element, ctx.position)
        return Symbol.Var.Type.TUnit
    }

// ============================= VAR-STATEMENTS ============================

    override fun visitVar_decl(c: Basic2Parser.Var_declContext): Symbol.Var.Type.TUnit {
        val ctx = c.var_decl_stmt()
        val id = ctx.IDENTIFIER().text
        val type = visitTyping(ctx.typing())
        getSymbolTable().declVar(id, type)
        return Symbol.Var.Type.TUnit
    }

    override fun visitVar_ass(c: Basic2Parser.Var_assContext): Symbol.Var.Type.TUnit {
        val ctx = c.var_decl_ass_stmt()
        val id = ctx.IDENTIFIER().text
        val value = exprTypeCtx(ctx.expr())
        val type = ctx.typing()?.let { visitTyping(it) }
        try {
            getSymbolTable().declAssVar(id, value.default(), type)
        } catch (_: NumberFormatException) {
            throw B2Exception.TypeException.InvalidCastException(id, value, type!!, ctx.position)
        }
        return Symbol.Var.Type.TUnit
    }

    override fun visitVar_re_ass(c: Basic2Parser.Var_re_assContext): Symbol.Var.Type.TUnit {
        val ctx = c.var_re_ass_stmt()
        val id = ctx.IDENTIFIER().text
        val value = exprTypeCtx(ctx.expr())
        val old = getSymbolTable().getVar(id)
        if (old.type() != value)
            throw B2Exception.TypeException.ReassignmentException(id, value, old.type(), ctx.position)
        return Symbol.Var.Type.TUnit
    }

    // ============================= IF-STATEMENTS =============================

    override fun visitIf_block(c: Basic2Parser.If_blockContext): Symbol.Var.Type = runScope {
        val ctx = c.if_stmt_block()
        if (exprTypeCtx(ctx.expr()) !is Symbol.Var.Type.TBool) {
            throw B2Exception.TypeException.NotBoolException(ctx.text, ctx.position)
        }
        visitBlock_stmt(ctx.block_stmt())
        Symbol.Var.Type.TUnit
    } as Symbol.Var.Type

    override fun visitIf(c: Basic2Parser.IfContext): Symbol.Var.Type = runScope {
        val ctx = c.if_stmt()
        stmtTypeCtx(ctx.stmt())
    } as Symbol.Var.Type

    override fun visitIf_else_block(ctx: Basic2Parser.If_else_blockContext) =
        visitIf_else_stmt_block(ctx.if_else_stmt_block())

    override fun visitIf_else_stmt_block(ctx: Basic2Parser.If_else_stmt_blockContext) = runScope {
        if (exprTypeCtx(ctx.expr()) !is Symbol.Var.Type.TBool)
            throw B2Exception.TypeException.NotBoolException(ctx.text, ctx.position)
        ctx.block_stmt(0)?.let { visitBlock_stmt(it) }
        ctx.block_stmt(1)?.let { visitBlock_stmt(it) } ?: Symbol.Var.Type.TUnit
    }

    override fun visitIf_else(ctx: Basic2Parser.If_elseContext) = visitIf_else_stmt(ctx.if_else_stmt())

    override fun visitIf_else_stmt(ctx: Basic2Parser.If_else_stmtContext) = runScope {
        if (exprTypeCtx(ctx.expr()) !is Symbol.Var.Type.TBool)
            throw B2Exception.TypeException.NotBoolException(ctx.text, ctx.position)
        ctx.stmt(0)?.let { stmtTypeCtx(it) }
        ctx.stmt(1)?.let { stmtTypeCtx(it) } ?: Symbol.Var.Type.TUnit
    }

    override fun visitIf_stmt(ctx: Basic2Parser.If_stmtContext): Symbol {
        return super.visitIf_stmt(ctx)
    }

    override fun visitIf_stmt_block(ctx: Basic2Parser.If_stmt_blockContext): Symbol {
        return super.visitIf_stmt_block(ctx)
    }

    // ============================ LOOP-STATEMENTS ============================

    override fun visitWhile(ctx: Basic2Parser.WhileContext) = visitWhile_stmt(ctx.while_stmt())

    override fun visitWhile_stmt(ctx: Basic2Parser.While_stmtContext) = runLoop {
        if (exprTypeCtx(ctx.expr()) !is Symbol.Var.Type.TBool)
            throw B2Exception.TypeException.NotBoolException(ctx.text, ctx.position)
        stmtTypeCtx(ctx.stmt())
    }

    override fun visitWhile_block(ctx: Basic2Parser.While_blockContext) = visitWhile_stmt_block(ctx.while_stmt_block())

    override fun visitWhile_stmt_block(ctx: Basic2Parser.While_stmt_blockContext): Symbol.Var.Type = runLoop {
        if (exprTypeCtx(ctx.expr()) !is Symbol.Var.Type.TBool)
            throw B2Exception.TypeException.NotBoolException(ctx.text, ctx.position)
        stmtLst(ctx.block_stmt().stmt())
    } as Symbol.Var.Type

    override fun visitFor_r(ctx: Basic2Parser.For_rContext) = visitFor_range(ctx.for_range())

    override fun visitFor_range(ctx: Basic2Parser.For_rangeContext) = runLoop {
        visitIterable(ctx.iterable())
        stmtTypeCtx(ctx.stmt())
    }

    override fun visitFor(ctx: Basic2Parser.ForContext) = visitFor_stmt(ctx.for_stmt())

    override fun visitFor_stmt(ctx: Basic2Parser.For_stmtContext) = runLoop {
        val startId = ctx.IDENTIFIER(0)!!.text
        val startValue = exprTypeCtx(ctx.expr(0)!!).default()
        getSymbolTable().declAssVar(startId, startValue)
        val compID = ctx.IDENTIFIER(0)!!.text
        val op = B2Eval.getCompOp(ctx.comp().text)
        val compValue = exprTypeCtx(ctx.expr(1)!!)
        val compOp = {
            val left = getSymbolTable().getVar(compID)
            op(left, compValue.default())
        }
        val incrId = ctx.IDENTIFIER(0)!!.text
        val incrOp = {
            ctx.incr()?.let {
                val iOp = B2Stmt.binIncr(it.text)
                val old = getSymbolTable().getVar(incrId)
                val incr = exprTypeCtx(ctx.expr(2)!!).default()
                val newValue = iOp(old, incr)
                getSymbolTable().reAssVar(incrId, newValue)
            } ?: ctx.incr_uni()?.let {
                val iOp = B2Stmt.uniIncr(it.text)
                getSymbolTable().reAssVar(incrId, iOp(getSymbolTable().getVar(incrId)))
            }!!
        }
        compOp()
        val r = stmtTypeCtx(ctx.stmt())
        incrOp()
        r
    }

    override fun visitIterable(ctx: Basic2Parser.IterableContext): Symbol.Var.Type.TList = ctx.expr()?.let {
        when (val v = exprTypeCtx(it)) {
            is Symbol.Var.Type.TList -> v
            else -> throw B2Exception.TypeException.NotIterableException(v, ctx.position)
        }
    } ?: Symbol.Var.Type.TList(Symbol.Var.Type.TInt)

    // ============================ BREAK-STATEMENTS ===========================

    override fun visitRet(ctx: Basic2Parser.RetContext) = visitReturn_stmt(ctx.return_stmt())

    override fun visitReturn_stmt(ctx: Basic2Parser.Return_stmtContext)
            = throw B2Exception.ReturnException(ctx.expr()?.let { exprTypeCtx(it).default() } ?: Symbol.Var.Value.VUnit)

    override fun visitBreak(ctx: Basic2Parser.BreakContext) = visitBreak_stmt(ctx.break_stmt())

    override fun visitBreak_stmt(ctx: Basic2Parser.Break_stmtContext) = throw B2Exception.BreakException

    // =========================== BUILTINS-STATEMENTS =========================

    override fun visitInput(ctx: Basic2Parser.InputContext) = visitInput_stmt(ctx.input_stmt())

    override fun visitInput_stmt(ctx: Basic2Parser.Input_stmtContext): Symbol.Var.Type
        = ctx.typing()?.let { visitTyping(it) } ?: Symbol.Var.Type.TStr

    override fun visitLen(ctx: Basic2Parser.LenContext) = Symbol.Var.Type.TInt

    // ============================ BLOCK-STATEMENTS ===========================

    override fun visitBlock(ctx: Basic2Parser.BlockContext) = visitBlock_stmt(ctx.block_stmt())

    override fun visitBlock_stmt(ctx: Basic2Parser.Block_stmtContext): Symbol.Var.Type = stmtLst(ctx.stmt())

    fun stmtLst(stmt: List<Basic2Parser.StmtContext>): Symbol.Var.Type = runScope {
        var value: Symbol.Var.Type = Symbol.Var.Type.TUnit
        stmt.forEach { value = stmtTypeCtx(it) }
        value
    } as Symbol.Var.Type

    // ========================== FUNCTION-STATEMENTS ==========================

    override fun visitFn_decl(ctx: Basic2Parser.Fn_declContext) = visitFn_decl_stmt(ctx.fn_decl_stmt())

    override fun visitFn_decl_stmt(ctx: Basic2Parser.Fn_decl_stmtContext): Symbol.Var.Type.TUnit {
        B2Stmt.visitFnDeclStmt(ctx, getSymbolTable())
        return Symbol.Var.Type.TUnit
    }

    override fun visitFn_impl(ctx: Basic2Parser.Fn_implContext) = visitFn_impl_stmt(ctx.fn_impl_stmt())

    override fun visitFn_impl_stmt(ctx: Basic2Parser.Fn_impl_stmtContext): Symbol.Var.Type.TUnit {
        val id = ctx.IDENTIFIER().text
        var argsParams = ctx.fn_param().map { visitFn_param(it) }
        val decl = getSymbolTable().getDecl(id)
        if (decl.params.size != argsParams.size)
            throw B2Exception.TypeException.NonMatchingParamException(id, ctx.position)
        decl.params.zip(argsParams).forEach {
            (param, arg) -> arg.value?.let {
                if (it.type() != param.type)
                    throw B2Exception.TypeException.NonMatchingParamTypeException(id, ctx.position)
            }
        }
        val body = { args: List<Symbol.Var.Value?> ->
            args.forEachIndexed { i, arg -> arg?.let { argsParams[i].value = it } }
            runFnCall {
                argsParams.forEach { getSymbolTable().declAssVar(it.id, it.value!!) }
                argsParams.forEach { getSymbolTable().remVar(it.id) }
                Symbol.Var.Value.VUnit
            } as Symbol.Var.Value.VUnit
        }
        getSymbolTable().addFnImpl(id, argsParams, body)
        return Symbol.Var.Type.TUnit
    }

    override fun visitFn_param(ctx: Basic2Parser.Fn_paramContext)
            = Symbol.Arg(ctx.IDENTIFIER().text, ctx.expr()?.let { exprTypeCtx(it).default() })

    // ========================== INCREMENT-STATEMENTS =========================

    override fun visitPreIncr(ctx: Basic2Parser.PreIncrContext): Symbol {
        return TODO()
    }

    override fun visitPostIncr(ctx: Basic2Parser.PostIncrContext): Symbol {
        return TODO()
    }

    override fun visitBinopIncr(ctx: Basic2Parser.BinopIncrContext): Symbol.Var.Type.TUnit {
        val id = ctx.IDENTIFIER().text
        var v = getSymbolTable().getVar(id)
        val i = exprTypeCtx(ctx.expr()).default()
        val newValue = B2Stmt.binIncr(ctx.incr().text)(v, i)
        getSymbolTable().reAssVar(id, newValue)
        return Symbol.Var.Type.TUnit
    }

    // ========================== TYPING-STATEMENTS ==========================

    override fun visitTyping(ctx: Basic2Parser.TypingContext): Symbol.Var.Type = visitType(ctx.type())


    override fun visitType(ctx: Basic2Parser.TypeContext) = B2Eval.visitType(ctx)
}
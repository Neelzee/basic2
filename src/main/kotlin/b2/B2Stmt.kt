package b2

import b2.symbols.Symbol
import no.nilsmf.antlr.Basic2Parser
import kotlin.streams.toList

open class B2Stmt : B2Eval() {

    companion object {

        fun visitFnDeclStmt(ctx: Basic2Parser.Fn_decl_stmtContext, symbolTable: SymbolTable): Symbol.Var.Value.VUnit {
            val id = ctx.IDENTIFIER().text
            var types = ctx.type().map { visitType(it) }
            val (typeParams, returnType) = if (types.isEmpty()) {
                Pair(emptyList(), Symbol.Var.Type.TUnit)
            } else {
                Pair(types.subList(0, types.size - 1), types[types.size - 1])
            }
            symbolTable.addFnDecl(id, typeParams, returnType)
            return Symbol.Var.Value.VUnit
        }

        fun uniIncr(kind: String): (Symbol.Var.Value) -> Symbol.Var.Value = when (kind) {
            "++" -> { a -> a + Symbol.Var.Value.VInt(1) }
            "--" -> { a -> a - Symbol.Var.Value.VInt(1) }
            else -> TODO("Missing op: $kind")
        }

        fun binIncr(kind: String): (Symbol.Var.Value, Symbol.Var.Value) -> Symbol.Var.Value = when (kind) {
            "+=" -> { a, b -> a + b }
            "-=" -> { a, b -> a - b }
            "%=" -> { a, b -> a % b }
            "*=" -> { a, b -> a * b }
            "/=" -> { a, b -> a / b }
            "&=" -> { a, b -> a.and(b) }
            "|=" -> { a, b -> a.or(b) }
            "^=" -> { a, b -> a.pow(b) }
            else -> TODO("Missing op: $kind")
        }
    }

    open fun stmtCtx(ctx: Basic2Parser.StmtContext): Symbol.Var.Value = when (ctx) {
        is Basic2Parser.Var_declContext -> visitVar_decl(ctx)
        is Basic2Parser.Var_re_assContext -> visitVar_re_ass_stmt(ctx.var_re_ass_stmt())
        is Basic2Parser.Var_assContext -> visitVar_ass(ctx)
        is Basic2Parser.AppendContext -> visitAppend_stmt(ctx.append_stmt())
        is Basic2Parser.If_blockContext -> visitIf_block(ctx)
        is Basic2Parser.While_blockContext -> visitWhile_block(ctx)
        is Basic2Parser.ArrReAssContext -> visitArr_re_ass_stmt(ctx.arr_re_ass_stmt())
        is Basic2Parser.IfContext -> visitIf(ctx)
        is Basic2Parser.PrintContext -> visitPrint(ctx)
        is Basic2Parser.BlockContext -> visitBlock(ctx)
        is Basic2Parser.InputContext -> visitInput(ctx)
        is Basic2Parser.RetContext -> visitRet(ctx)
        is Basic2Parser.BreakContext -> visitBreak(ctx)
        is Basic2Parser.BinopIncrContext -> visitBinopIncr(ctx)
        else -> TODO(ctx.text)
    } as Symbol.Var.Value

    // ============================= IF-STATEMENTS =============================

    override fun visitIf_elif(c: Basic2Parser.If_elifContext): Symbol.Var.Value = runScope {
        val ctx = c.if_elif_stmt()

        val elseStmt = ctx.stmt().last()

        val stmt = ctx.expr().zip(ctx.stmt().subList(0, ctx.expr().size - 1)).firstOrNull { (pred, _) ->
            exprCtx(pred).toBool()
        }?.second

        val pred = exprCtx(ctx.expr(0)!!).toBool()
        if (pred) {
            stmt?.let { stmtCtx(it) } ?: Symbol.Var.Value.VUnit
        } else {
            stmtCtx(elseStmt)
        }
    } as Symbol.Var.Value

    override fun visitIf_elif_block(c: Basic2Parser.If_elif_blockContext): Symbol.Var.Value = runScope {
        val ctx = c.if_elif_stmt_block()

        val elseStmt = ctx.block_stmt().last()

        val stmt = ctx.expr().zip(ctx.block_stmt().subList(0, ctx.expr().size - 1)).firstOrNull { (pred, _) ->
            exprCtx(pred).toBool()
        }?.second

        val pred = exprCtx(ctx.expr(0)!!).toBool()
        if (pred) {
            stmt?.let { visitBlock_stmt(it) } ?: Symbol.Var.Value.VUnit
        } else {
            visitBlock_stmt(elseStmt)
        }
    } as Symbol.Var.Value

    override fun visitIf_block(c: Basic2Parser.If_blockContext): Symbol.Var.Value = runScope {
        val ctx = c.if_stmt_block()
        val pred = exprCtx(ctx.expr()).toBool()
        if (pred) {
            visitBlock_stmt(ctx.block_stmt())
        } else {
            Symbol.Var.Value.VUnit
        }
    } as Symbol.Var.Value

    override fun visitIf(c: Basic2Parser.IfContext): Symbol.Var.Value = runScope {
        val ctx = c.if_stmt()
        val pred = exprCtx(ctx.expr()).toBool()
        if (pred) {
            stmtCtx(ctx.stmt())
        } else {
            Symbol.Var.Value.VUnit
        }
    } as Symbol.Var.Value

    override fun visitIf_else_block(ctx: Basic2Parser.If_else_blockContext) =
        visitIf_else_stmt_block(ctx.if_else_stmt_block())

    override fun visitIf_else_stmt_block(ctx: Basic2Parser.If_else_stmt_blockContext) = runScope {
        val pred = exprCtx(ctx.expr()).toBool()
        if (pred) {
            ctx.block_stmt(0)?.let { visitBlock_stmt(it) }
        } else {
            ctx.block_stmt(1)?.let { visitBlock_stmt(it) }
        } ?: Symbol.Var.Value.VUnit
    }

    override fun visitIf_else(ctx: Basic2Parser.If_elseContext) = visitIf_else_stmt(ctx.if_else_stmt())

    override fun visitIf_else_stmt(ctx: Basic2Parser.If_else_stmtContext) = runScope {
        val pred = exprCtx(ctx.expr()).toBool()
        if (pred) {
            ctx.stmt(0)?.let { stmtCtx(it) }
        } else {
            ctx.stmt(1)?.let { stmtCtx(it) }
        } ?: Symbol.Var.Value.VUnit
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
        var pred = exprCtx(ctx.expr()).toBool()
        var output: Symbol.Var.Value = Symbol.Var.Value.VUnit
        while (pred) {
            try {
                output = stmtCtx(ctx.stmt())
            } catch (_: B2Exception.ContinueException) {
                continue
            } finally {
                pred = exprCtx(ctx.expr()).toBool()
            }
        }
        output
    }

    override fun visitWhile_block(ctx: Basic2Parser.While_blockContext) = visitWhile_stmt_block(ctx.while_stmt_block())

    override fun visitWhile_stmt_block(ctx: Basic2Parser.While_stmt_blockContext) = runLoop {
        var pred = exprCtx(ctx.expr()).toBool()
        var output: Symbol.Var.Value = Symbol.Var.Value.VUnit
        while (pred) {
            try {
                output = stmtLst(ctx.block_stmt().stmt())
            } catch (_: B2Exception.ContinueException) {
                continue
            } finally {
                pred = exprCtx(ctx.expr()).toBool()
            }
        }
        output
    }

    override fun visitFor_r(ctx: Basic2Parser.For_rContext) = visitFor_range(ctx.for_range())

    override fun visitFor_range(ctx: Basic2Parser.For_rangeContext) = runLoop {
        var value: Symbol.Var.Value = Symbol.Var.Value.VUnit
        val iter = visitIterable(ctx.iterable())
        val id = ctx.IDENTIFIER().text
        getSymbolTable().declVar(id, iter.type.t)
        for (v in iter.toIterable()) {
            getSymbolTable().reAssVar(id, Symbol.Var.Value.from(v))
            try {
                value = stmtCtx(ctx.stmt())
            } catch (_: B2Exception.ContinueException) {
                continue
            }
        }
        value
    }

    override fun visitFor(ctx: Basic2Parser.ForContext) = visitFor_stmt(ctx.for_stmt())

    override fun visitFor_stmt(ctx: Basic2Parser.For_stmtContext) = runLoop {
        var value: Symbol.Var.Value = Symbol.Var.Value.VUnit
        val startId = ctx.IDENTIFIER(0)!!.text
        val startValue = exprCtx(ctx.expr(0)!!)
        getSymbolTable().declAssVar(startId, startValue)
        val compID = ctx.IDENTIFIER(0)!!.text
        val op = getCompOp(ctx.comp().text)
        val compValue = when (val v = exprCtxVar(ctx.expr(1)!!)) {
            is Symbol.Var.Variable -> {
                { getSymbolTable().getVar(v.id) }
            }
            is Symbol.Var.Value -> { { v } }
            else -> throw RuntimeException("??")
        }
        val compOp = {
            val left = getSymbolTable().getVar(compID)
            op(left, compValue())
        }
        val incrId = ctx.IDENTIFIER(0)!!.text
        val incrOp = {
            ctx.incr()?.let {
                val iOp = binIncr(it.text)
                val old = getSymbolTable().getVar(incrId)
                val incr = exprCtx(ctx.expr(2)!!)
                val newValue = iOp(old, incr)
                getSymbolTable().reAssVar(incrId, newValue)
            } ?: ctx.incr_uni()?.let {
                val iOp = uniIncr(it.text)
                getSymbolTable().reAssVar(incrId, iOp(getSymbolTable().getVar(incrId)))
            }!!
        }
        while (compOp()) {
            try {
                value = stmtCtx(ctx.stmt())
            } catch (_: B2Exception.ContinueException) {
                continue
            } finally {
                incrOp()
            }
        }
        value
    }

    override fun visitIterable(ctx: Basic2Parser.IterableContext): Symbol.Var.Value.VList = if (ctx.FROM_KW() != null) {
        Symbol.Var.Value.VList(
            (exprCtx(ctx.expr(0)!!).value() as Int..exprCtx(ctx.expr(1)!!).value() as Int)
                .map { Symbol.Var.Value.VInt(it) }
                .toMutableList(),
            Symbol.Var.Type.TList(Symbol.Var.Type.TInt)
        )
    } else {
        when (val v = exprCtx(ctx.expr(0)!!)) {
            is Symbol.Var.Value.VList -> v
            is Symbol.Var.Value.VString -> Symbol.Var.Value.VList(
                v.value.chars().toList().map { Symbol.Var.Value.VString(it.toChar().toString()) }.toMutableList(),
                Symbol.Var.Type.TList(Symbol.Var.Type.TStr)
            )
            else -> throw RuntimeException("I")
        }
    }

    // ============================ BREAK-STATEMENTS ===========================

    override fun visitRet(ctx: Basic2Parser.RetContext) = visitReturn_stmt(ctx.return_stmt())

    override fun visitReturn_stmt(ctx: Basic2Parser.Return_stmtContext)
        = throw B2Exception.ReturnException(ctx.expr()?.let { exprCtx(it) } ?: Symbol.Var.Value.VUnit)

    override fun visitBreak(ctx: Basic2Parser.BreakContext) = visitBreak_stmt(ctx.break_stmt())

    override fun visitBreak_stmt(ctx: Basic2Parser.Break_stmtContext) = throw B2Exception.BreakException

    // =========================== BUILTINS-STATEMENTS =========================

    override fun visitPrint(ctx: Basic2Parser.PrintContext) = visitPrint_stmt(ctx.print_stmt())

    override fun visitPrint_stmt(ctx: Basic2Parser.Print_stmtContext): Symbol.Var.Value.VUnit {
        val value = exprCtx(ctx.expr())
        println(value.value())
        return Symbol.Var.Value.VUnit
    }

    override fun visitInput(ctx: Basic2Parser.InputContext) = visitInput_stmt(ctx.input_stmt())

    override fun visitInput_stmt(ctx: Basic2Parser.Input_stmtContext): Symbol.Var.Value {
        val prompt = ctx.expr()?.let { exprCtx(it) }?.value()
        print(prompt)
        val result = readlnOrNull()
        val value: Symbol.Var.Value = Symbol.Var.Value.VString(result ?: "")
        val type = ctx.typing()?.let { visitTyping(it) }

        ctx.IDENTIFIER()?.text?.let { getSymbolTable().declAssVar(it, value, type) }
        return value
    }

    override fun visitLen(ctx: Basic2Parser.LenContext) = visitLen_stmt(ctx.len_stmt())

    override fun visitLen_stmt(ctx: Basic2Parser.Len_stmtContext): Symbol.Var.Value {
        val value = exprCtx(ctx.expr())
        return ctx.IDENTIFIER()?.text?.let {
            getSymbolTable().declAssVar(it, Symbol.Var.Value.VInt(value.size()))
            Symbol.Var.Value.VUnit
        } ?: value
    }

    override fun visitAppend(ctx: Basic2Parser.AppendContext) = visitAppend_stmt(ctx.append_stmt())

    override fun visitAppend_stmt(ctx: Basic2Parser.Append_stmtContext): Symbol.Var.Value {
        val element = exprCtx(ctx.expr())
        val id = ctx.IDENTIFIER().text
        return when (val arr = getSymbolTable().getVar(id)) {
            is Symbol.Var.Value.VList -> {
                arr.add(element)
                getSymbolTable().reAssVar(id, arr)
                Symbol.Var.Value.VUnit
            }
            else -> throw RuntimeException("This is wrong")
        }
    }

    // ============================ ARRAY-STATEMENTS ===========================

    override fun visitArrReAss(ctx: Basic2Parser.ArrReAssContext) = visitArr_re_ass_stmt(ctx.arr_re_ass_stmt())

    override fun visitArr_re_ass_stmt(ctx: Basic2Parser.Arr_re_ass_stmtContext): Symbol.Var.Value {
        val id = ctx.IDENTIFIER().text
        val ind = exprCtx(ctx.expr(0)!!) as Symbol.Var.Value.VInt
        val element = exprCtx(ctx.expr(1)!!)
        val value = getSymbolTable().getVar(id)
        value[ind] = element
        getSymbolTable().reAssVar(id, value)
        return Symbol.Var.Value.VUnit
    }

    // ============================= VAR-STATEMENTS ============================

    override fun visitVar_decl(ctx: Basic2Parser.Var_declContext) = visitVar_decl_stmt(ctx.var_decl_stmt())

    override fun visitVar_decl_stmt(ctx: Basic2Parser.Var_decl_stmtContext): Symbol.Var.Value.VUnit {
        val id = ctx.IDENTIFIER().text
        val type = visitTyping(ctx.typing())
        if (id == "_") return Symbol.Var.Value.VUnit
        getSymbolTable().declVar(id, type)
        return Symbol.Var.Value.VUnit
    }

    override fun visitVar_ass(ctx: Basic2Parser.Var_assContext) = visitVar_decl_ass_stmt(ctx.var_decl_ass_stmt())

    override fun visitVar_decl_ass_stmt(ctx: Basic2Parser.Var_decl_ass_stmtContext): Symbol.Var.Value.VUnit {
        val id = ctx.IDENTIFIER().text
        val type = ctx.typing()?.let { visitTyping(it) }
        val value = exprCtx(ctx.expr())
        if (id == "_") return Symbol.Var.Value.VUnit
        getSymbolTable().declAssVar(id, value, type)
        return Symbol.Var.Value.VUnit
    }

    override fun visitVar_re_ass(ctx: Basic2Parser.Var_re_assContext) = visitVar_re_ass_stmt(ctx.var_re_ass_stmt())

    override fun visitVar_re_ass_stmt(ctx: Basic2Parser.Var_re_ass_stmtContext): Symbol.Var.Value.VUnit {
        val id = ctx.IDENTIFIER().text
        val newValue = exprCtx(ctx.expr())
        if (id == "_") return Symbol.Var.Value.VUnit
        getSymbolTable().reAssVar(id, newValue)
        return Symbol.Var.Value.VUnit
    }

    // ============================ BLOCK-STATEMENTS ===========================

    override fun visitBlock(ctx: Basic2Parser.BlockContext) = visitBlock_stmt(ctx.block_stmt())

    override fun visitBlock_stmt(ctx: Basic2Parser.Block_stmtContext): Symbol.Var.Value = stmtLst(ctx.stmt())

    fun stmtLst(stmt: List<Basic2Parser.StmtContext>): Symbol.Var.Value = runScope {
        var value: Symbol.Var.Value = Symbol.Var.Value.VUnit
        stmt.forEach { value = stmtCtx(it) }
        value
    } as Symbol.Var.Value

    // ========================== FUNCTION-STATEMENTS ==========================

    override fun visitFn_decl(ctx: Basic2Parser.Fn_declContext) = visitFn_decl_stmt(ctx.fn_decl_stmt())

    override fun visitFn_decl_stmt(ctx: Basic2Parser.Fn_decl_stmtContext) = visitFnDeclStmt(ctx, getSymbolTable())

    override fun visitFn_impl(ctx: Basic2Parser.Fn_implContext) = visitFn_impl_stmt(ctx.fn_impl_stmt())

    override fun visitFn_impl_stmt(ctx: Basic2Parser.Fn_impl_stmtContext): Symbol.Var.Value.VUnit {
        val id = ctx.IDENTIFIER().text
        var argsParams = ctx.fn_param().map { visitFn_param(it) }
        val body = { args: List<Symbol.Var.Value?> ->
            args.forEachIndexed { i, arg -> arg?.let { argsParams[i].value = it } }
            runFnCall {
                argsParams.forEach { getSymbolTable().declAssVar(it.id, it.value!!) }
                var result: Symbol.Var.Value = Symbol.Var.Value.VUnit
                result = stmtCtx(ctx.stmt())
                argsParams.forEach { getSymbolTable().remVar(it.id) }
                result
            } as Symbol.Var.Value
        }
        getSymbolTable().addFnImpl(id, argsParams, body)
        return Symbol.Var.Value.VUnit
    }

    override fun visitFn_param(ctx: Basic2Parser.Fn_paramContext)
        = Symbol.Arg(ctx.IDENTIFIER().text, ctx.expr()?.let { exprCtx(it) })

    // ========================== TYPING-STATEMENTS ==========================

    override fun visitTyping(ctx: Basic2Parser.TypingContext): Symbol.Var.Type = visitType(ctx.type())

    // ========================== INCREMENT-STATEMENTS =========================

    override fun visitPreIncr(ctx: Basic2Parser.PreIncrContext): Symbol {
        return TODO()
    }

    override fun visitPostIncr(ctx: Basic2Parser.PostIncrContext): Symbol {
        return TODO()
    }

    override fun visitBinopIncr(ctx: Basic2Parser.BinopIncrContext): Symbol.Var.Value.VUnit {
        val id = ctx.IDENTIFIER().text
        var v = getSymbolTable().getVar(id)
        val i = exprCtx(ctx.expr())
        val newValue = binIncr(ctx.incr().text)(v, i)
        getSymbolTable().reAssVar(id, newValue)
        return Symbol.Var.Value.VUnit
    }
}
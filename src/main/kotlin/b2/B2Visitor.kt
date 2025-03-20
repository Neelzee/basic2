package b2

import no.nilsmf.antlr.Basic2BaseVisitor
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.tree.TerminalNode
import kotlin.math.max
import kotlin.math.min

sealed class B2Exception : Throwable() {
    data class ReturnException(val returnValue: Value) : B2Exception()
    data object BreakException : B2Exception() {
        private fun readResolve(): Any = BreakException
    }

    fun getValue() : Value = when (this) {
        is ReturnException -> returnValue
        is BreakException -> Value.VUnit
    }
}

class B2Visitor : Basic2BaseVisitor<Any?>() {

    private var symbolTable = SymbolTable()

    fun printSymbolTable() = symbolTable.print()

    override fun visitIdent(ctx: Basic2Parser.IdentContext): Value
        = symbolTable.getVar(ctx.IDENTIFIER().text).value

    override fun visitFloat(ctx: Basic2Parser.FloatContext): Value.VFloat {
        val rawTxt = ctx.FLOAT_LIT().text
        return Value.VFloat(rawTxt.replace("f", "").toFloat())
    }

    private fun numLit(i: TerminalNode): Value.VInt {
        val rawTxt = i.text
        return Value.VInt(rawTxt.replace("f", "").toInt())
    }

    override fun visitNum(ctx: Basic2Parser.NumContext): Value.VInt = numLit(ctx.NUM_LIT())

    override fun defaultResult(): Value.VUnit {
        return Value.VUnit
    }

    override fun visitStr(ctx: Basic2Parser.StrContext): Value.VString {
        val rawTxt = ctx.STR_LIT().text
        return Value.VString(rawTxt.replace("\"", "").replace("\"", ""))
    }

    override fun visitBool(ctx: Basic2Parser.BoolContext): Value.VBoolean {
        val rawTxt = ctx.BOOL_LIT().text
        return Value.VBoolean(rawTxt == "TRUE")
    }

    override fun visitTernary(ctx: Basic2Parser.TernaryContext): Value {
        val pred = exprCtx(ctx.expr(0)!!)
        val left = exprCtx(ctx.expr(1)!!)
        val right = exprCtx(ctx.expr(2)!!)
        return if (pred.toBool()) { left } else { right }
    }

    override fun visitTuple(ctx: Basic2Parser.TupleContext): Value.Tuple {
        val fst = super.visit(ctx.expr(0)!!) as Value
        val snd = super.visit(ctx.expr(1)!!) as Value
        return Value.Tuple(Pair(fst, snd), Type.Tuple(fst.type(), snd.type()))
    }

    override fun visitArray(ctx: Basic2Parser.ArrayContext): Value.VList
        = ctx.expr()
            .map { exprCtx(it) }
            .let { Value.VList(it.toMutableList(), Type.TList(Type.TUnit)) }

    override fun visitVar_decl(ctx: Basic2Parser.Var_declContext): Value.VUnit {
        val (id, type) = ctx.var_decl_stmt().let {
            Pair(it.IDENTIFIER().text, Type.from(it.typing().type().text))
        }
        symbolTable.declVar(id, type)

        return Value.VUnit
    }

    override fun visitArrReAss(ctx: Basic2Parser.ArrReAssContext): Value.VUnit {
        val atx = ctx.arr_re_ass_stmt()
        val arrVar = symbolTable.getVar(atx.IDENTIFIER().text)
        val ind = exprCtx(atx.expr(0)!!) as Value.VInt
        val newValue = exprCtx(atx.expr(1)!!)
        arrVar.value[ind] = newValue
        return Value.VUnit
    }

    override fun visitGroup(ctx: Basic2Parser.GroupContext): Value = exprCtx(ctx.expr())

    private fun exprCtx(ctx: Basic2Parser.ExprContext): Value = when (ctx) {
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
        else -> TODO(ctx.text)
    }

    private fun stmtCtx(ctx: Basic2Parser.StmtContext): Value = when (ctx) {
        is Basic2Parser.If_elseContext -> visitIf_else(ctx)
        is Basic2Parser.RetContext -> visitRet(ctx)
        is Basic2Parser.BlockContext -> visitBlock(ctx)
        is Basic2Parser.IfContext -> visitIf(ctx)
        is Basic2Parser.PrintContext -> visitPrint(ctx)
        is Basic2Parser.InputContext -> visitInput(ctx)
        is Basic2Parser.Var_assContext -> visitVar_ass(ctx)
        is Basic2Parser.Var_re_assContext -> visitVar_re_ass(ctx)
        is Basic2Parser.BreakContext -> visitBreak(ctx)
        is Basic2Parser.AppendContext -> visitAppend(ctx)
        is Basic2Parser.For_rContext -> visitFor_r(ctx)
        is Basic2Parser.ForContext -> visitFor(ctx)
        is Basic2Parser.If_blockContext -> visitIf_block(ctx)
        is Basic2Parser.While_blockContext -> visitWhile_block(ctx)
        is Basic2Parser.ArrReAssContext -> visitArrReAss(ctx)
        else -> TODO(ctx.text)
    }


    override fun visitVar_ass(ctx: Basic2Parser.Var_assContext): Value.VUnit {
        val dclCtx = ctx.var_decl_ass_stmt()
        val id = dclCtx.IDENTIFIER().text
        val type = dclCtx.typing()?.type()?.text?.let { Type.from(it) }
        val value = exprCtx(dclCtx.expr())
        if (value.type() != Type.TUnit) symbolTable.declAssVar(id, value, type)
        return Value.VUnit
    }

    override fun visitVar_re_ass(ctx: Basic2Parser.Var_re_assContext): Value.VUnit {
        val raCtx = ctx.var_re_ass_stmt()
        val id = raCtx.IDENTIFIER().text
        val value = exprCtx(raCtx.expr())
        symbolTable.reAssVar(id, value)
        return Value.VUnit
    }

    override fun visitPrint(ctx: Basic2Parser.PrintContext): Value.VUnit = visitPrint_stmt(ctx.print_stmt())

    override fun visitPrint_stmt(ctx: Basic2Parser.Print_stmtContext): Value.VUnit {
        val value = exprCtx(ctx.expr())
        println(value.value())

        return Value.VUnit
    }

    override fun visitInput(ctx: Basic2Parser.InputContext): Value.VUnit {
        val iCtx = ctx.input_stmt()
        iCtx.expr()?.let { print(exprCtx(it).value()) }
        val value = readlnOrNull() ?: ""
        iCtx.IDENTIFIER()?.let {
            val id = it.text
            val type = iCtx.typing()!!.type().text.let { t -> Type.from(t) }
            symbolTable.declAssVar(id, Value.VString(value), type)
        }
        return Value.VUnit
    }

    private fun binOpCtx(ctx: Basic2Parser.Bin_opContext): (Value, Value) -> Value = when (ctx) {
        is Basic2Parser.Incr_binContext -> incrBin(ctx.incr())
        is Basic2Parser.AddContext -> visitAdd(ctx)
        is Basic2Parser.SubContext -> visitSub(ctx)
        is Basic2Parser.DivContext -> visitDiv(ctx)
        is Basic2Parser.Comp_binContext -> comp(ctx.comp())
        is Basic2Parser.MulContext -> visitMul(ctx)
        is Basic2Parser.ModContext -> visitMod(ctx)
        else -> TODO("Bin-op: ${ctx.text}")
    }

    private fun comp(ctx: Basic2Parser.CompContext): (Value, Value) -> Value = when (ctx) {
        is Basic2Parser.EqContext -> visitEq(ctx)
        is Basic2Parser.LteqContext -> visitLteq(ctx)
        is Basic2Parser.GteqContext -> visitGteq(ctx)
        is Basic2Parser.GtContext -> visitGt(ctx)
        else -> TODO("comp: ${ctx.text}")
    }

    private fun incrBin(ctx: Basic2Parser.IncrContext): (Value, Value) -> Value = when (ctx) {
        is Basic2Parser.MutAddContext -> visitMutAdd(ctx)
        else -> TODO("incrBin")
    }

    override fun visitBinop(ctx: Basic2Parser.BinopContext): Value {
        val left = ctx.expr(0)?.let { exprCtx(it) }!!
        val right = ctx.expr(1)?.let { exprCtx(it) }!!

        return binOpCtx(ctx.bin_op())(left, right)
    }

    override fun visitAdd(ctx: Basic2Parser.AddContext): (Value, Value) -> Value = { a, b -> a + b }

    override fun visitSub(ctx: Basic2Parser.SubContext): (Value, Value) -> Value = { a, b -> a - b }

    override fun visitDiv(ctx: Basic2Parser.DivContext): (Value, Value) -> Value = { a, b -> a / b }

    override fun visitMul(ctx: Basic2Parser.MulContext): (Value, Value) -> Value = { a, b -> a * b }

    override fun visitMod(ctx: Basic2Parser.ModContext): (Value, Value) -> Value = { a, b -> a % b }

    override fun visitPow(ctx: Basic2Parser.PowContext): (Value, Value) -> Value = { a, b -> a.pow(b) }

    override fun visitAnd(ctx: Basic2Parser.AndContext): (Value, Value) -> Value = { a, b -> a.and(b) }

    override fun visitOr(ctx: Basic2Parser.OrContext): (Value, Value) -> Value = { a, b -> a.or(b) }

    override fun visitMutAdd(ctx: Basic2Parser.MutAddContext): (Value, Value) -> Value.VUnit {
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value + right
            symbolTable.reAssVar(id, lv)
        }
        return { _, _ -> Value.VUnit }
    }

    override fun visitMutSub(ctx: Basic2Parser.MutSubContext): (Value, Value) -> Value.VUnit {
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value - right
            symbolTable.reAssVar(id, lv)
        }
        return { _, _ -> Value.VUnit }
    }

    override fun visitMutDiv(ctx: Basic2Parser.MutDivContext): (Value, Value) -> Value.VUnit {
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value / right
            symbolTable.reAssVar(id, lv)
        }
        return { _, _ -> Value.VUnit }
    }

    override fun visitMutMul(ctx: Basic2Parser.MutMulContext):(Value, Value) -> Value {
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value * right
            symbolTable.reAssVar(id, lv)
        }
        return { _, _ -> Value.VUnit }
    }

    override fun visitMutMod(ctx: Basic2Parser.MutModContext): (Value, Value) -> Value.VUnit {
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value % right
            symbolTable.reAssVar(id, lv)
        }
        return { _, _ -> Value.VUnit }
    }

    override fun visitMutPow(ctx: Basic2Parser.MutPowContext): (Value, Value) -> Value.VUnit {
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value.pow(right)
            symbolTable.reAssVar(id, lv)
        }
        return { _, _ -> Value.VUnit }
    }

    override fun visitMutAnd(ctx: Basic2Parser.MutAndContext): (Value, Value) -> Value.VUnit {
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value.and(right)
            symbolTable.reAssVar(id, lv)
        }
        return { _, _ -> Value.VUnit }
    }

    override fun visitMutOr(ctx: Basic2Parser.MutOrContext): (Value, Value) -> Value.VUnit {
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value.or(right)
            symbolTable.reAssVar(id, lv)
        }
        return { _, _ -> Value.VUnit }
    }

    override fun visitEq(ctx: Basic2Parser.EqContext): (Value, Value) -> Value.VBoolean
            = { a, b -> Value.VBoolean(a == b) }

    override fun visitNeq(ctx: Basic2Parser.NeqContext): (Value, Value) -> Value.VBoolean
            = { a, b -> Value.VBoolean(a != b) }


    override fun visitGteq(ctx: Basic2Parser.GteqContext): (Value, Value) -> Value.VBoolean
            = { a, b -> Value.VBoolean(a <= b) }

    override fun visitGt(ctx: Basic2Parser.GtContext): (Value, Value) -> Value
            = { a, b -> Value.VBoolean(a < b) }

    override fun visitLteq(ctx: Basic2Parser.LteqContext): (Value, Value) -> Value
            = { a, b -> Value.VBoolean(a >= b) }

    override fun visitLt(ctx: Basic2Parser.LtContext): (Value, Value) -> Value
            = { a, b -> Value.VBoolean(a > b) }

    override fun visitNeg(ctx: Basic2Parser.NegContext): Value
        = ctx.children?.get(1)?.let { Value.from(super.visit(it)).not() }!!

    override fun visitSig(ctx: Basic2Parser.SigContext): Value
        = ctx.children?.get(1)?.let { Value.from(super.visit(it)).sig() }!!


    override fun visitInc(ctx: Basic2Parser.IncContext): Value
        = ctx.children?.get(1)?.let { Value.from(super.visit(it)).inc() }!!


    override fun visitDec(ctx: Basic2Parser.DecContext): Value
        = ctx.children?.get(1)?.let { Value.from(super.visit(it)).dec() }!!

    override fun visitFn_decl(ctx: Basic2Parser.Fn_declContext): Value.VUnit {
        val fnCtx = ctx.fn_decl_stmt()
        val id = fnCtx.IDENTIFIER().text
        val types: List<Type> = fnCtx.type().map { super.visit(it) as Type }

        symbolTable.addFnDecl(id, types.subList(0, types.size - 1), types[types.size - 1])

        return Value.VUnit
    }

    override fun visitFnCall(ctx: Basic2Parser.FnCallContext): Value {
        val fn = ctx.IDENTIFIER().text
        val args = ctx.expr().map { exprCtx(it) }
        val fnBody = symbolTable.getImpl(fn)
        return try {
            fnBody.body(args.toMutableList())
        } catch (e: B2Exception) {
            e.getValue()
        }
    }

    override fun visitType(ctx: Basic2Parser.TypeContext): Type
        = ctx.PRIM_TYPES()?.text?.let { Type.from(it) }
        ?: ctx.type().let {
            when {
                (ctx.TUPLE_STRT() != null && it.size == 2) ->
                    Type.Tuple(super.visit(it[0]) as Type, super.visit(it[1]) as Type)
                (ctx.ARRAY_STRT() != null) -> Type.TList(super.visit(it[0]) as Type)
                else -> throw IllegalStateException("This should not happen")
            }
        }

    override fun visitFn_param(ctx: Basic2Parser.Fn_paramContext): Symbol.Arg {
        val id = ctx.IDENTIFIER().text
        val value: Value? = ctx.expr()?.children?.get(0)?.let { super.visit(it) } as Value?
        return Symbol.Arg(id, value)
    }

    override fun visitFn_impl(ctx: Basic2Parser.Fn_implContext): Value.VUnit {
        val fnCtx = ctx.fn_impl_stmt()
        val id = fnCtx.IDENTIFIER().text
        val paramIds = fnCtx.fn_param().map { super.visit(it) as Symbol.Arg }
        val paramTypes = symbolTable.getParams(id)
        val pairs = paramTypes.zip(paramIds)
        val fnBody: (List<Value?>) -> Value = { args ->
            symbolTable = symbolTable.enterScope()
            if (args.isNotEmpty()) {
                for (i in 0..<min(args.size, pairs.size)) {
                    val value = args[i]
                    val (typeInfo, optArg) = pairs[i]
                    val v = value ?: optArg.value ?: throw RuntimeException("$id has no returnValue")
                    symbolTable.declAssVar(optArg.id, v, typeInfo.type)
                }
            }
            val result = try {
                stmtCtx(fnCtx.stmt())
            } catch (e: B2Exception) {
                e.getValue()
            } finally {
                symbolTable = symbolTable.exitScope()
            }
            result
        }
        symbolTable.addFnImpl(id, paramIds, fnBody)
        return Value.VUnit
    }

    override fun visitTrim(ctx: Basic2Parser.TrimContext): Value.VString = when (val s = exprCtx(ctx.expr())) {
        is Value.VString -> Value.VString(s.value.trim())
        else -> throw RuntimeException("Cannot trim $s")
    }

    override fun visitIf_else(ctx: Basic2Parser.If_elseContext): Value {
        val ifCtx = ctx.if_else_stmt()
        val pred = exprCtx(ifCtx.expr()).toBool()
        val ind = if (pred) { 0 } else { 1 }
        return withScope {
            ifCtx.stmt(ind)?.let { stmtCtx(it) } ?: Value.VUnit
        }
    }

    private fun withScope(block: () -> Value): Value = try {
        symbolTable.enterScope()
        block()
    } catch (e: B2Exception) {
        symbolTable.enterScope()
        throw e
    }

    private fun inLoop(block: () -> Value): Value = try {
        block()
    } catch (e: B2Exception) {
        e.getValue()
    }

    override fun visitArrInd(ctx: Basic2Parser.ArrIndContext): Value {
        val arr = when (val arr = exprCtx(ctx.expr(0)!!)) {
            is Value.VList -> arr
            is Value.Tuple -> arr
            else -> throw RuntimeException("Cannot index on non-array or non-tuple: $arr")
        }

        val ind = when (val ind = exprCtx(ctx.expr(1)!!)) {
            is Value.VInt -> ind
            else -> throw RuntimeException("Cannot index with non-integer: $ind")
        }

        return arr[ind]
    }

    override fun visitLen(ctx: Basic2Parser.LenContext): Value.VUnit {
        val ltx = ctx.len_stmt()
        val value = exprCtx(ltx.expr()).size()
        ltx.IDENTIFIER()?.let {
            val id = it.text
            val type = ltx.typing()!!.type().text.let { t -> Type.from(t) }
            symbolTable.declAssVar(id, Value.VInt(value), type)
        }
        return Value.VUnit
    }

    override fun visitAppend(ctx: Basic2Parser.AppendContext): Value.VUnit {
        val atx = ctx.append_stmt()

        val id = atx.IDENTIFIER().text
        val arr = symbolTable.getVar(id)
        val el = exprCtx(atx.expr())
        arr.value.add(el)

        return Value.VUnit
    }

    override fun visitIf(ctx: Basic2Parser.IfContext): Value {
        val ifCtx = ctx.if_stmt()
        val pred = exprCtx(ifCtx.expr()).toBool()
        if (pred) {
            return withScope {
                stmtCtx(ifCtx.stmt())
            }
        }
        return Value.VUnit
    }

    override fun visitIf_else_block(ctx: Basic2Parser.If_else_blockContext): Value {
        val ifCtx = ctx.if_else_stmt_block()
        val pred = (super.visit(ifCtx.expr()) as Value).toBool()
        val ind = if (pred) { 0 } else { 1 }
        return ifCtx.block_stmt(ind)?.let { stmtLst(it.stmt()) } ?: Value.VUnit
    }

    private fun stmtLst(stmt: List<Basic2Parser.StmtContext>): Value = withScope {
        var result: Value = Value.VUnit
        stmt.forEach { result = stmtCtx(it) }
        result
    }

    override fun visitBlock(ctx: Basic2Parser.BlockContext): Value = visitBlock_stmt(ctx.block_stmt())

    override fun visitBlock_stmt(ctx: Basic2Parser.Block_stmtContext): Value = stmtLst(ctx.stmt())

    override fun visitIf_block(ctx: Basic2Parser.If_blockContext): Value = withScope {
        val ifCtx = ctx.if_stmt_block()
        val pred = exprCtx(ifCtx.expr()).toBool()
        if (pred) {
            ifCtx.block_stmt().let { super.visit(it) }
        }
        Value.VUnit
    }

    override fun visitWhile(ctx: Basic2Parser.WhileContext): Value = inLoop {
        withScope {
            val wtx = ctx.while_stmt()
            var pred = exprCtx(wtx.expr()).toBool()
            while (pred) {
                stmtCtx(wtx.stmt())
                pred = exprCtx(wtx.expr()).toBool()
            }
            Value.VUnit
        }
    }

    override fun visitWhile_block(ctx: Basic2Parser.While_blockContext): Value = inLoop {
        withScope {
            val wtx = ctx.while_stmt_block()
            var pred = exprCtx(wtx.expr()).toBool()
            var value: Value = Value.VUnit
            while (pred) {
                value = stmtLst(wtx.block_stmt().stmt())
                pred = exprCtx(wtx.expr()).toBool()
            }
            value
        }
    }

    override fun visitFor_r(ctx: Basic2Parser.For_rContext): Value = inLoop {
        withScope {
            val ftx = ctx.for_range()
            val i = ftx.IDENTIFIER().text
            val iter = (super.visit(ftx.iterable()) as Value).toIterable().toList()
            if (iter.isEmpty()) return@withScope Value.VUnit
            symbolTable.declVar(i, Type.infer(iter[0]))
            var value: Value = Value.VUnit
            for (j in iter) {
                symbolTable.reAssVar(i, Value.from(j))
                val (brk, v) = stmtBody(ftx.stmt())
                value = v
                if (brk) break
            }
            value
        }
    }

    private fun stmtBody(ctx: Basic2Parser.StmtContext): Pair<Boolean, Value> {
        var brk = false
        var value: Value = Value.VUnit
        try {
            stmtCtx(ctx)
        } catch (e: B2Exception) {
            brk = true
            value = e.getValue()
        } catch (e: Exception) {
            throw e
        }
        return Pair(brk, value)
    }

    override fun visitFor(ctx: Basic2Parser.ForContext): Value = inLoop {
        withScope {
            val ftx = ctx.for_stmt()
            val i = ftx.IDENTIFIER(0)!!.text
            val iStart = exprCtx(ftx.expr(0)!!)
            symbolTable.declAssVar(i, iStart)
            val p = ftx.IDENTIFIER(1)!!.text
            val j = ftx.IDENTIFIER(2)!!.text
            @Suppress("UNCHECKED_CAST")
            val pred: (Value, Value) -> Value = super.visit(ftx.comp()) as (Value, Value) -> Value
            val comp: () -> Value = { super.visit(ftx.expr(1)!!) as Value }
            val inp: () -> Value = { symbolTable.getVar(p).value }
            val inc: (String, Value) -> Unit = ftx.incr()?.let {
                { id: String, v: Value ->
                    val expr = super.visit(ftx.expr(2)!!) as Value
                    val new = when (it.text) {
                        "+=" -> v + expr
                        else -> TODO("Unimplemented incr")
                    }
                    symbolTable.reAssVar(id, new)
                }
            } ?: ftx.incr_uni()!!.let {
                { id: String, v: Value ->
                    val new = when (it.text) {
                        "++" -> v.inc()
                        else -> TODO("Unimplemented incr")
                    }
                    symbolTable.reAssVar(id, new)
                }
            }
            var value: Value = Value.VUnit
            while (pred(inp(), comp()).toBool()) {
                val (brk, v) = stmtBody(ftx.stmt())
                value = v
                if (brk) break
                inc(j, symbolTable.getVar(j).value)
            }

            value
        }
    }

    override fun visitReturn_stmt(ctx: Basic2Parser.Return_stmtContext)
        = throw B2Exception.ReturnException(ctx.expr()?.let { exprCtx(it) } ?: Value.VUnit)

    override fun visitRet(ctx: Basic2Parser.RetContext) = visitReturn_stmt(ctx.return_stmt())

    override fun visitIterable(ctx: Basic2Parser.IterableContext): Value.VList
            = when (val lst = ctx.expr()?.let { exprCtx(it) }) {
        is Value.VList -> lst
        else -> {
            val start = (super.visit(ctx.NUM_LIT(0)!!) as Value.VInt).value
            val end = (super.visit(ctx.NUM_LIT(1)!!) as Value.VInt).value
            Value.VList((start..end).map { Value.VInt(it) }.toMutableList(), Type.TList(Type.TInt))
        }
    }

    override fun visitBreak_stmt(ctx: Basic2Parser.Break_stmtContext) = throw B2Exception.BreakException

    override fun visitBreak(ctx: Basic2Parser.BreakContext) = visitBreak_stmt(ctx.break_stmt())

    override fun visitCast(ctx: Basic2Parser.CastContext): Value
        = Value.withType(exprCtx(ctx.expr()), visitType(ctx.type()))
}
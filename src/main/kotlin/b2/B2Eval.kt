package b2

import b2.symbols.Symbol
import b2.symbols.Type
import no.nilsmf.antlr.Basic2BaseVisitor
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.tree.TerminalNode

open class B2Eval(private var symbolTable: SymbolTable = SymbolTable()) : Basic2BaseVisitor<Symbol>() {

    fun getSymbolTable() = symbolTable

    fun printSymbolTable() = symbolTable.print()

    /**
     * Runs the given statements in another scope, if a return or break happens,
     * the scope is exited, and the exception is thrown further up
     *
     * Used for if-statements, block-statements, etc.
     */
    fun runScope(f: () -> Symbol): Symbol = try {
        symbolTable = symbolTable.enterScope()
        f()
    } catch (e: B2Exception.ReturnException) {
        symbolTable = symbolTable.exitScope()
        throw e
    } catch (e: B2Exception.BreakException) {
        symbolTable = symbolTable.exitScope()
        throw e
    }

    /**
     * Runs the given statements in another scope, if a return or break happens,
     * the scope is exited, and the exception is thrown further up, if it is a return.
     * If not, the break stops at here.
     *
     * Used for for-statements, while-statements, etc.
     */
    fun runLoop(f: () -> Symbol): Symbol = try {
        symbolTable = symbolTable.enterScope()
        f()
    } catch (e: B2Exception.ReturnException) {
        symbolTable = symbolTable.exitScope()
        throw e
    } catch (_: B2Exception.BreakException) {
        symbolTable = symbolTable.exitScope()
        Symbol.Var(Value.VUnit)
    }

    /**
     * Runs the given statements in another scope, if a return or break happens,
     * the scope is exited, and the exception is thrown further up, if it is a break.
     * If not, the return stops at here.
     *
     * Used for function calls
     */
    fun runFnCall(f: () -> Symbol): Symbol = try {
        symbolTable = symbolTable.enterScope()
        f()
    } catch (e: B2Exception.ReturnException) {
        symbolTable = symbolTable.exitScope()
        e.getSymbol()
    } catch (e: B2Exception.BreakException) {
        symbolTable = symbolTable.exitScope()
        throw e
    }


    override fun visitGroup(ctx: Basic2Parser.GroupContext): Symbol.Var = exprCtx(ctx.expr())

    override fun defaultResult(): Symbol.Var = Symbol.Var.VUnit

    override fun visitIdent(ctx: Basic2Parser.IdentContext): Symbol.Var
            = symbolTable.getVar(ctx.IDENTIFIER().text)

    override fun visitFloat(ctx: Basic2Parser.FloatContext): Symbol.Var.VFloat {
        val rawTxt = ctx.FLOAT_LIT().text
        return Symbol.Var.VFloat(rawTxt.replace("f", "").toFloat())
    }

    private fun numLit(i: TerminalNode): Symbol.Var.VInt {
        val rawTxt = i.text
        return Symbol.Var.VInt(rawTxt.replace("f", "").toInt())
    }

    override fun visitNum(ctx: Basic2Parser.NumContext): Symbol.Var.VInt = numLit(ctx.NUM_LIT())

    override fun visitStr(ctx: Basic2Parser.StrContext): Symbol.Var.VString {
        val rawTxt = ctx.STR_LIT().text
        return Symbol.Var.VString(rawTxt.replace("\"", "").replace("\"", ""))
    }

    override fun visitBool(ctx: Basic2Parser.BoolContext): Symbol.Var.VBoolean {
        val rawTxt = ctx.BOOL_LIT().text
        return Symbol.Var.VBoolean(rawTxt == "TRUE")
    }

    override fun visitTernary(ctx: Basic2Parser.TernaryContext): Symbol.Var {
        val pred = exprCtx(ctx.expr(0)!!)
        val left = exprCtx(ctx.expr(1)!!)
        val right = exprCtx(ctx.expr(2)!!)
        return if (pred.toBool()) { left } else { right }
    }

    override fun visitTuple(ctx: Basic2Parser.TupleContext): Symbol.Var.Tuple {
        val fst = exprCtx(ctx.expr(0)!!)
        val snd = exprCtx(ctx.expr(1)!!)
        return Symbol.Var.Tuple(Pair(fst, snd), Type.Tuple(fst.type(), snd.type()))
    }

    override fun visitArray(ctx: Basic2Parser.ArrayContext): Symbol.Var.VList
            = ctx.expr()
        .map { exprCtx(it) }
        .let { Symbol.Var.VList(it.toMutableList(), Type.TList(Type.TUnit)) }

    override fun visitArrInd(ctx: Basic2Parser.ArrIndContext): Symbol.Var {
        val arr = when (val arr = exprCtx(ctx.expr(0)!!)) {
            is Symbol.Var.VList -> arr
            is Symbol.Var.Tuple -> arr
            else -> throw RuntimeException("Cannot index on non-array or non-tuple: $arr")
        }

        val ind = when (val ind = exprCtx(ctx.expr(1)!!)) {
            is Symbol.Var.VInt -> ind
            else -> throw RuntimeException("Cannot index with non-integer: $ind")
        }

        return arr[ind]
    }


    override fun visitFnCall(ctx: Basic2Parser.FnCallContext): Symbol.Var {
        val fn = ctx.IDENTIFIER().text
        val args = ctx.expr().map { exprCtx(it) }
        val fnBody = getSymbolTable().getImpl(fn)
        return try {
            fnBody.body(args.toList())
        } catch (e: B2Exception) {
            e.getSymbol()
        }
    }

    private fun binOpCtx(ctx: Basic2Parser.Bin_opContext): Symbol.Var = when (ctx) {
        is Basic2Parser.Incr_binContext -> incrBin(ctx.incr())
        is Basic2Parser.AddContext -> visitAdd(ctx)
        is Basic2Parser.SubContext -> visitSub(ctx)
        is Basic2Parser.DivContext -> visitDiv(ctx)
        is Basic2Parser.Comp_binContext -> comp(ctx.comp())
        is Basic2Parser.MulContext -> visitMul(ctx)
        is Basic2Parser.ModContext -> visitMod(ctx)
        else -> TODO("Bin-op: ${ctx.text}")
    }

    private fun comp(ctx: Basic2Parser.CompContext): (Symbol.Var, Symbol.Var) -> Symbol.Var = when (ctx) {
        is Basic2Parser.EqContext -> visitEq(ctx)
        is Basic2Parser.LteqContext -> visitLteq(ctx)
        is Basic2Parser.GteqContext -> visitGteq(ctx)
        is Basic2Parser.GtContext -> visitGt(ctx)
        else -> TODO("comp: ${ctx.text}")
    }

    private fun incrBin(ctx: Basic2Parser.IncrContext): Symbol.Var = when (ctx) {
        is Basic2Parser.MutAddContext -> visitMutAdd(ctx)
        else -> TODO("incrBin: ${ctx.text}")
    }

    override fun visitBinop(ctx: Basic2Parser.BinopContext): Symbol.Var {
        return binOpCtx(ctx.bin_op())
    }

    override fun visitAdd(ctx: Basic2Parser.AddContext): (Symbol.Var, Symbol.Var) -> Symbol.Var = { a, b -> a + b }

    override fun visitSub(ctx: Basic2Parser.SubContext): (Symbol.Var, Symbol.Var) -> Symbol.Var = { a, b -> a - b }

    override fun visitDiv(ctx: Basic2Parser.DivContext): (Symbol.Var, Symbol.Var) -> Symbol.Var = { a, b -> a / b }

    override fun visitMul(ctx: Basic2Parser.MulContext): (Symbol.Var, Symbol.Var) -> Symbol.Var = { a, b -> a * b }

    override fun visitMod(ctx: Basic2Parser.ModContext): (Symbol.Var, Symbol.Var) -> Symbol.Var = { a, b -> a % b }

    override fun visitPow(ctx: Basic2Parser.PowContext): (Symbol.Var, Symbol.Var) -> Symbol.Var = { a, b -> a.pow(b) }

    override fun visitAnd(ctx: Basic2Parser.AndContext): (Symbol.Var, Symbol.Var) -> Symbol.Var = { a, b -> a.and(b) }

    override fun visitOr(ctx: Basic2Parser.OrContext): (Symbol.Var, Symbol.Var) -> Symbol.Var = { a, b -> a.or(b) }

    override fun visitMutAdd(ctx: Basic2Parser.MutAddContext): Symbol.Var.VUnit {
        val id = ctx.children?.get(0)?.text ?: ""
        getSymbolTable().getVarNullable(id)?.let {
            val right = Symbol.Var.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value + right
            getSymbolTable().reAssVar(id, lv)
        }
        return { _, _ -> Symbol.Var.VUnit }
    }

    override fun visitMutSub(ctx: Basic2Parser.MutSubContext): (Symbol.Var, Symbol.Var) -> Symbol.Var.VUnit {
        val id = ctx.children?.get(0)?.text ?: ""
        getSymbolTable().getVarNullable(id)?.let {
            val right = Symbol.Var.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value - right
            getSymbolTable().reAssVar(id, lv)
        }
        return { _, _ -> Symbol.Var.VUnit }
    }

    override fun visitMutDiv(ctx: Basic2Parser.MutDivContext): (Symbol.Var, Symbol.Var) -> Symbol.Var.VUnit {
        val id = ctx.children?.get(0)?.text ?: ""
        getSymbolTable().getVarNullable(id)?.let {
            val right = Symbol.Var.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value / right
            getSymbolTable().reAssVar(id, lv)
        }
        return { _, _ -> Symbol.Var.VUnit }
    }

    override fun visitMutMul(ctx: Basic2Parser.MutMulContext):(Symbol.Var, Symbol.Var) -> Symbol.Var {
        val id = ctx.children?.get(0)?.text ?: ""
        getSymbolTable().getVarNullable(id)?.let {
            val right = Symbol.Var.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value * right
            getSymbolTable().reAssVar(id, lv)
        }
        return { _, _ -> Symbol.Var.VUnit }
    }

    override fun visitMutMod(ctx: Basic2Parser.MutModContext): (Symbol.Var, Symbol.Var) -> Symbol.Var.VUnit {
        val id = ctx.children?.get(0)?.text ?: ""
        getSymbolTable().getVarNullable(id)?.let {
            val right = Symbol.Var.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value % right
            getSymbolTable().reAssVar(id, lv)
        }
        return { _, _ -> Symbol.Var.VUnit }
    }

    override fun visitMutPow(ctx: Basic2Parser.MutPowContext): (Symbol.Var, Symbol.Var) -> Symbol.Var.VUnit {
        val id = ctx.children?.get(0)?.text ?: ""
        getSymbolTable().getVarNullable(id)?.let {
            val right = Symbol.Var.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value.pow(right)
            getSymbolTable().reAssVar(id, lv)
        }
        return { _, _ -> Symbol.Var.VUnit }
    }

    override fun visitMutAnd(ctx: Basic2Parser.MutAndContext): (Symbol.Var, Symbol.Var) -> Symbol.Var.VUnit {
        val id = ctx.children?.get(0)?.text ?: ""
        getSymbolTable().getVarNullable(id)?.let {
            val right = Symbol.Var.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value.and(right)
            getSymbolTable().reAssVar(id, lv)
        }
        return { _, _ -> Symbol.Var.VUnit }
    }

    override fun visitMutOr(ctx: Basic2Parser.MutOrContext): (Symbol.Var, Symbol.Var) -> Symbol.Var.VUnit {
        val id = ctx.children?.get(0)?.text ?: ""
        getSymbolTable().getVarNullable(id)?.let {
            val right = Symbol.Var.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value.or(right)
            getSymbolTable().reAssVar(id, lv)
        }
        return { _, _ -> Symbol.Var.VUnit }
    }

    override fun visitEq(ctx: Basic2Parser.EqContext): (Symbol.Var, Symbol.Var) -> Symbol.Var.VBoolean
            = { a, b -> Symbol.Var.VBoolean(a == b) }

    override fun visitNeq(ctx: Basic2Parser.NeqContext): (Symbol.Var, Symbol.Var) -> Symbol.Var.VBoolean
            = { a, b -> Symbol.Var.VBoolean(a != b) }


    override fun visitGteq(ctx: Basic2Parser.GteqContext): (Symbol.Var, Symbol.Var) -> Symbol.Var.VBoolean
            = { a, b -> Symbol.Var.VBoolean(a <= b) }

    override fun visitGt(ctx: Basic2Parser.GtContext): (Symbol.Var, Symbol.Var) -> Symbol.Var
            = { a, b -> Symbol.Var.VBoolean(a < b) }

    override fun visitLteq(ctx: Basic2Parser.LteqContext): (Symbol.Var, Symbol.Var) -> Symbol.Var.VBoolean
            = { a, b -> Symbol.Var.VBoolean(a >= b) }

    override fun visitLt(ctx: Basic2Parser.LtContext): (Symbol.Var, Symbol.Var) -> Symbol.Var.VBoolean
            = { a, b -> Symbol.Var.VBoolean(a > b) }

    override fun visitNeg(ctx: Basic2Parser.NegContext): Symbol.Var
            = ctx.children?.get(1)?.let { Symbol.Var.from(super.visit(it)).not() }!!

    override fun visitSig(ctx: Basic2Parser.SigContext): Symbol.Var
            = ctx.children?.get(1)?.let { Symbol.Var.from(super.visit(it)).sig() }!!


    override fun visitInc(ctx: Basic2Parser.IncContext): Symbol.Var
            = ctx.children?.get(1)?.let { Symbol.Var.from(super.visit(it)).inc() }!!


    override fun visitDec(ctx: Basic2Parser.DecContext): Symbol.Var
            = ctx.children?.get(1)?.let { Symbol.Var.from(super.visit(it)).dec() }!!

    private fun exprCtx(ctx: Basic2Parser.ExprContext): Symbol.Var = when (ctx) {
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
        else -> TODO("Not implemented for: ${ctx.text}")
    }
}
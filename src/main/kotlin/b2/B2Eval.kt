package b2

import b2.symbols.Symbol
import no.nilsmf.antlr.Basic2BaseVisitor
import no.nilsmf.antlr.Basic2Parser
import org.antlr.v4.kotlinruntime.tree.TerminalNode
import kotlin.text.trim

open class B2Eval : B2() {
    companion object {
        fun getCompOp(kind: String): (Symbol.Var.Value, Symbol.Var.Value) -> Boolean = when (kind) {
            "==" -> { a, b -> a == b }
            "!=" -> { a, b -> a != b }
            ">" -> { a, b -> a > b }
            "<=" -> { a, b -> a <= b }
            else -> TODO("Missing op: $kind")
        }

        fun visitType(ctx: Basic2Parser.TypeContext): Symbol.Var.Type {
            return ctx.PRIM_TYPES()?.let { Symbol.Var.Type.from(it.text) } ?: if (ctx.ARRAY_STRT() != null) {
                Symbol.Var.Type.TList(visitType(ctx.type(0)!!))
            } else {
                Symbol.Var.Type.Tuple(visitType(ctx.type(0)!!), visitType(ctx.type(1)!!))
            }
        }
    }

    override fun visitGroup(ctx: Basic2Parser.GroupContext): Symbol.Var.Value = exprCtx(ctx.expr())

    override fun visitIdent(ctx: Basic2Parser.IdentContext): Symbol.Var
            = getSymbolTable().getVar(ctx.IDENTIFIER().text)

    override fun visitFloat(ctx: Basic2Parser.FloatContext): Symbol.Var.Value.VFloat {
        val rawTxt = ctx.FLOAT_LIT().text
        return Symbol.Var.Value.VFloat(rawTxt.replace("f", "").toFloat())
    }

    private fun numLit(i: TerminalNode): Symbol.Var.Value.VInt {
        val rawTxt = i.text
        return Symbol.Var.Value.VInt(rawTxt.replace("f", "").toInt())
    }

    override fun visitNum(ctx: Basic2Parser.NumContext): Symbol.Var.Value.VInt = numLit(ctx.NUM_LIT())

    override fun visitStr(ctx: Basic2Parser.StrContext): Symbol.Var.Value.VString {
        val rawTxt = ctx.STR_LIT().text
        return Symbol.Var.Value.VString(rawTxt.replace("\"", "").replace("\"", ""))
    }

    override fun visitBool(ctx: Basic2Parser.BoolContext): Symbol.Var.Value.VBoolean {
        val rawTxt = ctx.BOOL_LIT().text
        return Symbol.Var.Value.VBoolean(rawTxt == "TRUE")
    }

    override fun visitTernary(ctx: Basic2Parser.TernaryContext): Symbol.Var.Value {
        val pred = exprCtx(ctx.expr(0)!!)
        val left = exprCtx(ctx.expr(1)!!)
        val right = exprCtx(ctx.expr(2)!!)
        return if (pred.toBool()) { left } else { right }
    }

    override fun visitTuple(ctx: Basic2Parser.TupleContext): Symbol.Var.Value.Tuple {
        val fst = exprCtx(ctx.expr(0)!!)
        val snd = exprCtx(ctx.expr(1)!!)
        return Symbol.Var.Value.Tuple(Pair(fst, snd), Symbol.Var.Type.Tuple(fst.type(), snd.type()))
    }

    override fun visitArray(ctx: Basic2Parser.ArrayContext): Symbol.Var.Value.VList
            = ctx.expr()
        .map { exprCtx(it) }
        .let { Symbol.Var.Value.VList(it.toMutableList(), Symbol.Var.Type.TList(Symbol.Var.Type.TUnit)) }

    override fun visitArrInd(ctx: Basic2Parser.ArrIndContext): Symbol.Var.Value {
        val arr = when (val arr = exprCtx(ctx.expr(0)!!)) {
            is Symbol.Var.Value.VList -> arr
            is Symbol.Var.Value.Tuple -> arr
            else -> throw RuntimeException("Cannot index on non-array or non-tuple: $arr")
        }

        val ind = when (val ind = exprCtx(ctx.expr(1)!!)) {
            is Symbol.Var.Value.VInt -> ind
            else -> throw RuntimeException("Cannot index with non-integer: $ind")
        }

        return arr[ind]
    }

    override fun visitFnCall(ctx: Basic2Parser.FnCallContext): Symbol.Var.Value {
        val fn = ctx.IDENTIFIER().text
        val args = ctx.expr().map { exprCtx(it) }
        val fnBody = getSymbolTable().getImpl(fn)
        return try {
            fnBody.body(args.toList())
        } catch (e: B2Exception) {
            e.getSymbol()
        }
    }

    override fun visitBinop(ctx: Basic2Parser.BinopContext): Symbol.Var.Value {
        val left = exprCtx(ctx.expr(0)!!)
        val right = exprCtx(ctx.expr(1)!!)
        return when (ctx.bin_op().text) {
            "+" -> left + right
            "-" -> left - right
            "*" -> left * right
            "%" -> left % right
            else -> TODO("Missing operator: ${ctx.bin_op().text}")
        }
    }

    override fun visitTrim(ctx: Basic2Parser.TrimContext): Symbol.Var.Value.VString = when (val s = exprCtx(ctx.expr())) {
        is Symbol.Var.Value.VString -> Symbol.Var.Value.VString(s.value.trim())
        else -> throw RuntimeException("Cannot trim $s")
    }

    fun exprCtxVar(ctx: Basic2Parser.ExprContext): Symbol.Var = when (ctx) {
        is Basic2Parser.IdentContext -> visitIdent(ctx)
        else -> exprCtx(ctx)
    }

    fun exprCtx(ctx: Basic2Parser.ExprContext): Symbol.Var.Value = when (ctx) {
        is Basic2Parser.NumContext -> visitNum(ctx)
        is Basic2Parser.FloatContext -> visitFloat(ctx)
        is Basic2Parser.BoolContext -> visitBool(ctx)
        is Basic2Parser.TupleContext -> visitTuple(ctx)
        is Basic2Parser.FnCallContext -> visitFnCall(ctx)
        is Basic2Parser.BinopContext -> visitBinop(ctx)
        is Basic2Parser.IdentContext -> when (val v = visitIdent(ctx)) {
            is Symbol.Var.Variable -> v.value
            is Symbol.Var.Value -> v
            else -> throw RuntimeException("??: ${ctx.text}")
        }
        is Basic2Parser.StrContext -> visitStr(ctx)
        is Basic2Parser.TrimContext -> visitTrim(ctx)
        is Basic2Parser.ArrayContext -> visitArray(ctx)
        is Basic2Parser.ArrIndContext -> visitArrInd(ctx)
        is Basic2Parser.GroupContext -> visitGroup(ctx)
        is Basic2Parser.BinopCompContext -> visitBinopComp(ctx)
        is Basic2Parser.CastContext -> visitCast(ctx)
        else -> TODO("Not implemented for: ${ctx.text}")
    }

    override fun visitBinopComp(ctx: Basic2Parser.BinopCompContext): Symbol.Var.Value.VBoolean {
        val left = exprCtx(ctx.expr(0)!!)
        val right = exprCtx(ctx.expr(1)!!)
        return run {
            val res = getCompOp(ctx.comp().text)(left, right)
            Symbol.Var.Value.VBoolean(res)
        }
    }

    override fun visitType(ctx: Basic2Parser.TypeContext) = B2Eval.visitType(ctx)

    override fun visitCast(ctx: Basic2Parser.CastContext): Symbol.Var.Value
        = Symbol.Var.Value.withType(exprCtx(ctx.expr()), visitType(ctx.type()))
}
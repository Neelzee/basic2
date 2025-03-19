package b2

import no.nilsmf.antlr.Basic2BaseVisitor
import no.nilsmf.antlr.Basic2Parser
import kotlin.math.max

class B2InterpreterVisitor : Basic2BaseVisitor<Any?>() {

    private var symbolTable = SymbolTable()

    override fun visitIdent(ctx: Basic2Parser.IdentContext): Value
        = symbolTable.getVar(ctx.IDENTIFIER().text).value

    override fun visitFloat(ctx: Basic2Parser.FloatContext): Value.VFloat {
        val rawTxt = ctx.FLOAT_LIT().text
        return Value.VFloat(rawTxt.replace("f", "").toFloat())
    }

    override fun visitNum(ctx: Basic2Parser.NumContext): Value.VInt {
        val rawTxt = ctx.NUM_LIT().text
        return Value.VInt(rawTxt.replace("f", "").toInt())
    }

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

    override fun visitTuple(ctx: Basic2Parser.TupleContext): Value.Tuple {
        val fst = super.visit(ctx.expr(0)!!) as Value
        val snd = super.visit(ctx.expr(1)!!) as Value
        return Value.Tuple(Pair(fst, snd), Type.Tuple(fst.type(), snd.type()))
    }

    override fun visitArray(ctx: Basic2Parser.ArrayContext): Value.VList
        = ctx.expr()
            .map { super.visit(it) as Value }
            .let { Value.VList(it, Type.TList(it[0].type())) }

    override fun visitVar_decl(ctx: Basic2Parser.Var_declContext): Value.VUnit {
        val (id, type) = ctx.var_decl_stmt().let {
            Pair(it.IDENTIFIER().text, Type.from(it.typing().type().text))
        }
        symbolTable.declVar(id, type)

        return Value.VUnit
    }

    override fun visitVar_ass(ctx: Basic2Parser.Var_assContext): Value.VUnit {
        val dclCtx = ctx.var_decl_ass_stmt()
        val id = dclCtx.IDENTIFIER().text
        val type = dclCtx.typing()?.type()?.text?.let { Type.from(it) }
        val value = super.visit(dclCtx.expr())!! as Value
        if (value.type() != Type.TUnit) symbolTable.declAssVar(id, value, type)
        return Value.VUnit
    }

    override fun visitVar_re_ass(ctx: Basic2Parser.Var_re_assContext): Value.VUnit {
        val raCtx = ctx.var_re_ass_stmt()
        val id = raCtx.IDENTIFIER().text
        val value = super.visit(raCtx.expr())!! as Value
        symbolTable.reAssVar(id, value)
        return Value.VUnit
    }

    override fun visitPrint(ctx: Basic2Parser.PrintContext): Value.VUnit {
        val pCtx = ctx.print_stmt()
        val value = super.visit(pCtx.expr())!! as Value
        println(value.value())

        return Value.VUnit
    }

    override fun visitInput(ctx: Basic2Parser.InputContext): Value.VUnit {
        val iCtx = ctx.input_stmt()
        print(super.visit(iCtx.expr()))
        val value = readlnOrNull() ?: ""
        iCtx.var_decl_stmt()?.let {
            val id = it.IDENTIFIER().text
            val type = it.typing().type().text.let { t -> Type.from(t) }
            symbolTable.declAssVar(id, Value.VString(value), type)
        }
        return Value.VUnit
    }

    override fun visitBinop(ctx: Basic2Parser.BinopContext): Value {
        val left = ctx.children?.get(0)?.let { super.visit(it) as Value }!!
        val right = ctx.children?.get(2)?.let { super.visit(it) as Value }!!

        @Suppress("UNCHECKED_CAST")
        return (super.visit(ctx.children!![1]) as ((Value, Value) -> Value))(left, right)
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
        val args = ctx.expr().map { super.visit(it) as Value }
        val fnBody = symbolTable.getImpl(fn)
        return fnBody.body(args)
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

    override fun visitFn_impl(ctx: Basic2Parser.Fn_implContext): Value.VUnit {
        val fnCtx = ctx.fn_impl_stmt()
        val id = fnCtx.IDENTIFIER().text
        val paramIds = fnCtx.fn_param().map { super.visit(it) as Symbol.Arg }
        val paramTypes = symbolTable.getParams(id)
        val pairs = paramTypes.zip(paramIds)
        val body = fnCtx.stmt()
        val fnBody: (List<Value?>) -> Value = { args ->
            symbolTable = symbolTable.enterScope()
            if (args.isNotEmpty()) {
                for (i in 0..(max(args.size, pairs.size))) {
                    val value = args[i]
                    val (typeInfo, optArg) = pairs[i]
                    val v = value ?: optArg.value ?: throw RuntimeException("$id has no value")
                    symbolTable.declAssVar(optArg.id, v, typeInfo.type)
                }
            }
            val result = (super.visit(body) as Value)
            symbolTable = symbolTable.exitScope()
            result
        }
        symbolTable.addFnImpl(id, paramIds, fnBody)
        return Value.VUnit
    }

    fun printSymbolTable() = symbolTable.print()
}
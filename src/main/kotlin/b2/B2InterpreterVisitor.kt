package b2

import no.nilsmf.antlr.Basic2BaseVisitor
import no.nilsmf.antlr.Basic2Parser

class B2InterpreterVisitor : Basic2BaseVisitor<Any?>() {

    private val symbolTable = SymbolTable()

    override fun visitIdent(ctx: Basic2Parser.IdentContext): Any?
        = symbolTable.getVarNullable(ctx.IDENTIFIER().text)?.value?.value()

    override fun visitFloat(ctx: Basic2Parser.FloatContext): Float {
        val rawTxt = ctx.FLOAT_LIT().text
        return rawTxt.replace("f", "").toFloat()
    }

    override fun defaultResult(): Any? {
        return null
    }

    override fun visitStr(ctx: Basic2Parser.StrContext): String {
        val rawTxt = ctx.STR_LIT().text
        return rawTxt.replace("\"", "").replace("\"", "")
    }

    override fun visitBool(ctx: Basic2Parser.BoolContext): Boolean {
        val rawTxt = ctx.BOOL_LIT().text
        return rawTxt == "TRUE"
    }

    override fun visitTuple(ctx: Basic2Parser.TupleContext): Any? {
        TODO()
    }

    override fun visitArray(ctx: Basic2Parser.ArrayContext): Any? {
        TODO()
    }

    override fun visitVar_decl(ctx: Basic2Parser.Var_declContext) {
        val (id, type) = ctx.var_decl_stmt().let {
            Pair(it.IDENTIFIER().text, Type.from(it.typing().type().text))
        }
        symbolTable.declVar(id, type)
    }

    override fun visitVar_ass(ctx: Basic2Parser.Var_assContext) {
        val dclCtx = ctx.var_decl_ass_stmt()
        val id = dclCtx.IDENTIFIER().text
        val type = dclCtx.typing()?.type()?.text?.let { Type.from(it) }
        val value = Value.from(super.visit(dclCtx.expr())!!)
        symbolTable.declAssVar(id, value, type)
    }

    override fun visitVar_re_ass(ctx: Basic2Parser.Var_re_assContext) {
        val raCtx = ctx.var_re_ass_stmt()
        val id = raCtx.IDENTIFIER().text
        val value = Value.from(super.visit(raCtx.expr())!!)
        symbolTable.reAssVar(id, value)
    }

    override fun visitPrint(ctx: Basic2Parser.PrintContext) {
        val pCtx = ctx.print_stmt()
        val value = super.visit(pCtx.expr())!!
        println(value)
    }

    override fun visitInput(ctx: Basic2Parser.InputContext) {
        val iCtx = ctx.input_stmt()
        print(super.visit(iCtx.expr()))
        val value = readlnOrNull() ?: ""
        iCtx.var_decl_stmt()?.let {
            val id = it.IDENTIFIER().text
            val type = it.typing().type().text.let { t -> Type.from(t) }
            symbolTable.declAssVar(id, Value.VString(value), type)
        }
    }

    override fun visitBinop(ctx: Basic2Parser.BinopContext): Value? {
        val left = ctx.children?.get(0)?.let { super.visit(it)?.let { v -> Value.from(v) } }!!
        val right = ctx.children?.get(2)?.let { super.visit(it)?.let { v -> Value.from(v) } } ?: Value.VNull(Type.TInt)

        @Suppress("UNCHECKED_CAST")
        return (super.visit(ctx.children!![1]) as ((Value, Value) -> Value)?)?.let { it(left, right) }
    }

    override fun visitAdd(ctx: Basic2Parser.AddContext): (Value, Value) -> Value = { a, b -> a + b }

    override fun visitSub(ctx: Basic2Parser.SubContext): (Value, Value) -> Value = { a, b -> a - b }

    override fun visitDiv(ctx: Basic2Parser.DivContext): (Value, Value) -> Value = { a, b -> a / b }

    override fun visitMul(ctx: Basic2Parser.MulContext): (Value, Value) -> Value = { a, b -> a * b }

    override fun visitMod(ctx: Basic2Parser.ModContext): (Value, Value) -> Value = { a, b -> a % b }

    override fun visitPow(ctx: Basic2Parser.PowContext): (Value, Value) -> Value = { a, b -> a.pow(b) }

    override fun visitAnd(ctx: Basic2Parser.AndContext): (Value, Value) -> Value = { a, b -> a.and(b) }

    override fun visitOr(ctx: Basic2Parser.OrContext): (Value, Value) -> Value = { a, b -> a.or(b) }

    override fun visitMutAdd(ctx: Basic2Parser.MutAddContext): ((Value, Value) -> Value)? {
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value + right
            symbolTable.reAssVar(id, lv)
        }
        return null
    }

    override fun visitMutSub(ctx: Basic2Parser.MutSubContext): ((Value, Value) -> Value)? {
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value - right
            symbolTable.reAssVar(id, lv)
        }
        return null
    }

    override fun visitMutDiv(ctx: Basic2Parser.MutDivContext): ((Value, Value) -> Value)? {
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value / right
            symbolTable.reAssVar(id, lv)
        }
        return null
    }

    override fun visitMutMul(ctx: Basic2Parser.MutMulContext):((Value, Value) -> Value)? {
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value * right
            symbolTable.reAssVar(id, lv)
        }
        return null
    }

    override fun visitMutMod(ctx: Basic2Parser.MutModContext):((Value, Value) -> Value)?{
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value % right
            symbolTable.reAssVar(id, lv)
        }
        return null
    }

    override fun visitMutPow(ctx: Basic2Parser.MutPowContext):((Value, Value) -> Value)?{
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value.pow(right)
            symbolTable.reAssVar(id, lv)
        }
        return null
    }

    override fun visitMutAnd(ctx: Basic2Parser.MutAndContext):((Value, Value) -> Value)? {
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value.and(right)
            symbolTable.reAssVar(id, lv)
        }
        return null
    }

    override fun visitMutOr(ctx: Basic2Parser.MutOrContext):((Value, Value) -> Value)?{
        val id = ctx.children?.get(0)?.text ?: ""
        symbolTable.getVarNullable(id)?.let {
            val right = Value.from(super.visit(ctx.children?.get(2)!!))
            val lv = it.value.or(right)
            symbolTable.reAssVar(id, lv)
        }
        return null
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

    override fun visitFn_decl(ctx: Basic2Parser.Fn_declContext): Any? {
        return super.visitFn_decl(ctx)
    }

    override fun visitFn_impl(ctx: Basic2Parser.Fn_implContext): Any? {
        return super.visitFn_impl(ctx)
    }
}
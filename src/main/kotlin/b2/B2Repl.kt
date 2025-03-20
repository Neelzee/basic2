package b2

import b2.symbols.Value
import no.nilsmf.antlr.Basic2BaseVisitor
import no.nilsmf.antlr.Basic2Parser

class B2Repl : Basic2BaseVisitor<Value>(){
    override fun defaultResult(): Value {
        TODO("Not yet implemented")
    }

    override fun visitTuple(ctx: Basic2Parser.TupleContext): Value.Tuple {
        return TODO()
    }

    override fun visitStr(ctx: Basic2Parser.StrContext): Value.VString {
        return TODO()
    }

    override fun visitFloat(ctx: Basic2Parser.FloatContext): Value.VFloat {
        return TODO()
    }

    override fun visitNum(ctx: Basic2Parser.NumContext): Value.VInt {
        return TODO()
    }
}
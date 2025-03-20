package b2

import b2.symbols.Symbol

sealed class B2Exception : Throwable() {
    data class ReturnException(val returnValue: Symbol.Var) : B2Exception()
    data object BreakException : B2Exception() {
        private fun readResolve(): Any = BreakException
    }

    fun getSymbol() : Symbol.Var = when (this) {
        is ReturnException -> returnValue
        is BreakException -> Symbol.Var.VUnit
    }
}
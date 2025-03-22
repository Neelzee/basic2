package b2

import b2.symbols.Symbol
import org.antlr.v4.kotlinruntime.ast.Position

sealed class B2Exception : Throwable() {
    data class MissingModuleException(val moduleName: String, val position: Position?) : B2Exception()
    sealed class TypeException(val position: Position? = null) : B2Exception() {
        data class InvalidModuleException(val moduleName1: String, val moduleName2: String) : TypeException()
        data class NumberFormatException(val rawText: String, val pos: Position? = null) : TypeException(pos)
        data class NotBoolException(val rawText: String, val pos: Position? = null) : TypeException(pos)
        data object InvalidOpException : TypeException(null) {
            private fun readResolve(): Any = InvalidOpException
        }
        data class InvalidOperandsException(
            val operands: List<Symbol.Var.Type>,
            val pos: Position? = null
        ) : TypeException(pos)
        data class NotIterableException(val type: Symbol.Var.Type, val pos: Position? = null) : TypeException(pos)
        data class NonMatchingParamException(val id: String, val pos: Position? = null) : TypeException(pos)
        data class NonMatchingParamTypeException(val id: String, val pos: Position? = null) : TypeException(pos)
        data class InvalidIndexingException(
            val id: String,
            val type: Symbol.Var.Type,
            val pos: Position? = null
        ) : TypeException(pos)
        data class InvalidIndexingTypeException(
            val id: String,
            val type: Symbol.Var.Type,
            val pos: Position? = null
        ) : TypeException(pos)
        data class InvalidIndexingElementException(
            val id: String,
            val type: Symbol.Var.Type,
            val element: Symbol.Var.Type,
            val pos: Position? = null
        ) : TypeException(pos)
        data class ReassignmentException(
            val id: String,
            val newType: Symbol.Var.Type,
            val oldType: Symbol.Var.Type,
            val pos: Position? = null
        ) : TypeException(pos)
        data class InvalidCastException(
            val id: String,
            val valueType: Symbol.Var.Type,
            val type: Symbol.Var.Type,
            val pos: Position? = null
        ) : TypeException(pos)
        data class ImportException(
            val id: String,
            val e: TypeException,
            val pos: Position? = null
        ) : TypeException(pos)
        data class MissingModuleException(val id: String, val pos: Position? = null) : TypeException(pos)
    }
    data class ReturnException(val returnValue: Symbol.Var.Value) : B2Exception()
    data object BreakException : B2Exception() {
        private fun readResolve(): Any = BreakException
    }

    fun getSymbol() : Symbol.Var.Value = when (this) {
        is ReturnException -> returnValue
        else -> Symbol.Var.Value.VUnit
    }
}
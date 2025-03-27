package b2.interpreter

import b2.symbols.B2Ast
import b2.symbols.Position

sealed class B2Exception : Throwable() {
    data class MissingModuleException(val moduleName: String, val position: Position?) : B2Exception()
    sealed class TypeException(val position: Position? = null) : B2Exception() {
        data class MissingVariableException(val ident: String, val pos: Position?) : TypeException(pos)
        data class MissingTypeAliasException(val ident: String) : TypeException(null)
        data class InvalidModuleException(val moduleName1: String, val moduleName2: String) : TypeException()
        data class NumberFormatException(val rawText: String, val pos: Position? = null) : TypeException(pos)
        data class NotBoolException(val rawText: String, val pos: Position? = null) : TypeException(pos)
        data object InvalidOpException : TypeException(null) {
            private fun readResolve(): Any = InvalidOpException
        }
        data class InvalidOperandsException(
            val operands: List<B2Ast.Type>,
            val pos: Position? = null
        ) : TypeException(pos)
        data class NotIterableException(val type: B2Ast.Type, val pos: Position? = null) : TypeException(pos)
        data class NonMatchingParamException(val id: String, val pos: Position? = null) : TypeException(pos)
        data class NonMatchingParamTypeException(val id: String, val pos: Position? = null) : TypeException(pos)
        data class InvalidIndexingException(
            val id: String,
            val type: B2Ast.Type,
            val pos: Position? = null
        ) : TypeException(pos)
        data class InvalidIndexingTypeException(
            val id: String,
            val type: B2Ast.Type,
            val pos: Position? = null
        ) : TypeException(pos)
        data class InvalidIndexingElementException(
            val id: String,
            val type: B2Ast.Type,
            val element: B2Ast.Type,
            val pos: Position? = null
        ) : TypeException(pos)
        data class ReassignmentException(
            val id: String,
            val newType: B2Ast.Type,
            val oldType: B2Ast.Type,
            val pos: Position? = null
        ) : TypeException(pos)
        data class InvalidCastException(
            val operand: B2Ast.Expression,
            val valueType: B2Ast.Type,
            val type: B2Ast.Type,
            val pos: Position? = null
        ) : TypeException(pos)
        data class ImportException(
            val id: String,
            val e: TypeException,
            val pos: Position? = null
        ) : TypeException(pos)
        data class MissingModuleException(val id: String, val pos: Position? = null) : TypeException(pos)
        data class InvalidResultType(
            val declaredType: B2Ast.Type,
            val inferredType: B2Ast.Type,
            val pos: Position? = null
        ) : TypeException(pos)
        data class MissingFunctionDeclarationException(
            val id: String,
            val pos: Position? = null
        ) : TypeException(pos)
        data class MissingFunctionImplementationException(
            val id: String,
            val pos: Position? = null
        ) : TypeException(pos)
        data class MissingArgumentException(
            val ident: String,
            val argName: String,
            val pos: Position? = null
        ) : TypeException(pos)
        data class InvalidArgumentCountException(
            val ident: String,
            val declaredCount: Int,
            val implementedCount: Int,
            val pos: Position? = null
        ) : TypeException(pos)
        data class InvalidArgumentInImplAndDeclException(
            val ident: String,
            val argName: String,
            val declaredType: B2Ast.Type,
            val implementedType: B2Ast.Type,
            val pos: Position? = null
        ) : TypeException(pos)
        data class InvalidArgumentTypeException(
            val ident: String,
            val argName: String,
            val actualType: B2Ast.Type,
            val expectedType: B2Ast.Type,
            val pos: Position? = null
        ) : TypeException(pos)
        data class InvalidControlFlowException(
            val controlFlow: B2Exception,
            val pos: Position? = null
        ) : TypeException(pos)
    }
    data object ContinueException : B2Exception() {
        private fun readResolve(): Any = ContinueException
    }

    data class ReturnException(val returnValue: Val?) : B2Exception()
    data object BreakException : B2Exception() {
        private fun readResolve(): Any = BreakException
    }

    fun getVal() = when (this) {
        is ReturnException -> returnValue ?: Val.Unit
        else -> Val.Unit
    }
}
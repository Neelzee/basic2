package b2.interpreter

import b2.symbols.B2Ast
import b2.symbols.SymbolTable
import b2.visitor.B2
import no.nilsmf.antlr.Basic2Parser

sealed class Val {
    data class Int(val value: kotlin.Int) : Val()
    data class Float(val value: kotlin.Float) : Val()
    data class Str(val value: String) : Val()
    data class Bool(val value: Boolean) : Val()
    data class Tuple(val fst: Val, val snd: Val) : Val()
    data class Array(val value: kotlin.Array<Val>) : Val() {
        override fun equals(other: Any?): Boolean {
            if (this === other) return true
            if (javaClass != other?.javaClass) return false

            other as Array

            return value.contentEquals(other.value)
        }

        override fun hashCode(): kotlin.Int {
            return value.contentHashCode()
        }
    }
}

class B2Interpreter {
    private val scopes = ArrayDeque<SymbolTable>()

    fun enterScope() {
        scopes.add(SymbolTable())
    }

    fun exitScope() {
        scopes.removeFirst()
    }

    fun getVar(ident: String): B2Ast.Expression.Var
        = scopes.firstOrNull { it.hasVar(ident) }?.getVarNullable(ident)
            ?: throw RuntimeException("Variable: $ident not found")

    fun reAssignVar(ident: String, value: Val) {
        val scope = scopes.firstOrNull { it.hasVar(ident) }
            ?: throw RuntimeException("Variable: $ident not found")
        scope.reAssVar(ident, value)
    }

    /**
     * Runs the given statements in another scope, if a return or break happens,
     * the scope is exited, and the exception is thrown further up
     *
     * Used for if-statements, block-statements, etc.
     */
    fun runScope(f: () -> B2Ast.Expression): B2Ast.Expression = try {
        enterScope()
        f()
    } catch (e: B2Exception.ReturnException) {
        exitScope()
        throw e
    } catch (e: B2Exception.BreakException) {
        exitScope()
        throw e
    }

    /**
     * Runs the given statements in another scope, if a return or break happens,
     * the scope is exited, and the exception is thrown further up, if it is a return.
     * If not, the break stops at here.
     *
     * Used for for-statements, while-statements, etc.
     */
    fun runLoop(f: () -> B2Ast.Expression): B2Ast.Expression = try {
        enterScope()
        f()
    } catch (e: B2Exception.ReturnException) {
        exitScope()
        throw e
    } catch (_: B2Exception.BreakException) {
        exitScope()
        B2Ast.Expression.NoOp
    }

    /**
     * Runs the given statements in another scope, if a return or break happens,
     * the scope is exited, and the exception is thrown further up, if it is a break.
     * If not, the return stops at here.
     *
     * Used for function calls
     */
    fun runFnCall(f: () -> B2Ast.Expression): B2Ast.Expression = try {
        enterScope()
        f()
    } catch (e: B2Exception.ReturnException) {
        exitScope()
        e.getSymbol()
    } catch (e: B2Exception.BreakException) {
        exitScope()
        throw e
    }

    fun interpret(ctx: Basic2Parser.ProgramContext) {
        val astVisitor = B2()
        val ast = astVisitor.visitProgram(ctx)
    }

    fun parse(stmt: B2Ast.Statement) = when (stmt) {
        is B2Ast.Statement.ArrIndReAssign -> {
            val arr = getVar(stmt.ident)
            when (val value = arr.value()) {
                is String -> TODO("String indexing reassignment")
                is Array<*> -> {
                    val newArr = value.toMutableList()
                    newArr.add(stmt.newValue)
                }
                is Pair<*, *> -> TODO()
                else -> TODO()
            }
        }
        is B2Ast.Statement.Block -> TODO()
        is B2Ast.Statement.Break -> throw B2Exception.BreakException
        is B2Ast.Statement.Continue -> throw B2Exception.ContinueException
        is B2Ast.Statement.Return -> throw B2Exception.ReturnException(stmt.value)
        is B2Ast.Statement.FnDecl -> TODO()
        is B2Ast.Statement.FnImpl -> TODO()
        is B2Ast.Statement.For -> TODO()
        is B2Ast.Statement.ForIter -> TODO()
        is B2Ast.Statement.If -> TODO()
        is B2Ast.Statement.Import -> TODO()
        is B2Ast.Statement.ImportSpecific -> TODO()
        B2Ast.Statement.NoOp -> TODO()
        is B2Ast.Statement.Print -> TODO()
        is B2Ast.Statement.VarAssDecl -> TODO()
        is B2Ast.Statement.VarDecl -> TODO()
        is B2Ast.Statement.VarReAssign -> TODO()
        is B2Ast.Statement.While -> TODO()
    }
}
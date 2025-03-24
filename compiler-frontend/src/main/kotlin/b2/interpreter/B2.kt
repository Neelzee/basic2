package b2.interpreter

import b2.symbols.Symbol
import b2.symbols.SymbolTable
import no.nilsmf.antlr.Basic2BaseVisitor

open class B2(
    private var symbolTable: SymbolTable = SymbolTable(),
    private val debug: Boolean = false
) : Basic2BaseVisitor<Symbol>() {

    override fun defaultResult(): Symbol.Var = Symbol.Var.Value.VUnit

    fun getSymbolTable() = symbolTable

    fun printSymbolTable() = symbolTable.print()

    /**
     * Runs the given statements in another scope, if a return or break happens,
     * the scope is exited, and the exception is thrown further up
     *
     * Used for if-statements, block-statements, etc.
     */
    fun runScope(f: () -> Symbol.Var): Symbol.Var = try {
        symbolTable = symbolTable.enterScope()
        f()
    } catch (e: B2Exception.ReturnException) {
        symbolTable = symbolTable.exitScope(debug)
        throw e
    } catch (e: B2Exception.BreakException) {
        symbolTable = symbolTable.exitScope(debug)
        throw e
    }

    /**
     * Runs the given statements in another scope, if a return or break happens,
     * the scope is exited, and the exception is thrown further up, if it is a return.
     * If not, the break stops at here.
     *
     * Used for for-statements, while-statements, etc.
     */
    fun runLoop(f: () -> Symbol.Var): Symbol.Var = try {
        symbolTable = symbolTable.enterScope()
        f()
    } catch (e: B2Exception.ReturnException) {
        symbolTable = symbolTable.exitScope(debug)
        throw e
    } catch (_: B2Exception.BreakException) {
        symbolTable = symbolTable.exitScope(debug)
        Symbol.Var.Value.VUnit
    }

    /**
     * Runs the given statements in another scope, if a return or break happens,
     * the scope is exited, and the exception is thrown further up, if it is a break.
     * If not, the return stops at here.
     *
     * Used for function calls
     */
    fun runFnCall(f: () -> Symbol.Var): Symbol.Var = try {
        symbolTable = symbolTable.enterScope()
        f()
    } catch (e: B2Exception.ReturnException) {
        symbolTable = symbolTable.exitScope()
        e.getSymbol()
    } catch (e: B2Exception.BreakException) {
        symbolTable = symbolTable.exitScope()
        throw e
    }

}
package b2

import b2.symbols.Symbol
import b2.symbols.Type
import b2.symbols.Value

data class SymbolTable(
    private val variables: MutableMap<String, Symbol.Var> = mutableMapOf(),
    private val fnImpls: MutableMap<String, Symbol.FnImpl> = mutableMapOf(),
    private val fnDecls: MutableMap<String, Symbol.FnDecl> = mutableMapOf(),
    private var next: SymbolTable? = null,
) {

    companion object {
        fun SymbolTable(table: SymbolTable): SymbolTable = SymbolTable(
            variables = mutableMapOf(),
            fnImpls = mutableMapOf(),
            fnDecls = mutableMapOf(),
            next = table
        )
    }

    fun enterScope(): SymbolTable = SymbolTable(
        variables = mutableMapOf(),
        fnImpls = mutableMapOf(),
        fnDecls = mutableMapOf(),
        next = this
    )

    fun exitScope(): SymbolTable = next ?: this

    fun declVar(id: String, type: Type) {
        variables[id] = Symbol.Var(Value.VNull(type))
    }

    fun declAssVar(id: String, value: Value, type: Type? = null) {
        val t = type ?: value.type()
        val v = if (type == null) { value } else { Value.withType(value, t) }
        variables[id] = Symbol.Var(v)
    }

    fun reAssVar(id: String, value: Value) {
        when (val old = getVarNullable(id)) {
            is Symbol.Var -> {
                if (old.value.type() != value.type()) throw RuntimeException("Cannot reassign with different types")
                variables[id] = Symbol.Var(value)
            }
            else -> throw RuntimeException("Cannot reassign non-existing variable")
        }
    }

    fun getVar(id: String): Symbol.Var = when (variables[id]) {
        is Symbol.Var -> variables[id] as Symbol.Var
        else -> next?.getVar(id) ?: throw RuntimeException("Could not find $id in scope")
    }

    fun remVar(id: String) {
        variables.remove(id)
    }

    fun getVarNullable(id: String): Symbol.Var? = when (variables[id]) {
        is Symbol.Var -> variables[id] as Symbol.Var
        else -> next?.getVarNullable(id)
    }

    fun addFnDecl(id: String, params: List<Type>, result: Type) {
        fnDecls[id] = Symbol.FnDecl(params.map { Symbol.Param(it) }, result)
    }

    fun addFnImpl(id: String, args: List<Symbol.Arg>, body: (List<Value?>) -> Value) {
        fnImpls[id] = Symbol.FnImpl(args, body)
    }

    fun getDecl(id: String): Symbol.FnDecl = when {
        fnDecls[id] != null -> fnDecls[id] as Symbol.FnDecl
        else -> next?.getDecl(id) ?: throw RuntimeException("Could not find $id in scope")
    }

    fun getDeclNullable(id: String): Symbol.FnDecl? = when {
        fnDecls[id] != null -> fnDecls[id] as Symbol.FnDecl
        else -> next?.getDecl(id)
    }

    fun getImpl(id: String): Symbol.FnImpl  = when {
        fnImpls[id] != null -> fnImpls[id] as Symbol.FnImpl
        else -> next?.getImplNullable(id) ?: throw RuntimeException("Could not find $id in scope")
    }

    fun getImplNullable(id: String): Symbol.FnImpl? = when {
        fnImpls[id] != null -> fnImpls[id] as Symbol.FnImpl
        else -> next?.getImplNullable(id)
    }

    fun getParams(id: String): List<Symbol.Param> = fnDecls[id]?.params
        ?: throw RuntimeException("Could not find $id in declarations")

    fun print() {
        next?.printVariables()
        printVariables()
        println("Function Declarations:")
        next?.printFnDecls()
        printFnDecls()
        println("Function Implementations: ")
        next?.printFnImpls()
        printFnImpls()
    }

    private fun printVariables() = variables.forEach { (k, v) -> println("$k: $v") }

    private fun printFnImpls() = fnImpls.forEach { (k, v) -> println("$k: $v") }

    private fun printFnDecls() = fnDecls.forEach { (k, v) -> println("$k: $v") }

}

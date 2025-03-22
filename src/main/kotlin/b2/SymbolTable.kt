package b2

import b2.symbols.Symbol

data class SymbolTable(
    private val variables: MutableMap<String, Symbol.Var> = mutableMapOf(),
    private val fnImpls: MutableMap<String, Symbol.FnImpl> = mutableMapOf(),
    private val fnDecls: MutableMap<String, Symbol.FnDecl> = mutableMapOf(),
    private val modules: MutableList<String> = mutableListOf(),
    private var next: SymbolTable? = null,
    private val scopes: MutableList<SymbolTable> = mutableListOf(),
) {

    fun getScopes() = scopes
    fun getNext() = next

    fun enterScope(): SymbolTable = SymbolTable(
        variables = mutableMapOf(),
        fnImpls = mutableMapOf(),
        fnDecls = mutableMapOf(),
        next = this
    )

    fun exitScope(debug: Boolean = false): SymbolTable = if (debug) {
        next?.let {
            next!!.scopes.add(this)
            next
        } ?: this
    } else {
        next ?: this
    }

    fun declModule(module: String) {
        modules.add(module)
    }

    fun hasModule(module: String): Boolean = when (modules.contains(module)) {
        true -> true
        false -> next?.hasModule(module) == true
    }

    fun declVar(id: String, type: Symbol.Var.Type) {
        variables[id] = Symbol.Var.Value.VNull(type)
    }

    fun declAssVar(id: String, value: Symbol.Var.Value, type: Symbol.Var.Type? = null) {
        variables[id] = if (type == null) { value } else { Symbol.Var.Value.withType(value, type) }
    }

    fun reAssVar(id: String, value: Symbol.Var.Value) {
        when (val old = variables[id]) {
            is Symbol.Var.Value -> {
                if (old.type() != value.type()) throw RuntimeException("Cannot reassign with different types")
                variables[id] = value
            }
            else -> if (next == null) {
                throw RuntimeException("Cannot reassign non-existing variable")
            } else {
                next!!.reAssVar(id, value)
            }
        }
    }

    fun getVar(id: String): Symbol.Var.Value = when (variables[id]) {
        is Symbol.Var.Value -> variables[id] as Symbol.Var.Value
        else -> next?.getVar(id) ?: throw RuntimeException("Could not find $id in scope")
    }

    fun remVar(id: String) {
        variables.remove(id)
    }

    fun getVarNullable(id: String): Symbol.Var? = when (variables[id]) {
        is Symbol.Var -> variables[id] as Symbol.Var
        else -> next?.getVarNullable(id)
    }

    fun addFnDecl(id: String, params: List<Symbol.Var.Type>, result: Symbol.Var.Type) {
        fnDecls[id] = Symbol.FnDecl(params.map { Symbol.Param(it) }, result)
    }

    fun addFnImpl(id: String, args: List<Symbol.Arg>, body: (List<Symbol.Var.Value?>) -> Symbol.Var.Value) {
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
        println("Function Variables:")
        printVariables()
        println("Function Variables END")
        println("Function Declarations:")
        printFnDecls()
        println("Function Declarations END")
        println("Function Implementations:")
        printFnImpls()
        println("Function Implementations END")
    }

    private fun printVariables() {
        variables.forEach { (k, v) -> println("$k: ${v.format()}") }
        next?.let {
            println("Inner: ")
            it.printVariables()
            println("END")
        }
    }

    private fun printFnImpls() {
        fnImpls.forEach { (k, v) -> println("$k: $v") }
        next?.let {
            println("Inner: ")
            it.printFnImpls()
            println("END")
        }
    }

    private fun printFnDecls() {
        fnDecls.forEach { (k, v) -> println("$k: $v") }
        next?.let {
            println("Inner: ")
            it.printFnDecls()
            println("END")
        }
    }

}

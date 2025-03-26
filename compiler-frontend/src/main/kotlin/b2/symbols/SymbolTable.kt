package b2.symbols

data class SymbolTable(
    private val variables: MutableMap<String, B2Ast.Expression.Var> = mutableMapOf(),
    private val fnImpls: MutableMap<String, B2Ast.Statement.FnImpl> = mutableMapOf(),
    private val fnDecls: MutableMap<String, B2Ast.Statement.FnDecl> = mutableMapOf(),
    private val modules: MutableList<String> = mutableListOf(),
) {

    fun iterVariable() = variables.asIterable()

    fun iterFnDecl() = fnDecls.asIterable()

    fun iterFnImpl() = fnImpls.asIterable()

    fun declModule(module: String) {
        modules.add(module)
    }

    fun hasVar(id: String): Boolean = variables.containsKey(id)

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
                if (old.type() == value.type() || old.type() == Symbol.Var.Type.TUnit)
                    variables[id] = value
                else
                    throw RuntimeException("Cannot reassign with different types: ${old.type()} and ${value.type()}")
            }
            else -> if (next == null) {
                throw RuntimeException("Cannot reassign non-existing variable")
            } else {
                next!!.reAssVar(id, value)
            }
        }
    }

    fun getVarNullable(id: String) = variables[id]

    fun addFnDecl(id: String, params: List<Symbol.Var.Type>, result: Symbol.Var.Type) {
        fnDecls[id] = Symbol.Fn.FnDecl(null, params.map { Symbol.Param(it) }, result)
    }

    fun addFnImpl(id: String, args: List<Symbol.Arg>, body: (List<Symbol.Var.Value?>) -> Symbol.Var.Value) {
        fnImpls[id] = Symbol.Fn.FnImpl(args, body)
    }

    fun getDeclNullable(id: String) = fnDecls[id]

    fun getImplNullable(id: String) = fnImpls[id]

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
package b2.symbols

data class SymbolTable(
    private val variables: MutableMap<String, B2Ast.Expression> = mutableMapOf(),
    private val fnImpls: MutableMap<String, B2Ast.Statement.FnImpl> = mutableMapOf(),
    private val fnDecls: MutableMap<String, B2Ast.Statement.FnDecl> = mutableMapOf(),
    private val typeDecls: MutableMap<String, B2Ast.Type> = mutableMapOf(),
    private val modules: MutableList<String> = mutableListOf(),
) {

    fun iterVariable() = variables.asIterable()

    fun iterFnDecl() = fnDecls.asIterable()

    fun iterFnImpl() = fnImpls.asIterable()

    fun declType(id: String, type: B2Ast.Type) {
        typeDecls[id] = type
    }

    fun getTypeNullable(id: String) = typeDecls[id]

    fun declModule(module: String) {
        modules.add(module)
    }

    fun hasVar(id: String): Boolean = variables.containsKey(id)

    fun declVar(id: String, type: B2Ast.Type) {
        variables[id] = type.default()
    }

    fun declAssVar(id: String, value: B2Ast.Expression, type: B2Ast.Type? = null) {
        variables[id] = if (type == null) { value } else { B2Ast.Expression.Cast(value, type, null) }
    }

    fun reAssVar(id: String, value: B2Ast.Expression) {
        variables[id] = value
    }

    fun getVarNullable(id: String) = variables[id]

    fun addFnDecl(id: String, params: List<B2Ast.Type>, result: B2Ast.Type) {
        fnDecls[id] = B2Ast.Statement.FnDecl(id, params, result, null)
    }

    fun addFnImpl(id: String, args: List<Pair<String, B2Ast.Expression?>>, body: List<B2Ast.Statement>) {
        fnImpls[id] = B2Ast.Statement.FnImpl(id, args, body, null)
    }

    fun getDeclNullable(id: String) = fnDecls[id]

    fun getImplNullable(id: String) = fnImpls[id]
}
package b2

class SymbolTable {
    private val table = mutableMapOf<String, Symbol>()

    fun declVar(id: String, type: Type) {
        table[id] = Symbol.Var(Value.VNull(type))
    }

    fun declAssVar(id: String, value: Value, type: Type?) {
        val t = type ?: value.type()
        val v = if (type == null) { value } else { Value.withType(value, t) }
        table[id] = Symbol.Var(v)
    }

    fun reAssVar(id: String, value: Value) {
        when (val old = table[id]) {
            is Symbol.Var -> {
                if (old.value.type() != value.type()) throw RuntimeException("Cannot reassign with different types")
                table[id] = Symbol.Var(value)
            }
            else -> throw RuntimeException("Cannot reassign non-existing variable")
        }
    }

    fun getVar(id: String): Symbol.Var = when (table[id]) {
        is Symbol.Var -> table[id] as Symbol.Var
        else -> throw RuntimeException("Could not find $id in scope")
    }


    fun getVarNullable(id: String): Symbol.Var? = when (table[id]) {
        is Symbol.Var -> table[id] as Symbol.Var
        else -> null
    }
}

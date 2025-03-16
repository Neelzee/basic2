package b2

sealed class Value {
    data class VInt(val v: Int) : Value()
    data class VString(val v: String) : Value()
    data class VBoolean(val v: Boolean) : Value()
    data class VList(val v: List<Value>) : Value()
    data class FnDecl(val types: List<Type>, val retType: Type) : Value()
    data class FnImpl(val params: List<Param>, val fn: Unit) : Value()
    data class Param(val id: String, val default: Value? = null) : Value()
}


sealed class Type {
    data object Int : Type()
    data object Str : Type()
    data object Bool : Type()
    data class List(val t: Type) : Type()
}

class SymbolTable {
    private val table = mutableMapOf<String, Value>()

    fun addVariable(name: String, value: Value) {
        table[name] = value
    }
}

package b2.symbols

data class TypeTable(
    private val mutableVariables: MutableList<String> = mutableListOf(),
) {
    fun addVar(ident: String) {
        mutableVariables.add(ident)
    }

    fun getVariables(): List<String> = mutableVariables.toList()
}
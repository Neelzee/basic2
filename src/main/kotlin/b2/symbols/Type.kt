package b2.symbols

sealed class Type {
    data object TUnit : Type()
    data object TInt : Type()
    data object TFloat : Type()
    data object TStr : Type()
    data object TBool : Type()
    data class Tuple(val fst: Type, val snd: Type) : Type()
    data class TList(val t: Type) : Type()

    companion object {
        fun infer(v: Any?): Type = when (v) {
            is Int -> TInt
            is Float -> TFloat
            is Boolean -> TBool
            is String -> TStr
            is MutableList<*> -> TList(infer(v.first()))
            is Pair<*, *> -> Tuple(infer(v.first), infer(v.second))
            is Value -> v.type()
            else -> throw RuntimeException("Unknown type inference: $v")
        }

        fun from(s: String): Type = when (s.trim()) {
            "INT" -> TInt
            "FLOAT" -> TFloat
            "BOOL" -> TBool
            "STR" -> TStr
            else -> when {
                s.contains('[') ->
                    TList(from(s.replace("[", "").replace("]", "")))
                s.contains('(') -> s.replace("(", "")
                    .replace(")", "")
                    .split(",")
                    .map { it.trim() }
                    .let { (fst, snd) -> Tuple(from(fst), from(snd)) }
                else -> throw RuntimeException("Unknown type: $s")
            }
        }
    }
}
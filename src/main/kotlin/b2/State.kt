package b2

import kotlin.math.pow

sealed class Value {
    data object VUnit : Value()
    data class VNull(val type: Type) : Value()
    data class VInt(val value: Int) : Value()
    data class VFloat(val value: Float) : Value()
    data class VString(val value: String) : Value()
    data class VBoolean(val value: Boolean) : Value()
    data class Tuple(val value: Pair<Value, Value>, val type: Type.Tuple) : Value()
    data class VList(val value: MutableList<Value>, val type: Type.TList) : Value()

    fun type(): Type = when (this) {
        is VNull -> this.type
        is VInt -> Type.TInt
        is VFloat -> Type.TFloat
        is VString -> Type.TStr
        is VBoolean -> Type.TBool
        is Tuple -> this.type
        is VList -> this.type
        is VUnit -> Type.TUnit
    }

    operator fun plus(b: Value): Value {
        val left = this.value()
        val right = b.value()
        return when {
            (left is Int && right is Int) -> VInt(left + right)
            (left is Float && right is Int) -> VFloat(left + right)
            (left is Int && right is Float) -> VFloat(left + right)
            (left is Float && right is Float) -> VFloat(left + right)
            (left is String && right is String) -> VString(left + right)
            (left is MutableList<*> && right is MutableList<*>) -> (left + right)
                .map { from(it) }
                .let { VList(it.toMutableList(), Type.infer(it) as Type.TList) }
            (left is Pair<*, *> && right is Pair<*, *>) -> {
                val (ll, lr) = left.let { Pair(from(it.first), from(it.second) ) }
                val (rl, rr) = right.let { Pair(from(it.first), from(it.second) ) }
                val pair = Pair(ll + rl, lr + rr)
                Tuple(pair, Type.Tuple(Type.infer(pair.first), Type.infer(pair.second)))
            }
            else -> throw RuntimeException("Illegal operation, cannot add $this and $b")
        }
    }

    fun value() : Any = when (this) {
        is VInt -> this.value
        is VFloat -> this.value
        is VString -> this.value
        is VBoolean -> this.value
        is Tuple -> this.value
        is VList -> this.value
        else -> throw RuntimeException("Could not get returnValue from $this")
    }

    operator fun minus(b: Value): Value {
        val left = this.value()
        val right = b.value()
        return when {
            (left is Int && right is Int) -> VInt(left - right)
            (left is Float && right is Int) -> VFloat(left - right)
            (left is Int && right is Float) -> VFloat(left - right)
            (left is Float && right is Float) -> VFloat(left - right)
            (left is String && right is String) -> VString(left.replace(right, ""))
            (left is Pair<*, *> && right is Pair<*, *>) -> {
                val (ll, lr) = left.let { Pair(from(it.first), from(it.second) ) }
                val (rl, rr) = right.let { Pair(from(it.first), from(it.second) ) }
                val pair = Pair(ll - rl, lr - rr)
                Tuple(pair, Type.Tuple(Type.infer(pair.first), Type.infer(pair.second)))
            }
            else -> throw RuntimeException("Illegal operation, cannot add $this and $b")
        }
    }

    operator fun div(b: Value): Value {
        val left = this.value()
        val right = b.value()
        return when {
            (left is Int && right is Int) -> VInt(left / right)
            (left is Float && right is Int) -> VFloat(left / right)
            (left is Int && right is Float) -> VFloat(left / right)
            (left is Float && right is Float) -> VFloat(left / right)
            (left is String && right is String) -> VString(left + right)
            (left is MutableList<*> && right is MutableList<*>) -> left.filter { !right.contains(it) }
                .map { from(it) }
                .let { VList(it.toMutableList(), Type.infer(it) as Type.TList) }
            (left is Pair<*, *> && right is Pair<*, *>) -> {
                val (ll, lr) = left.let { Pair(from(it.first), from(it.second) ) }
                val (rl, rr) = right.let { Pair(from(it.first), from(it.second) ) }
                val pair = Pair(ll / rl, lr / rr)
                Tuple(pair, Type.Tuple(Type.infer(pair.first), Type.infer(pair.second)))
            }
            else -> throw RuntimeException("Illegal operation, cannot add $this and $b")
        }
    }

    operator fun times(b: Value): Value {
        val left = this.value()
        val right = b.value()
        return when {
            (left is Int && right is Int) -> VInt(left * right)
            (left is Float && right is Int) -> VFloat(left * right)
            (left is Int && right is Float) -> VFloat(left * right)
            (left is Float && right is Float) -> VFloat(left * right)
            (left is String && right is Int) -> VString(left.repeat(right))
            (left is Int && right is String) -> VString(right.repeat(left))
            (left is MutableList<*> && right is MutableList<*>) -> left.zip(right)
                .map { from(it) }
                .let { VList(it.toMutableList(), Type.infer(it) as Type.TList) }
            (left is Pair<*, *> && right is Pair<*, *>) -> {
                val (ll, lr) = left.let { Pair(from(it.first), from(it.second) ) }
                val (rl, rr) = right.let { Pair(from(it.first), from(it.second) ) }
                val pair = Pair(ll * rl, lr * rr)
                Tuple(pair, Type.Tuple(Type.infer(pair.first), Type.infer(pair.second)))
            }
            else -> throw RuntimeException("Illegal operation, cannot add $this and $b")
        }
    }

    operator fun rem(b: Value): Value {
        val left = this.value()
        val right = b.value()
        return when {
            (left is Int && right is Int) -> VInt(left % right)
            (left is Float && right is Int) -> VFloat(left % right)
            (left is Int && right is Float) -> VFloat(left % right)
            (left is Float && right is Float) -> VFloat(left % right)
            (left is Pair<*, *> && right is Pair<*, *>) -> {
                val (ll, lr) = left.let { Pair(from(it.first), from(it.second) ) }
                val (rl, rr) = right.let { Pair(from(it.first), from(it.second) ) }
                val pair = Pair(ll % rl, lr % rr)
                Tuple(pair, Type.Tuple(Type.infer(pair.first), Type.infer(pair.second)))
            }
            else -> throw RuntimeException("Illegal operation, cannot add $this and $b")
        }
    }

    fun pow(b: Value): Value {
        val left = this.value()
        val right = b.value()
        return when {
            (left is Int && right is Int) -> VInt(left.toFloat().pow(right).toInt())
            (left is Float && right is Int) -> VFloat(left.pow(right))
            (left is Int && right is Float) -> VFloat(left.toFloat().pow(right))
            (left is Float && right is Float) -> VFloat(left.pow(right))
            (left is Pair<*, *> && right is Pair<*, *>) -> {
                val (ll, lr) = left.let { Pair(from(it.first), from(it.second) ) }
                val (rl, rr) = right.let { Pair(from(it.first), from(it.second) ) }
                val pair = Pair(ll.pow(rl), rr.pow(lr))
                Tuple(pair, Type.Tuple(Type.infer(pair.first), Type.infer(pair.second)))
            }
            else -> throw RuntimeException("Illegal operation, cannot add $this and $b")
        }
    }

    fun and(b: Value): Value {
        val left = this.value()
        val right = b.value()
        return when {
            (left is Boolean && right is Boolean) -> VBoolean(left && right)
            (left is Pair<*, *> && right is Pair<*, *>) -> {
                val (ll, lr) = left.let { Pair(from(it.first), from(it.second) ) }
                val (rl, rr) = right.let { Pair(from(it.first), from(it.second) ) }
                val pair = Pair(ll.and(rl), lr.and(rr))
                Tuple(pair, Type.Tuple(Type.infer(pair.first), Type.infer(pair.second)))
            }
            else -> throw RuntimeException("Illegal operation, cannot add $this and $b")
        }
    }

    fun or(b: Value): Value {
        val left = this.value()
        val right = b.value()
        return when {
            (left is Boolean && right is Boolean) -> VBoolean(left || right)
            (left is Pair<*, *> && right is Pair<*, *>) -> {
                val (ll, lr) = left.let { Pair(from(it.first), from(it.second) ) }
                val (rl, rr) = right.let { Pair(from(it.first), from(it.second) ) }
                val pair = Pair(ll.or(rl), lr.or(rr))
                Tuple(pair, Type.Tuple(Type.infer(pair.first), Type.infer(pair.second)))
            }
            else -> throw RuntimeException("Illegal operation, cannot add $this and $b")
        }
    }

    operator fun compareTo(b: Value): Int {
        val left = this.value()
        val right = b.value()
        return when {
            (left is Int && right is Int) -> left.compareTo(right)
            (left is Float && right is Int) -> left.compareTo(right)
            (left is Int && right is Float) -> left.compareTo(right)
            (left is Float && right is Float) -> left.compareTo(right)
            (left is String && right is String) -> left.length.compareTo(right.length)
            (left is MutableList<*> && right is MutableList<*>) -> left.size.compareTo(right.size)
            else -> throw RuntimeException("Illegal operation, cannot add $this and $b")
        }
    }

    fun not(): Value = when (this) {
        is VBoolean -> VBoolean(!this.value)
        is VList -> VList(this.value.reversed().toMutableList(), this.type)
        is VString -> VString(this.value.reversed())
        is Tuple -> Tuple(Pair(this.value.second, this.value.first), Type.Tuple(this.type.snd, this.type.snd))
        else -> throw RuntimeException("Illegal operation, cannot negate $this")
    }

    fun sig(): Value = when (this) {
        is VFloat -> VFloat(-this.value)
        is VInt -> VInt(-this.value)
        else -> throw RuntimeException("Illegal operation, cannot sign $this")
    }

    fun inc(): Value = when (this) {
        is VFloat -> VFloat(this.value + 1)
        is VInt -> VInt(this.value + 1)
        else -> throw RuntimeException("Illegal operation, cannot sign $this")
    }

    fun dec(): Value = when (this) {
        is VFloat -> VFloat(this.value - 1)
        is VInt -> VInt(this.value - 1)
        else -> throw RuntimeException("Illegal operation, cannot sign $this")
    }

    fun toBool(): Boolean = when (this) {
        is VBoolean -> this.value
        is Tuple -> this.value.first.toBool() && this.value.second.toBool()
        is VFloat -> this.value.isFinite() && this.value == 1f
        is VInt -> this.value == 1
        is VList -> this.value.isNotEmpty()
        is VString -> this.value.lowercase() == "true"
        else -> throw RuntimeException("Illegal operation, cannot convert $this to Bool")
    }

    fun toIterable(): Iterable<*> = when (this) {
        is VList -> this.value.toList()
        else -> throw RuntimeException("Illegal operation, cannot iterate $this")
    }

    operator fun get(ind: VInt): Value = when (this) {
        is VList if (ind.value >= this.value.size || ind.value < 0) ->
            throw RuntimeException("OutOfBoundsException: ${ind.value} on $this")
        is VList -> this.value[ind.value]
        is Tuple if (ind.value == 0) -> this.value.first
        is Tuple if (ind.value == 1) -> this.value.second
        is Tuple ->
            throw RuntimeException("OutOfBoundsException: ${ind.value} on $this")
        else -> throw RuntimeException("Illegal operation, cannot index $this")
    }

    operator fun set(ind: VInt, newValue: Value): Value = when (this) {
        is VList if (ind.value >= this.value.size || ind.value < 0) ->
            throw RuntimeException("OutOfBoundsException: ${ind.value} on $this")
        is VList if (newValue.type() != this.type.t) ->
            throw RuntimeException(
                "Illegal operation, element $newValue is of type ${newValue.type()}" +
                        ", while the array is of type: ${this.type}"
            )
        is VList -> this.value[ind.value]
        else -> throw RuntimeException("Illegal operation, cannot index $this")
    }

    fun size(): Int = when (this) {
        is VList -> this.value.size
        is VString -> this.value.length
        else -> throw RuntimeException("Illegal operation, cannot take length of $this")
    }

    fun add(value: Value) = when (this) {
        is VList if (this.type == Type.TList(value.type())) -> this.value.add(value)
        is VList ->
            throw RuntimeException(
                "Illegal operation, element $value is of type ${value.type()}, while the array is of type: ${this.type}"
            )
        else -> throw RuntimeException("Illegal operation cannot append element to $this")
    }


    companion object {
        fun from(v: Any?): Value = when (v) {
            is Int -> VInt(v)
            is Float -> VFloat(v)
            is Boolean -> VBoolean(v)
            is String -> VString(v)
            is Pair<*, *> ->
                Tuple(Pair(from(v.first), from(v.second)), Type.Tuple(Type.infer(v.first), Type.infer(v.second)))
            is MutableList<*> -> VList(v.map { from(it!!) }.toMutableList(), Type.infer(v) as Type.TList)
            is Value -> v
            else -> throw RuntimeException("Unknown type: $v")
        }

        fun withType(v: Value, t: Type): Value = when {
            v is VInt && t is Type.TFloat -> VFloat(v.value.toFloat())
            v is VFloat && t is Type.TInt -> VInt(v.value.toInt())
            v is VString && t is Type.TInt -> VInt(v.value.toInt())
            v is VString && t is Type.TFloat -> VFloat(v.value.toFloat())
            v is VList && t is Type.TList -> v.copy(type = t)
            else -> if (v.type() == t) {
                v
            } else {
                throw RuntimeException("Invalid type casting: ${v.type()} to $t")
            }
        }
    }
}

sealed class Symbol {
    data class Var(val value: Value) : Symbol()
    data class Param(val type: Type)
    data class Arg(val id: String, val value: Value?)
    data class FnDecl(
        val params: List<Param>,
        val resultType: Type
    ) : Symbol()
    data class FnImpl(val args: List<Arg>, val body: (args: List<Value?>) -> Value) : Symbol()
}

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
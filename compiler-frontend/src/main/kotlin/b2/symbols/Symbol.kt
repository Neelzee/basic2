package b2.symbols

import b2.interpreter.B2Exception
import b2.symbols.Symbol.Var.Value.*
import kotlinx.serialization.Serializable

import kotlin.math.pow

sealed class Symbol {
    sealed class Var : Symbol() {

        open fun value(): Any = when (this) {
            is Variable -> this.value.value()
            is Value -> this.value()
            else -> throw RuntimeException("Cant get value from type")
        }

        open fun format(): String = when (this) {
            is Type.TBool -> "BOOL"
            is Type.TFloat -> "FLOAT"
            is Type.TInt -> "NUM"
            is Type.TList -> "[${this.t}]"
            is Type.TStr -> "STR"
            is Type.TUnit -> "UNIT"
            is Type.Tuple -> "(${this.fst}, ${this.snd})"
            is Value -> this.format()
            is Variable -> "${this.id} = ${this.value}"
            is ImportItem -> TODO()
        }

        data class ImportItem(val id: String, val type: Symbol, val newName: String? = null) : Var()

        @Serializable
        data class Variable(
            val id: String = "",
            val value: Value = VUnit
        ) : Var()

        @Serializable
        sealed class Type : Var() {
            @Serializable
            data object TUnit : Type()
            @Serializable
            data object TInt : Type()
            @Serializable
            data object TFloat : Type()
            @Serializable
            data object TStr : Type()
            @Serializable
            data object TBool : Type()
            @Serializable
            data class Tuple(val fst: Type, val snd: Type) : Type()
            @Serializable
            data class TList(val t: Type) : Type()

            operator fun plus(right: Type): Type {
                val left = this
                return when {
                    (right == this) -> this
                    (left is TFloat && right is TInt) -> TFloat
                    (left is TInt && right is TFloat) -> TFloat
                    else -> throw B2Exception.TypeException.InvalidOperandsException(listOf(left, right))
                }
            }

            operator fun minus(right: Type): Type {
                val left = this
                return when {
                    (right == this) -> this
                    (left is TFloat && right is TInt) -> TFloat
                    (left is TInt && right is TFloat) -> TFloat
                    else -> throw B2Exception.TypeException.InvalidOperandsException(listOf(left, right))
                }
            }

            operator fun times(right: Type): Type {
                val left = this
                return when {
                    (right == this) -> if (this is TList || this is Tuple || this is TStr) {
                        throw B2Exception.TypeException.InvalidOperandsException(listOf(left, right))
                    } else {
                        this
                    }
                    (right == this) -> this
                    (left is TFloat && right is TInt) -> TFloat
                    (left is TInt && right is TFloat) -> TFloat
                    (left is TStr && right is TInt) -> TStr
                    (left is TInt && right is TStr) -> TStr
                    else -> throw B2Exception.TypeException.InvalidOperandsException(listOf(left, right))
                }
            }

            operator fun rem(right: Type): Type {
                val left = this
                return when {
                    (right == this) -> if (this is TList || this is Tuple || this is TStr) {
                        throw B2Exception.TypeException.InvalidOperandsException(listOf(left, right))
                    } else {
                        this
                    }
                    (right == this) -> this
                    else -> throw B2Exception.TypeException.InvalidOperandsException(listOf(left, right))
                }
            }

            fun add(value: Type) = when (this) {
                is TList if (this == TList(value)) -> Unit
                is TList if (this.t == TUnit) -> Unit
                else -> throw B2Exception.TypeException.InvalidOperandsException(listOf(value, this))
            }

            fun default(): Value = when (this) {
                is TBool -> VBoolean(true)
                is TFloat -> VFloat(0f)
                is TInt -> VInt(0)
                is TList -> VList(mutableListOf<Value>(), this)
                is TStr -> VString("")
                is TUnit -> VUnit
                is Tuple -> Tuple(Pair(this.fst.default(), this.snd.default()), this)
            }

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

        @Serializable
        sealed class Value : Var() {
            @Serializable
            data object VUnit : Value()
            @Serializable
            data class VNull(val type: Type) : Value()
            @Serializable
            data class VInt(val value: Int) : Value()
            @Serializable
            data class VFloat(val value: Float) : Value()
            @Serializable
            data class VString(var value: String) : Value()
            @Serializable
            data class VBoolean(val value: Boolean) : Value()
            @Serializable
            data class Tuple(val value: Pair<Value, Value>, val type: Type.Tuple) : Value()
            @Serializable
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
                        .let { VList(it.toMutableList(), Type.Companion.infer(it) as Type.TList) }
                    (left is Pair<*, *> && right is Pair<*, *>) -> {
                        val (ll, lr) = left.let { Pair(from(it.first), from(it.second) ) }
                        val (rl, rr) = right.let { Pair(from(it.first), from(it.second) ) }
                        val pair = Pair(ll + rl, lr + rr)
                        Tuple(pair, Type.Tuple(Type.Companion.infer(pair.first), Type.Companion.infer(pair.second)))
                    }
                    else -> throw RuntimeException("Illegal operation, cannot add $this and $b")
                }
            }

            override fun value() : Any = when (this) {
                is VInt -> this.value
                is VFloat -> this.value
                is VString -> this.value
                is VBoolean -> this.value
                is Tuple -> this.value
                is VList -> this.value
                else -> throw RuntimeException("Could not get returnVar from $this")
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
                        Tuple(pair, Type.Tuple(Type.Companion.infer(pair.first), Type.Companion.infer(pair.second)))
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
                        .let { VList(it.toMutableList(), Type.Companion.infer(it) as Type.TList) }
                    (left is Pair<*, *> && right is Pair<*, *>) -> {
                        val (ll, lr) = left.let { Pair(from(it.first), from(it.second) ) }
                        val (rl, rr) = right.let { Pair(from(it.first), from(it.second) ) }
                        val pair = Pair(ll / rl, lr / rr)
                        Tuple(pair, Type.Tuple(Type.Companion.infer(pair.first), Type.Companion.infer(pair.second)))
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
                        .let { VList(it.toMutableList(), Type.Companion.infer(it) as Type.TList) }
                    (left is Pair<*, *> && right is Pair<*, *>) -> {
                        val (ll, lr) = left.let { Pair(from(it.first), from(it.second) ) }
                        val (rl, rr) = right.let { Pair(from(it.first), from(it.second) ) }
                        val pair = Pair(ll * rl, lr * rr)
                        Tuple(pair, Type.Tuple(Type.Companion.infer(pair.first), Type.Companion.infer(pair.second)))
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
                        Tuple(pair, Type.Tuple(Type.Companion.infer(pair.first), Type.Companion.infer(pair.second)))
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
                        Tuple(pair, Type.Tuple(Type.Companion.infer(pair.first), Type.Companion.infer(pair.second)))
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
                        Tuple(pair, Type.Tuple(Type.Companion.infer(pair.first), Type.Companion.infer(pair.second)))
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
                        Tuple(pair, Type.Tuple(Type.Companion.infer(pair.first), Type.Companion.infer(pair.second)))
                    }
                    else -> throw RuntimeException("Illegal operation, cannot || $this and $b")
                }
            }

            override fun format(): String = when (this) {
                is Tuple -> "(${this.value.first}, ${this.value.second})"
                is VBoolean -> this.value.toString()
                is VFloat -> this.value.toString()
                is VInt -> this.value.toString()
                is VList -> "[${this.value.joinToString(", ") { it.format() }}]"
                is VNull -> "NULL"
                is VString -> this.value
                is VUnit -> "UNIT"
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
                is VString if (ind.value >= this.value.length || ind.value < 0) ->
                    throw RuntimeException("OutOfBoundsException: ${ind.value} on $this")
                is VString -> Value.VString(this.value[ind.value].toString())
                is Tuple if (ind.value == 0) -> this.value.first
                is Tuple if (ind.value == 1) -> this.value.second
                is Tuple ->
                    throw RuntimeException("OutOfBoundsException: ${ind.value} on $this")
                else -> throw RuntimeException("Illegal operation, cannot index $this")
            }

            operator fun set(ind: VInt, newVar: Value): Value = when (this) {
                is VList if (ind.value >= this.value.size || ind.value < 0) ->
                    throw RuntimeException("OutOfBoundsException: ${ind.value} on $this")
                is VList if (newVar.type() != this.type.t) ->
                    throw RuntimeException(
                        "Illegal operation, element $newVar is of type ${newVar.type()}" +
                                ", while the array is of type: ${this.type}"
                    )
                is VList -> {
                    this.value[ind.value] = newVar
                    this
                }
                is VString if (ind.value >= this.value.length || ind.value < 0) ->
                    throw RuntimeException("OutOfBoundsException: ${ind.value} on $this")
                is VString if (newVar.type() !is Type.TStr) ->
                    throw RuntimeException(
                        "Illegal operation, element $newVar is of type ${newVar.type()}, which is not STR"
                    )
                is VString -> {
                    var i = 0;
                    var newStr = "";
                    for (c in this.value) {
                        newStr += if (ind.value == i) {
                            newVar.value() as String
                        } else {
                            c
                        }
                        i++
                    }
                    this.value = newStr
                    this
                }
                else -> throw RuntimeException("Illegal operation, cannot index $this")
            }

            fun size(): VInt = when (this) {
                is VList -> VInt(this.value.size)
                is VString -> VInt(this.value.length)
                else -> throw RuntimeException("Illegal operation, cannot take length of $this")
            }

            fun add(value: Value) = when (this) {
                is VList if (this.type.t == value.type()) -> this.value.add(value)
                is VList if (this.type.t == Type.TUnit) -> this.value.add(value)
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
                        Tuple(Pair(from(v.first), from(v.second)), Type.Tuple(Type.Companion.infer(v.first), Type.Companion.infer(v.second)))
                    is MutableList<*> -> VList(v.map { from(it!!) }.toMutableList(), Type.Companion.infer(v) as Type.TList)
                    is Value -> v
                    else -> throw RuntimeException("Unknown type: $v")
                }

                fun withType(v: Value, t: Type): Value = when {
                    v is VInt && t is Type.TFloat -> VFloat(v.value.toFloat())
                    v is VInt && t is Type.TStr -> VString(v.value.toString())
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
    }
    data class Param(val type: Var.Type) : Symbol()
    data class Arg(val id: String, var value: Var.Value?) : Symbol()
    sealed class Fn : Symbol() {
        data class FnImpl(
            val args: List<Arg> = emptyList(),
            val body: (args: List<Var.Value?>) -> Var.Value = { VUnit }
        ) : Fn()
        data class FnDecl(
            val id: String? = null,
            val params: List<Param> = emptyList(),
            val resultType: Var.Type = Var.Type.TUnit
        ) : Fn()
    }
}
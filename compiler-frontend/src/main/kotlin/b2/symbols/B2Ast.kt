package b2.symbols

import b2.interpreter.B2Exception
import b2.symbols.B2Ast.Expression.*
import b2.symbols.B2Ast.Type.*
import kotlinx.serialization.Serializable

@Serializable
data class Point(
    val line: Int,
    val column: Int,
) {
    companion object {
        fun from(other: org.antlr.v4.kotlinruntime.ast.Point) = Point(other.line, other.column)
    }
}

@Serializable
data class Position(
    val start: Point,
    val end: Point,
) {
    companion object {
        fun from(other: org.antlr.v4.kotlinruntime.ast.Position?)
            = other?.let { Position(Point.from(it.start), Point.from(it.end)) }
    }
}

@Serializable
sealed class B2Ast {

    data class Program(val program: String, val statements: List<Statement>) : B2Ast()

    @Serializable
    sealed class Statement : B2Ast() {
        @Serializable
        data class TypeAlias(val ident: String, val type: Type) : Statement()
        @Serializable
        data class NoOp(val position: Position? = null) : Statement()
        @Serializable
        data class VarDecl(val ident: String, val type: Type, val position: Position?) : Statement()
        @Serializable
        data class VarAssDecl(
            val ident: String,
            val type: Type,
            val value: Expression,
            val position: Position?
        ) : Statement()
        @Serializable
        data class VarReAssign(
            val ident: String,
            val newValue: Expression,
            val position: Position?
        ) : Statement()
        @Serializable
        data class ArrIndReAssign(
            val ident: String,
            val ind: Expression,
            val newValue: Expression,
            val position: Position?
        ) : Statement()
        @Serializable
        data class Unpack(
            val idents: Array<String>,
            val expr: Expression,
            val position: Position?
        ) : Statement() {
            override fun equals(other: Any?): Boolean {
                if (this === other) return true
                if (javaClass != other?.javaClass) return false

                other as Unpack

                return idents.contentEquals(other.idents)
            }

            override fun hashCode(): Int {
                return idents.contentHashCode()
            }
        }

        @Serializable
        data class ImportSpecific(
            val moduleIdent: String,
            val imports: List<Pair<ImportItem, String?>>,
            val position: Position?
            ) : Statement()
        @Serializable
        data class Import(
            val moduleIdent: String,
            val rename: String?,
            val position: Position?
            ) : Statement()
        @Serializable
        sealed class ImportItem() : B2Ast() {
            @Serializable
            data class Var(val value: String, val position: Position?) : ImportItem()
            @Serializable
            data class FnDecl(val ident: String, val position: Position?) : ImportItem()
            @Serializable
            data class FnImpl(val ident: String, val position: Position?) : ImportItem()
            @Serializable
            data class FnDeclImpl(val ident: String, val position: Position?) : ImportItem()
        }
        @Serializable
        data class FnDecl(
            val ident: String,
            val params: List<Type>,
            val resultType: Type,
            val position: Position?
        ) : Statement()
        @Serializable
        data class FnImpl(
            val ident: String,
            val args: List<Pair<String, Expression?>>,
            val body: List<Statement>,
            val position: Position?
        ) : Statement()
        @Serializable
        data class If(
            val condition: Expression,
            val thenBranch: List<Statement>,
            val elif: List<Pair<Expression, List<Statement>>> = listOf(),
            val elseBranch: List<Statement> = listOf(),
            val position: Position?
        ) : Statement()
        @Serializable
        data class While(
            val condition: Expression,
            val body: List<Statement>,
            val position: Position?,
        ) : Statement()
        @Serializable
        data class Continue(val position: Position?) : Statement();
        @Serializable
        data class Break(val position: Position?) : Statement();
        @Serializable
        data class Return(val value: Expression?, val position: Position?) : Statement();
        @Serializable
        data class ForIter(
            val ident: String,
            val iter: Expression,
            val body: List<Statement>,
            val position: Position?
        ) : Statement()
        @Serializable
        data class For(
            val i: String,
            val init: Expression,
            val cond: Expression,
            val h: String,
            val incr: Expression,
            val body: List<Statement>,
            val position: Position?,
        ) : Statement()
        @Serializable
        data class Print(val expr: Expression?, val position: Position?) : Statement()
        @Serializable
        data class Block(val body: List<Statement>, val position: Position?) : Statement()
        @Serializable
        data class Expr(val expr: Expression, val position: Position?) : Statement()
    }
    @Serializable
    sealed class Expression : B2Ast() {
        @Serializable
        data class NoOp(val position: Position? = null) : Expression()
        @Serializable
        data class Var(val ident: String, val position: Position?) : Expression()
        @Serializable
        data class Str(val value: String, val position: Position?) : Expression() {
            companion object {
                fun default() = Str("", null)
            }
        }
        @Serializable
        data class Int(val value: kotlin.Int, val position: Position?) : Expression() {
            companion object {
                fun default() = Int(0, null)
            }
        }
        @Serializable
        data class Bol(val value: Boolean, val position: Position?) : Expression() {
            companion object {
                fun default() = Bol(true, null)
            }
        }
        @Serializable
        data class Flt(val value: Float, val position: Position?) : Expression() {
            companion object {
                fun default() = Flt(0f, null)
            }
        }
        @Serializable
        data class Arr(
            val value: Array<Expression>,
            val elementType: Type,
            val size: kotlin.Int,
            val position: Position?
        ) : Expression() {
            override fun equals(other: Any?): Boolean {
                if (this === other) return true
                if (javaClass != other?.javaClass) return false

                other as Arr

                if (size != other.size) return false
                if (!value.contentEquals(other.value)) return false
                if (elementType != other.elementType) return false

                return true
            }

            override fun hashCode(): kotlin.Int {
                var result = size
                result = 31 * result + value.contentHashCode()
                result = 31 * result + elementType.hashCode()
                return result
            }

            companion object {
                fun default() = Arr(emptyArray(), Type.Int, 0, null)
            }
        }
        @Serializable
        data class Group(val value: Expression, val position: Position?) : Expression()
        @Serializable
        data class Tup(val fst: Expression, val snd: Expression, val position: Position?) : Expression()
        @Serializable
        data class FnCall(val ident: String, val args: List<Expression>, val position: Position?) : Expression()
        @Serializable
        data class BinOp(
            val left: Expression,
            val operator: Operator.Bi,
            val right: Expression,
            val position: Position?
        ) : Expression()
        @Serializable
        data class ArrInd(val arr: Expression, val ind: Expression, val position: Position?) : Expression()
        @Serializable
        data class UniOp(
            val value: Expression,
            val operator: Operator.Uni,
            val position: Position?
        ) : Expression()
        @Serializable
        data class Cast(val expr: Expression, val type: Type, val position: Position?) : Expression()
        @Serializable
        data class Input(val prompt: Expression?, val position: Position?) : Expression()
        @Serializable
        data class Trim(val value: Str, val position: Position?) : Expression()
        @Serializable
        data class Len(val value: Expression, val position: Position?) : Expression()

        fun from(s: String): Expression = TODO()

        fun value(): Any {
            TODO("Not yet implemented")
        }

        fun type(): Type = when (this) {
            is ArrInd -> this.ind.type()
            is BinOp -> Type.Unit
            is Bol -> Type.Bool
            is Flt -> Type.Float
            is FnCall -> Type.Unit
            is Int -> Type.Int
            is Arr -> Lst(this.elementType)
            is NoOp -> Type.Unit
            is Str -> Type.Str
            is Tup -> Type.Tup(this.fst.type(), this.snd.type())
            is UniOp -> Type.Unit
            is Var -> Type.Unit
            is Cast -> TODO()
            is Input -> TODO()
            is Len -> TODO()
            is Trim -> TODO()
            is Group -> TODO()
        }
    }

    @Serializable
    sealed class Operator : B2Ast() {
        @Serializable
        sealed class Uni : Operator() {
            @Serializable
            data object Incr : Uni()
            @Serializable
            data object Decr : Uni()
            @Serializable
            data object Sign : Uni()
        }
        @Serializable
        sealed class Bi : Operator() {
            @Serializable
            data object Add : Bi()
            @Serializable
            data object Sub : Bi()
            @Serializable
            data object Mul : Bi()
            @Serializable
            data object Div : Bi()
            @Serializable
            data object AddMut : Bi()
            @Serializable
            data object SubMut : Bi()
            @Serializable
            data object MulMut : Bi()
            @Serializable
            data object DivMut : Bi()
            @Serializable
            data object Eq : Bi()
            @Serializable
            data object Neq : Bi()
            @Serializable
            data object Gt : Bi()
            @Serializable
            data object Lt : Bi()
            @Serializable
            data object Geq : Bi()
            @Serializable
            data object Leq : Bi()
        }
    }
    @Serializable
    sealed class Type : B2Ast() {
        @Serializable
        data object Unit : Type()
        @Serializable
        data object Int : Type()
        @Serializable
        data object Float : Type()
        @Serializable
        data object Str : Type()
        @Serializable
        data object Bool : Type()
        @Serializable
        data class Tup(val fst: Type, val snd: Type) : Type()
        @Serializable
        data class Lst(val elementType: Type) : Type()
        @Serializable
        data class TypeAlias(val ident: String) : Type()

        operator fun plus(right: Type): Type {
            val left = this
            return when {
                (right == this) -> this
                (left is Float && right is Int) -> Float
                (left is Int && right is Float) -> Float
                else -> throw B2Exception.TypeException.InvalidOperandsException(listOf(left, right))
            }
        }

        operator fun minus(right: Type): Type {
            val left = this
            return when {
                (right == this) -> this
                (left is Float && right is Int) -> Float
                (left is Int && right is Float) -> Float
                else -> throw B2Exception.TypeException.InvalidOperandsException(listOf(left, right))
            }
        }

        operator fun times(right: Type): Type {
            val left = this
            return when {
                (right == this) -> if (this is Lst || this is Tup || this is Str) {
                    throw B2Exception.TypeException.InvalidOperandsException(listOf(left, right))
                } else {
                    this
                }
                (right == this) -> this
                (left is Float && right is Int) -> Float
                (left is Int && right is Float) -> Float
                (left is Str && right is Int) -> Str
                (left is Int && right is Str) -> Str
                else -> throw B2Exception.TypeException.InvalidOperandsException(listOf(left, right))
            }
        }

        operator fun rem(right: Type): Type {
            val left = this
            return when {
                (right == this) -> if (this is Lst || this is Tup || this is Str) {
                    throw B2Exception.TypeException.InvalidOperandsException(listOf(left, right))
                } else {
                    this
                }
                (right == this) -> this
                else -> throw B2Exception.TypeException.InvalidOperandsException(listOf(left, right))
            }
        }

        fun add(value: Type) = when (this) {
            is Lst if (this == Lst(value)) -> Unit
            is Lst if (this.elementType == Unit) -> Unit
            else -> throw B2Exception.TypeException.InvalidOperandsException(listOf(value, this))
        }

        fun default(): Expression = when (this) {
            is Bool -> Bol(true, null)
            is Float -> Flt(0f, null)
            is Int -> Expression.Int(0, null)
            is Lst -> Arr(emptyArray(), this.elementType, 0, null)
            is Str -> Expression.Str("", null)
            is Unit, is TypeAlias -> NoOp()
            is Tup -> Expression.Tup(this.fst.default(), this.snd.default(), null)
        }

        companion object {
            fun infer(v: Any?): Type = when (v) {
                is kotlin.Int -> Int
                is kotlin.Float -> Float
                is Boolean -> Bool
                is String -> Str
                is MutableList<*> -> Lst(infer(v.first()))
                is Pair<*, *> -> Tup(infer(v.first), infer(v.second))
                else -> throw RuntimeException("Unknown type inference: $v")
            }

            fun from(s: String): Type = when (val c = s.trim()) {
                "INT" -> Int
                "FLOAT" -> Float
                "BOOL" -> Bool
                "STR" -> Str
                else -> throw RuntimeException("Cannot get type from non-primitive: $c")
            }
        }
    }
}
package b2.symbols

import b2.symbols.Symbol.Var.Type
import kotlinx.serialization.Serializable

@Serializable
sealed class Program(val program: String, val statements: List<Statement>) {

    @Serializable
    sealed class Statement {

        @Serializable
        data class VarDecl(val ident: String, val type: Type) : Statement()
        @Serializable
        data class VarAssDecl(val ident: String, val type: Type? = null, val value: Expression) : Statement()
        @Serializable
        data class VarReAssign(val ident: String, val newValue: Expression) : Statement()
        @Serializable
        data class ArrIndReAssign(val ident: String, val ind: Expression, val newValue: Expression) : Statement()
        @Serializable
        data class ImportSpecific(val moduleIdent: String, val imports: List<Pair<ImportItem, String?>>) : Statement()
        @Serializable
        data class Import(val moduleIdent: String, val rename: String?) : Statement()
        @Serializable
        sealed class ImportItem() {
            @Serializable
            data class Var(val ident: String, val type: Type) : ImportItem()
            @Serializable
            data class FnDecl(val fn: Statement.FnDecl) : ImportItem()
            @Serializable
            data class FnImpl(val fn: Statement.FnImpl) : ImportItem()
        }
        @Serializable
        data class FnDecl(val ident: String, val params: List<Type>, val resultType: Type) : Statement()
        @Serializable
        data class FnImpl(
            val ident: String,
            val args: List<Pair<String, Expression?>>,
            val resultType: Type
        ) : Statement()
        @Serializable
        data class If(
            val condition: Expression,
            val thenBranch: List<Statement>,
            val elif: List<Pair<Expression, List<Statement>>>,
            val elseBranch: List<Statement>
        ) : Statement()
        @Serializable
        data class While(
            val condition: Expression,
            val body: List<Statement>
        ) : Statement()
        @Serializable
        data object Continue : Statement();
        @Serializable
        data object Break : Statement();
        @Serializable
        data class Return(val value: Expression?) : Statement();
        @Serializable
        data class ForIter(
            val ident: String,
            val iter: Expression,
            val body: List<Statement>,
        ) : Statement()
        @Serializable
        data class For(
            val i: Expression,
            val cond: Expression,
            val incr: Statement,
            val body: List<Statement>,
        ) : Statement()
    }

    @Serializable
    sealed class Expression {
        data class Var(val ident: String) : Expression()
        data class FnCall(val ident: String, val args: List<Expression>)
        data class BinOp(val left: Expression, val operator: Operator, val right: Expression) : Expression()
    }

    @Serializable
    sealed class Operator {
        @Serializable
        data object Add : Operator()
        @Serializable
        data object Sub : Operator()
        @Serializable
        data object Mul : Operator()
        @Serializable
        data object Div : Operator()
        @Serializable
        data object AddMut : Operator()
        @Serializable
        data object SubMut : Operator()
        @Serializable
        data object MulMut : Operator()
        @Serializable
        data object DivMut : Operator()
    }
}
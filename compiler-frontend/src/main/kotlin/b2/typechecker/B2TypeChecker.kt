package b2.typechecker
import b2.interpreter.B2Interpreter
import b2.interpreter.B2Exception
import b2.interpreter.B2Exception.TypeException
import b2.interpreter.B2Exception.TypeException.*
import b2.symbols.B2Ast
import b2.symbols.B2Ast.Expression
import b2.symbols.B2Ast.Statement
import b2.symbols.B2Ast.Type
import b2.symbols.TypeTable

class B2TypeChecker(
    private val typeTable: TypeTable = TypeTable()
) : B2Interpreter() {

    fun getDecl(ident: String): Statement.FnDecl?
        = scopes.firstOrNull { it.getDeclNullable(ident) != null }?.getDeclNullable(ident)

    fun getImpl(ident: String): Statement.FnImpl?
            = scopes.firstOrNull { it.getImplNullable(ident) != null }?.getImplNullable(ident)

    fun typecheck(ast: B2Ast.Program) {

    }

    fun stmt(stmt: Statement): Type = when (stmt) {
        is Statement.Block -> stmt.body.map { stmt(it) }.last()
        is Statement.Break -> Type.Unit
        is Statement.Continue -> Type.Unit
        is Statement.Expr -> expr(stmt.expr)
        is Statement.FnDecl -> {
            declFn(stmt)
            Type.Unit
        }
        is Statement.FnImpl -> {
            implFn(stmt)
            val (ident, args, _, pos) = stmt
            val (_, params, _) = getDecl(ident)
                ?: throw MissingFunctionImplementationException(ident, pos)
            if (args.size != params.size) throw InvalidArgumentCountException(ident, params.size, args.size, pos)

            val invalidArgs = args.zip(params)
                .filter { it.first.second != null }
                .map { Triple(it.first.first, expr(it.first.second!!), it.second) }
                .filterNot { it.second isEquivalentTo it.third  }

            if (invalidArgs.isNotEmpty()) {
                val (id, it, dt) = invalidArgs.first()
                throw InvalidArgumentInImplAndDeclException(ident, id, it, dt)
            }

            Type.Unit
        }
        is Statement.For -> TODO()
        is Statement.ForIter -> TODO()
        is Statement.If -> TODO()
        is Statement.While -> TODO()
        is Statement.ArrIndReAssign -> Type.Unit
        is Statement.VarAssDecl -> Type.Unit
        is Statement.VarDecl -> Type.Unit
        is Statement.VarReAssign -> {
            typeTable.addVar(stmt.ident)
            Type.Unit
        }
        is Statement.Import -> Type.Unit
        is Statement.ImportSpecific -> Type.Unit
        is Statement.NoOp -> Type.Unit
        is Statement.Print -> Type.Unit
        is Statement.Return -> stmt.value?.let { expr(it) } ?: Type.Unit
        is Statement.TypeAlias -> TODO()
        is Statement.Unpack -> TODO()
    }

    fun expr(expr: Expression): Type = when (expr) {
        is Expression.Arr -> expr.type()
        is Expression.ArrInd -> expr(super.eval(expr).toLit())
        is Expression.BinOp -> {
            val (l, op, r, pos) = expr
            val left = expr(l)
            val right = expr(r)
            when (op) {
                B2Ast.Operator.Bi.Add -> when {
                    left is Type.Str || right is Type.Str -> Type.Str
                    left is Type.Int && right is Type.Int -> Type.Int
                    left is Type.Float && right is Type.Int -> Type.Float
                    left is Type.Int && right is Type.Float -> Type.Float
                    left is Type.Float && right is Type.Float -> Type.Float
                    else -> throw B2Exception.TypeException.InvalidOperandsException(listOf(left, right), pos)
                }
                B2Ast.Operator.Bi.AddMut -> TODO()
                B2Ast.Operator.Bi.Div -> TODO()
                B2Ast.Operator.Bi.DivMut -> TODO()
                B2Ast.Operator.Bi.Eq -> TODO()
                B2Ast.Operator.Bi.Geq,
                B2Ast.Operator.Bi.Gt,
                B2Ast.Operator.Bi.Leq,
                B2Ast.Operator.Bi.Lt -> when {
                    (left is Type.Int && right is Type.Int)
                            || (left is Type.Float && right is Type.Int)
                            || (left is Type.Int && right is Type.Float )
                            || (left is Type.Float && right is Type.Float) -> Type.Bool
                    else -> throw B2Exception.TypeException.InvalidOperandsException(listOf(left, right), pos)
                }
                B2Ast.Operator.Bi.Mul -> TODO()
                B2Ast.Operator.Bi.MulMut -> TODO()
                B2Ast.Operator.Bi.Neq -> TODO()
                B2Ast.Operator.Bi.Sub -> TODO()
                B2Ast.Operator.Bi.SubMut -> TODO()
            }
        }
        is Expression.Bol -> Type.Bool
        is Expression.Cast -> {
            val e = expr(expr.expr)
            when {
                else -> throw B2Exception.TypeException.InvalidCastException(
                    operand = expr.expr,
                    valueType = e,
                    type = expr.type,
                    pos = expr.position
                )
            }
        }
        is Expression.Flt -> Type.Float
        is Expression.FnCall -> {
            val (ident, args, pos) = expr
            val (_, argTypes, resultType) = getDecl(ident)
                ?: throw B2Exception.TypeException.MissingFunctionDeclarationException(ident, pos)
            val (_, defArgs, body) = getImpl(ident)
                ?: throw B2Exception.TypeException.MissingFunctionImplementationException(ident, pos)

            val input = defArgs.mapIndexed { i, (ars, df) -> when (val a = if (i >= defArgs.size) df else args[i]) {
                null -> throw B2Exception.TypeException.MissingArgumentException(ident, ars, pos)
                else -> if (a.type() != argTypes[i])
                    throw B2Exception.TypeException.InvalidArgumentTypeException(ident, ars, a.type(), argTypes[i], pos)
                else
                    a
            } }

            fun getRet(stmt: Statement): Type? = when (stmt) {
                    is Statement.Block -> stmt.body.firstOrNull { b -> getRet(b) != null }?.let { getRet(it) }
                    is Statement.FnImpl -> stmt.body.firstOrNull { b -> getRet(b) != null }?.let { getRet(it) }
                    is Statement.For -> stmt.body.firstOrNull { b -> getRet(b) != null }?.let { getRet(it) }
                    is Statement.ForIter -> stmt.body.firstOrNull { b -> getRet(b) != null }?.let { getRet(it) }
                    is Statement.If -> {
                        val lst = stmt.thenBranch.toMutableList()
                        lst.addAll(stmt.elif.flatMap { it.second })
                        lst.addAll(stmt.elseBranch)
                        lst.find { it -> getRet(it) != null }?.let { getRet(it) }
                    }
                    is Statement.Return -> stmt.value?.let { expr(it) }
                    is Statement.While -> stmt.body.firstOrNull { b -> getRet(b) != null }?.let { getRet(it) }
                    else -> null
                }

            var inferredType: Type = body.find { getRet(it) != null }?.let { getRet(it) } ?: Type.Unit
            if (inferredType isEquivalentTo resultType)
                throw InvalidResultType(resultType, inferredType, expr.position)

            resultType
        }
        is Expression.Group -> expr(expr.value)
        is Expression.Input, is Expression.Str, is Expression.Trim -> Type.Str
        is Expression.Int, is Expression.Len -> Type.Int
        is Expression.NoOp -> TODO()
        is Expression.Tup -> Type.Tup(expr(expr.fst), expr(expr.snd))
        is Expression.UniOp -> Type.Unit
        is Expression.Var -> expr(
            getVarNullable(expr.ident)
                ?: throw MissingVariableException(expr.ident, expr.position)
        )
    }

    fun unwrapType(type: Type): Type = when (type) {
        is Type.TypeAlias -> getTypeNullable(type.ident)
            ?: throw MissingTypeAliasException(type.ident)
        else -> type
    }

    infix fun Type.isEquivalentTo(other: Type): Boolean = unwrapType(this) == unwrapType(other)
}
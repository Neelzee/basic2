package b2.interpreter

import b2.interpreter.B2Exception.ReturnException
import b2.symbols.B2Ast
import b2.symbols.B2Ast.Expression.*
import b2.symbols.SymbolTable
import b2.visitor.B2
import no.nilsmf.antlr.Basic2Parser
import kotlin.streams.toList

sealed class Val {
    data object Unit : Val()
    data class Int(val value: kotlin.Int) : Val()
    data class Float(val value: kotlin.Float) : Val()
    data class Str(val value: String) : Val()
    data class Bool(val value: Boolean) : Val()
    data class Tuple(val fst: Val, val snd: Val) : Val() {
        fun unpack(): List<Val> {
            val fst = this.fst
            val snd = this.snd
            val lst = mutableListOf<Val>()
            if (fst is Tuple) lst.addAll(fst.unpack())
            if (snd is Tuple) lst.addAll(snd.unpack())

            return lst
        }
    }
    data class Array(val value: kotlin.Array<Val>) : Val() {
        override fun equals(other: Any?): Boolean {
            if (this === other) return true
            if (javaClass != other?.javaClass) return false

            other as Array

            return value.contentEquals(other.value)
        }

        override fun hashCode(): kotlin.Int {
            return value.contentHashCode()
        }
    }

    fun toLit(): B2Ast.Expression = when (this) {
        is Array ->
            Arr(
                this.value.map { it.toLit() }.toTypedArray(),
                this.value.first().toLit().type(),
                this.value.size,
                null
            )

        is Bool -> Bol(this.value, null)
        is Float -> Flt(this.value, null)
        is Int -> Int(this.value, null)
        is Str -> Str(this.value, null)
        is Tuple -> Tup(this.fst.toLit(), this.snd.toLit(), null)
        Unit -> TODO()
    }

    fun value(): Any = when (this) {
        is Array -> this.value
        is Bool -> this.value
        is Float -> this.value
        is Int -> this.value
        is Str -> this.value
        is Tuple -> Pair(this.fst.value(), this.snd.value())
        Unit -> TODO()
    }

    fun toIter(): List<B2Ast.Expression> = when (this) {
        is Array -> this.value.toList().map { it.toLit() }
        is Str -> this.value.chars().toList().map { Str(it.toChar().toString(), null) }
        else -> throw RuntimeException("Cannot iterate ${this.toLit().type()}")
    }

    operator fun set(ind: Val, newValue: Val): Nothing = when (this) {
        is Array -> TODO()
        is Str -> TODO()
        else -> throw RuntimeException("Cannot index reassign ${this.toLit().type()}")
    }

    operator fun get(ind: Val): Val = when (this) {
        is Array -> this.value[ind.value() as kotlin.Int]
        is Str -> wrap(this.value[ind.value() as kotlin.Int].toString())
        is Tuple if (ind.value() == 0) -> this.fst
        is Tuple if (ind.value() == 1) -> this.snd
        else -> throw RuntimeException("Cannot index ${this.toLit().type()}")
    }

    fun decr(): Val = when (this) {
        is Float -> Float(this.value - 1f)
        is Int -> Int(this.value - 1)
        else -> throw RuntimeException("Cannot decr ${this.toLit().type()}")
    }

    fun incr(): Val = when (this) {
        is Float -> Float(this.value + 1f)
        is Int -> Int(this.value + 1)
        else -> throw RuntimeException("Cannot incr ${this.toLit().type()}")
    }

    fun sign(): Val = when (this) {
        is Float -> Float(-this.value)
        is Int -> Int(-this.value)
        else -> throw RuntimeException("Cannot incr ${this.toLit().type()}")
    }

    operator fun plus(other: Val): Val = when {
        this is Int && other is Float -> TODO()
        this is Float && other is Int -> TODO()
        this is Str && other is Str -> Str(this.value + other.value)
        else -> TODO()
    }

    fun cast(type: B2Ast.Type): Val = when {
        this.toLit().type() == type -> this
        type is B2Ast.Type.Unit -> this
        else -> TODO("$this as $type")
    }

    companion object {
        fun wrap(any: Any): Val = when (any) {
            is Arr -> TODO()
            is Bol -> TODO()
            is Flt -> TODO()
            is B2Ast.Expression.Int -> TODO()
            is B2Ast.Expression.Str -> TODO()
            is Tup -> Tuple(wrap(any.fst), wrap(any.snd))
            is String -> Str(any)
            else -> throw RuntimeException("Invalid wrap: $any")
        }
    }
}

open class B2Interpreter {

    protected val scopes = ArrayDeque<SymbolTable>()

    init {
        scopes.add(SymbolTable())
    }

    fun enterScope() {
        scopes.addFirst(SymbolTable())
    }

    fun exitScope() {
        scopes.removeFirst()
    }

    fun getVarNullable(ident: String) = scopes.firstOrNull { it.hasVar(ident) }?.getVarNullable(ident)

    fun getVar(ident: String): B2Ast.Expression = scopes.firstOrNull { it.hasVar(ident) }?.getVarNullable(ident)
        ?: throw RuntimeException("Variable: $ident not found")

    fun declVar(ident: String, type: B2Ast.Type) = scopes.first().declVar(ident, type)

    fun declAssignVar(ident: String, type: B2Ast.Type?, value: B2Ast.Expression) = scopes.first().declAssVar(
        ident,
        value,
        type,
    )

    fun reAssignVar(ident: String, value: Val) {
        val scope = scopes.firstOrNull { it.hasVar(ident) }
            ?: throw RuntimeException("Variable: $ident not found")
        scope.reAssVar(ident, value.toLit())
    }

    fun declFn(decl: B2Ast.Statement.FnDecl) = scopes.first().addFnDecl(decl.ident, decl.params, decl.resultType)

    fun implFn(impl: B2Ast.Statement.FnImpl) = scopes.first().addFnImpl(impl.ident, impl.args, impl.body)

    fun declType(type: B2Ast.Statement.TypeAlias) = scopes.first().declType(type.ident, type.type)

    fun getTypeNullable(id: String) = scopes.first().getTypeNullable(id)

    fun invokeFn(ident: String, args: List<B2Ast.Expression>): Val {
        val (_, params, body) = scopes.firstOrNull { it.getImplNullable(ident) != null }?.getImplNullable(ident)
            ?: throw RuntimeException("Function implementation: $ident not found")
        return runFnCall {
            params.forEachIndexed { i, (id, def) ->
                if (i >= args.size)
                    declAssignVar(id, null, def!!)
                else
                    declAssignVar(id, null, args[i])
            }
            body.forEach { parse(it) }

            Val.Unit
        }
    }

    /**
     * Runs the given statements in another scope, if a return or break happens,
     * the scope is exited, and the exception is thrown further up
     *
     * Used for if-statements, block-statements, etc.
     */
    fun runScope(f: () -> Unit): Unit = try {
        enterScope()
        f()
    } catch (e: ReturnException) {
        exitScope()
        throw e
    } catch (e: B2Exception.BreakException) {
        exitScope()
        throw e
    }

    /**
     * Runs the given statements in another scope, if a return or break happens,
     * the scope is exited, and the exception is thrown further up, if it is a return.
     * If not, the break stops at here.
     *
     * Used for for-statements, while-statements, etc.
     */
    fun runLoop(f: () -> Unit): Unit = try {
        enterScope()
        f()
    } catch (e: ReturnException) {
        exitScope()
        throw e
    } catch (_: B2Exception.BreakException) {
        exitScope()
    }

    /**
     * Runs the given statements in another scope, if a return or break happens,
     * the scope is exited, and the exception is thrown further up, if it is a break.
     * If not, the return stops at here.
     *
     * Used for function calls
     */
    fun runFnCall(f: () -> Val): Val = try {
        enterScope()
        f()
    } catch (e: ReturnException) {
        val r = e.getVal()
        exitScope()
        r
    } catch (e: B2Exception.BreakException) {
        exitScope()
        throw e
    }

    fun interpret(ctx: Basic2Parser.ProgramContext) {
        val astVisitor = B2()
        val ast: B2Ast.Program = astVisitor.visitProgram(ctx) as B2Ast.Program
        println("RUNNING PROCEDURE ${ast.program}")
        ast.statements.forEach { parse(it) }
        println("ENDING PROCEDURE ${ast.program}")
    }

    fun parse(stmt: B2Ast.Statement): Unit = when (stmt) {
        is B2Ast.Statement.ArrIndReAssign -> {
            val arr = getVar(stmt.ident)
            when (val value = arr.value()) {
                is String -> TODO("String indexing reassignment")
                is Array<*> -> {
                    val newArr = value.toMutableList()
                    newArr[stmt.ind.value() as Int] = stmt.newValue
                    reAssignVar(
                        stmt.ident,
                        Val.Array(newArr.map { Val.wrap(it as B2Ast.Expression) }.toTypedArray())
                    )
                }

                else -> throw RuntimeException("Cannot index ${arr.type()}")
            }
        }

        is B2Ast.Statement.Block -> runScope { stmt.body.forEach { parse(it) } }
        is B2Ast.Statement.Break -> throw B2Exception.BreakException
        is B2Ast.Statement.Continue -> throw B2Exception.ContinueException
        is B2Ast.Statement.Return -> throw ReturnException(stmt.value?.let { eval(it) })
        is B2Ast.Statement.FnDecl -> declFn(stmt)
        is B2Ast.Statement.FnImpl -> implFn(stmt)
        is B2Ast.Statement.For -> runLoop {
            val (i, init, cond, j, incr, body) = stmt
            declAssignVar(i, null, init)
            val c = { eval(cond).value() as Boolean }
            val inc = {
                reAssignVar(j, eval(incr))
            }
            while (c()) {
                body.forEach { parse(it) }
                inc()
            }

        }

        is B2Ast.Statement.ForIter -> runLoop {
            val (ident, iter, body) = stmt
            for (i in eval(iter).toIter()) {
                declAssignVar(ident, null, i)
                body.forEach { parse(it) }
            }
        }

        is B2Ast.Statement.If -> runScope {
            val (cnd, thn, elf, ese) = stmt
            if (eval(cnd).value() as Boolean) {
                thn.forEach { parse(it) }
                return@runScope
            }
            elf.firstOrNull { (c, _) -> eval(c).value() as Boolean }?.let { (_, b) ->
                b.forEach { parse(it) }
                return@runScope
            }
            ese.forEach { parse(it) }
        }

        is B2Ast.Statement.Import -> TODO()
        is B2Ast.Statement.ImportSpecific -> TODO()
        is B2Ast.Statement.NoOp -> Unit
        is B2Ast.Statement.Print ->
            println(stmt.expr?.let { eval(it).value().toString().replace("\"", "").replace("\"", "") } ?: "")

        is B2Ast.Statement.VarAssDecl -> declAssignVar(stmt.ident, stmt.type, stmt.value)
        is B2Ast.Statement.VarDecl -> declVar(stmt.ident, stmt.type)
        is B2Ast.Statement.VarReAssign -> reAssignVar(stmt.ident, eval(stmt.newValue))
        is B2Ast.Statement.While -> runLoop {
            val pred = { eval(stmt.condition).value() as Boolean }
            while (pred()) {
                stmt.body.forEach { parse(it) }
            }
        }

        is B2Ast.Statement.Expr -> {
            eval(stmt.expr)
            Unit
        }

        is B2Ast.Statement.TypeAlias -> declType(stmt)
        is B2Ast.Statement.Unpack -> when (val x = eval(stmt.expr)) {
            is Val.Array -> stmt.idents.zip(x.toIter()).forEach { (id, e) -> declAssignVar(id, null, e) }
            is Val.Tuple -> stmt.idents.zip(x.unpack()).forEach { (id, e) -> declAssignVar(id, null, e.toLit()) }
            else -> throw RuntimeException("Cannot unpack $x")
        }
    }

    fun eval(expr: B2Ast.Expression): Val = when (expr) {
        is Arr -> Val.wrap(expr)
        is ArrInd -> {
            val arr = eval(expr.arr)
            val ind = eval(expr.ind)

            arr[ind]
        }

        is BinOp -> when (expr.operator) {
            B2Ast.Operator.Bi.Add -> eval(expr.left) + eval(expr.right)
            B2Ast.Operator.Bi.AddMut -> {
                val id = if (expr.left is Var) {
                    expr.left.ident
                } else {
                    null
                }
                val left = eval(expr.left)
                val right = eval(expr.right)

                val bop: (Val, Val) -> Val = when (expr.operator) {
                    B2Ast.Operator.Bi.AddMut -> {
                        { l, r -> l + r }
                    }

                    B2Ast.Operator.Bi.DivMut -> TODO()
                    B2Ast.Operator.Bi.MulMut -> TODO()
                    B2Ast.Operator.Bi.SubMut -> TODO()
                    else -> TODO()
                }

                val newValue = bop(left, right)

                id?.let {
                    reAssignVar(it, newValue)
                }

                newValue
            }

            B2Ast.Operator.Bi.Div -> TODO()
            B2Ast.Operator.Bi.DivMut -> TODO()
            B2Ast.Operator.Bi.Eq -> TODO()
            B2Ast.Operator.Bi.Geq -> TODO()
            B2Ast.Operator.Bi.Gt -> TODO()
            B2Ast.Operator.Bi.Leq -> TODO()
            B2Ast.Operator.Bi.Lt -> TODO()
            B2Ast.Operator.Bi.Mul -> TODO()
            B2Ast.Operator.Bi.MulMut -> TODO()
            B2Ast.Operator.Bi.Neq -> TODO()
            B2Ast.Operator.Bi.Sub -> TODO()
            B2Ast.Operator.Bi.SubMut -> TODO()
        }

        is Bol -> Val.wrap(expr)
        is Cast -> eval(expr.expr).cast(expr.type)
        is Flt -> Val.wrap(expr)
        is FnCall -> invokeFn(expr.ident, expr.args)
        is Group -> eval(expr.value)
        is Input -> {
            print(expr.prompt?.let { eval(it).value().toString() } ?: "")
            Val.Str(readlnOrNull() ?: "")
        }

        is B2Ast.Expression.Int -> Val.Int(expr.value)
        is Len -> when (val e = eval(expr.value)) {
            is Val.Array -> Val.Int(e.value.size)
            is Val.Str -> Val.Int(e.value.length)
            else -> throw RuntimeException("Cannot do length of ${e.toLit().type()}")
        }

        is B2Ast.Expression.NoOp -> Val.Unit
        is Str -> Val.wrap(expr.value)
        is Trim -> Val.wrap(expr.value.value.trim())
        is Tup -> Val.Tuple(eval(expr.fst), eval(expr.snd))
        is UniOp -> {
            val op: (Val) -> Val = when (expr.operator) {
                B2Ast.Operator.Uni.Decr -> {
                    { v -> v.decr() }
                }

                B2Ast.Operator.Uni.Incr -> {
                    { v -> v.incr() }
                }

                B2Ast.Operator.Uni.Sign -> {
                    { v -> v.sign() }
                }
            }
            op(eval(expr.value))
        }

        is Var -> when (val r = getVar(expr.ident)) {
            is Var if (expr.ident == r.ident) -> throw RuntimeException("Recursive declaration: $r")
            else -> eval(r)
        }
    }
}
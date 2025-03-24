package b2.compiler

import no.nilsmf.antlr.Basic2BaseVisitor
import no.nilsmf.antlr.Basic2Parser

data class Program(
    val name: String,
)

sealed class Statement {

}

class B2Compiler : Basic2BaseVisitor<Any?>() {
    override fun defaultResult(): Any? {
        TODO("Not yet implemented")
    }

    override fun visitProgram(ctx: Basic2Parser.ProgramContext): Any? {
        return super.visitProgram(ctx)
    }

    override fun visitIf_else_block(ctx: Basic2Parser.If_else_blockContext): Any? {
        return super.visitIf_else_block(ctx)
    }

    override fun visitIf_else(ctx: Basic2Parser.If_elseContext): Any? {
        return super.visitIf_else(ctx)
    }

    override fun visitIf_elif_block(ctx: Basic2Parser.If_elif_blockContext): Any? {
        return super.visitIf_elif_block(ctx)
    }

    override fun visitIf_elif(ctx: Basic2Parser.If_elifContext): Any? {
        return super.visitIf_elif(ctx)
    }

    override fun visitIf_block(ctx: Basic2Parser.If_blockContext): Any? {
        return super.visitIf_block(ctx)
    }

    override fun visitIf(ctx: Basic2Parser.IfContext): Any? {
        return super.visitIf(ctx)
    }

    override fun visitWhile(ctx: Basic2Parser.WhileContext): Any? {
        return super.visitWhile(ctx)
    }

    override fun visitWhile_block(ctx: Basic2Parser.While_blockContext): Any? {
        return super.visitWhile_block(ctx)
    }

    override fun visitFor_r(ctx: Basic2Parser.For_rContext): Any? {
        return super.visitFor_r(ctx)
    }

    override fun visitFor(ctx: Basic2Parser.ForContext): Any? {
        return super.visitFor(ctx)
    }

    override fun visitRet(ctx: Basic2Parser.RetContext): Any? {
        return super.visitRet(ctx)
    }

    override fun visitBreak(ctx: Basic2Parser.BreakContext): Any? {
        return super.visitBreak(ctx)
    }

    override fun visitContinue(ctx: Basic2Parser.ContinueContext): Any? {
        return super.visitContinue(ctx)
    }

    override fun visitPrint(ctx: Basic2Parser.PrintContext): Any? {
        return super.visitPrint(ctx)
    }

    override fun visitAppend(ctx: Basic2Parser.AppendContext): Any? {
        return super.visitAppend(ctx)
    }

    override fun visitArrReAss(ctx: Basic2Parser.ArrReAssContext): Any? {
        return super.visitArrReAss(ctx)
    }

    override fun visitVar_decl(ctx: Basic2Parser.Var_declContext): Any? {
        return super.visitVar_decl(ctx)
    }

    override fun visitVar_ass(ctx: Basic2Parser.Var_assContext): Any? {
        return super.visitVar_ass(ctx)
    }

    override fun visitVar_re_ass(ctx: Basic2Parser.Var_re_assContext): Any? {
        return super.visitVar_re_ass(ctx)
    }

    override fun visitBlock(ctx: Basic2Parser.BlockContext): Any? {
        return super.visitBlock(ctx)
    }

    override fun visitFn_decl(ctx: Basic2Parser.Fn_declContext): Any? {
        return super.visitFn_decl(ctx)
    }

    override fun visitFn_impl(ctx: Basic2Parser.Fn_implContext): Any? {
        return super.visitFn_impl(ctx)
    }

    override fun visitUse(ctx: Basic2Parser.UseContext): Any? {
        return super.visitUse(ctx)
    }

    override fun visitPreIncr(ctx: Basic2Parser.PreIncrContext): Any? {
        return super.visitPreIncr(ctx)
    }

    override fun visitPostIncr(ctx: Basic2Parser.PostIncrContext): Any? {
        return super.visitPostIncr(ctx)
    }

    override fun visitBinopIncr(ctx: Basic2Parser.BinopIncrContext): Any? {
        return super.visitBinopIncr(ctx)
    }

    override fun visitReturn_stmt(ctx: Basic2Parser.Return_stmtContext): Any? {
        return super.visitReturn_stmt(ctx)
    }

    override fun visitBreak_stmt(ctx: Basic2Parser.Break_stmtContext): Any? {
        return super.visitBreak_stmt(ctx)
    }

    override fun visitContinue_stmt(ctx: Basic2Parser.Continue_stmtContext): Any? {
        return super.visitContinue_stmt(ctx)
    }

    override fun visitPrint_stmt(ctx: Basic2Parser.Print_stmtContext): Any? {
        return super.visitPrint_stmt(ctx)
    }

    override fun visitAppend_stmt(ctx: Basic2Parser.Append_stmtContext): Any? {
        return super.visitAppend_stmt(ctx)
    }

    override fun visitArr_re_ass_stmt(ctx: Basic2Parser.Arr_re_ass_stmtContext): Any? {
        return super.visitArr_re_ass_stmt(ctx)
    }

    override fun visitUseAll(ctx: Basic2Parser.UseAllContext): Any? {
        return super.visitUseAll(ctx)
    }

    override fun visitUseSpecific(ctx: Basic2Parser.UseSpecificContext): Any? {
        return super.visitUseSpecific(ctx)
    }

    override fun visitRenaming(ctx: Basic2Parser.RenamingContext): Any? {
        return super.visitRenaming(ctx)
    }

    override fun visitFnDecl(ctx: Basic2Parser.FnDeclContext): Any? {
        return super.visitFnDecl(ctx)
    }

    override fun visitFnImpl(ctx: Basic2Parser.FnImplContext): Any? {
        return super.visitFnImpl(ctx)
    }

    override fun visitFnDeclImpl(ctx: Basic2Parser.FnDeclImplContext): Any? {
        return super.visitFnDeclImpl(ctx)
    }

    override fun visitVar(ctx: Basic2Parser.VarContext): Any? {
        return super.visitVar(ctx)
    }

    override fun visitVar_decl_stmt(ctx: Basic2Parser.Var_decl_stmtContext): Any? {
        return super.visitVar_decl_stmt(ctx)
    }

    override fun visitVar_decl_ass_stmt(ctx: Basic2Parser.Var_decl_ass_stmtContext): Any? {
        return super.visitVar_decl_ass_stmt(ctx)
    }

    override fun visitVar_re_ass_stmt(ctx: Basic2Parser.Var_re_ass_stmtContext): Any? {
        return super.visitVar_re_ass_stmt(ctx)
    }

    override fun visitTyping(ctx: Basic2Parser.TypingContext): Any? {
        return super.visitTyping(ctx)
    }

    override fun visitBlock_stmt(ctx: Basic2Parser.Block_stmtContext): Any? {
        return super.visitBlock_stmt(ctx)
    }

    override fun visitFn_decl_stmt(ctx: Basic2Parser.Fn_decl_stmtContext): Any? {
        return super.visitFn_decl_stmt(ctx)
    }

    override fun visitFn_impl_stmt(ctx: Basic2Parser.Fn_impl_stmtContext): Any? {
        return super.visitFn_impl_stmt(ctx)
    }

    override fun visitFn_param(ctx: Basic2Parser.Fn_paramContext): Any? {
        return super.visitFn_param(ctx)
    }

    override fun visitIf_elif_stmt(ctx: Basic2Parser.If_elif_stmtContext): Any? {
        return super.visitIf_elif_stmt(ctx)
    }

    override fun visitIf_elif_stmt_block(ctx: Basic2Parser.If_elif_stmt_blockContext): Any? {
        return super.visitIf_elif_stmt_block(ctx)
    }

    override fun visitElifBlockBranch(ctx: Basic2Parser.ElifBlockBranchContext): Any? {
        return super.visitElifBlockBranch(ctx)
    }

    override fun visitIf_else_stmt(ctx: Basic2Parser.If_else_stmtContext): Any? {
        return super.visitIf_else_stmt(ctx)
    }

    override fun visitIf_else_stmt_block(ctx: Basic2Parser.If_else_stmt_blockContext): Any? {
        return super.visitIf_else_stmt_block(ctx)
    }

    override fun visitIfThenBlock(ctx: Basic2Parser.IfThenBlockContext): Any? {
        return super.visitIfThenBlock(ctx)
    }

    override fun visitIfElseBlock(ctx: Basic2Parser.IfElseBlockContext): Any? {
        return super.visitIfElseBlock(ctx)
    }

    override fun visitIf_stmt(ctx: Basic2Parser.If_stmtContext): Any? {
        return super.visitIf_stmt(ctx)
    }

    override fun visitIf_stmt_block(ctx: Basic2Parser.If_stmt_blockContext): Any? {
        return super.visitIf_stmt_block(ctx)
    }

    override fun visitWhile_stmt(ctx: Basic2Parser.While_stmtContext): Any? {
        return super.visitWhile_stmt(ctx)
    }

    override fun visitWhile_stmt_block(ctx: Basic2Parser.While_stmt_blockContext): Any? {
        return super.visitWhile_stmt_block(ctx)
    }

    override fun visitFor_stmt(ctx: Basic2Parser.For_stmtContext): Any? {
        return super.visitFor_stmt(ctx)
    }

    override fun visitFor_range(ctx: Basic2Parser.For_rangeContext): Any? {
        return super.visitFor_range(ctx)
    }

    override fun visitComp(ctx: Basic2Parser.CompContext): Any? {
        return super.visitComp(ctx)
    }

    override fun visitIncr(ctx: Basic2Parser.IncrContext): Any? {
        return super.visitIncr(ctx)
    }

    override fun visitIncr_uni(ctx: Basic2Parser.Incr_uniContext): Any? {
        return super.visitIncr_uni(ctx)
    }

    override fun visitType(ctx: Basic2Parser.TypeContext): Any? {
        return super.visitType(ctx)
    }

    override fun visitIterable(ctx: Basic2Parser.IterableContext): Any? {
        return super.visitIterable(ctx)
    }

    override fun visitBin_op(ctx: Basic2Parser.Bin_opContext): Any? {
        return super.visitBin_op(ctx)
    }

    override fun visitBool(ctx: Basic2Parser.BoolContext): Any? {
        return super.visitBool(ctx)
    }

    override fun visitIdent(ctx: Basic2Parser.IdentContext): Any? {
        return super.visitIdent(ctx)
    }

    override fun visitNum(ctx: Basic2Parser.NumContext): Any? {
        return super.visitNum(ctx)
    }

    override fun visitFloat(ctx: Basic2Parser.FloatContext): Any? {
        return super.visitFloat(ctx)
    }

    override fun visitStr(ctx: Basic2Parser.StrContext): Any? {
        return super.visitStr(ctx)
    }

    override fun visitTuple(ctx: Basic2Parser.TupleContext): Any? {
        return super.visitTuple(ctx)
    }

    override fun visitInput(ctx: Basic2Parser.InputContext): Any? {
        return super.visitInput(ctx)
    }

    override fun visitCast(ctx: Basic2Parser.CastContext): Any? {
        return super.visitCast(ctx)
    }

    override fun visitTrim(ctx: Basic2Parser.TrimContext): Any? {
        return super.visitTrim(ctx)
    }

    override fun visitLen(ctx: Basic2Parser.LenContext): Any? {
        return super.visitLen(ctx)
    }

    override fun visitArray(ctx: Basic2Parser.ArrayContext): Any? {
        return super.visitArray(ctx)
    }

    override fun visitBinopComp(ctx: Basic2Parser.BinopCompContext): Any? {
        return super.visitBinopComp(ctx)
    }

    override fun visitFnCall(ctx: Basic2Parser.FnCallContext): Any? {
        return super.visitFnCall(ctx)
    }

    override fun visitTernary(ctx: Basic2Parser.TernaryContext): Any? {
        return super.visitTernary(ctx)
    }

    override fun visitBinop(ctx: Basic2Parser.BinopContext): Any? {
        return super.visitBinop(ctx)
    }

    override fun visitGroup(ctx: Basic2Parser.GroupContext): Any? {
        return super.visitGroup(ctx)
    }

    override fun visitArrInd(ctx: Basic2Parser.ArrIndContext): Any? {
        return super.visitArrInd(ctx)
    }

    override fun visitInputExpr(ctx: Basic2Parser.InputExprContext): Any? {
        return super.visitInputExpr(ctx)
    }

    override fun visitLenExpr(ctx: Basic2Parser.LenExprContext): Any? {
        return super.visitLenExpr(ctx)
    }
}
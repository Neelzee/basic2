import org.bytedeco.javacpp.BytePointer
import org.bytedeco.javacpp.Pointer
import org.bytedeco.javacpp.PointerPointer
import org.bytedeco.llvm.LLVM.LLVMExecutionEngineRef
import org.bytedeco.llvm.LLVM.LLVMMCJITCompilerOptions
import org.bytedeco.llvm.global.LLVM

fun main() {
    val error = BytePointer()

    // Stage 1: Initialize LLVM components
    LLVM.LLVMInitializeNativeTarget()
    LLVM.LLVMInitializeNativeAsmPrinter()

    // Stage 2: Build the factorial function.
    val context = LLVM.LLVMContextCreate()
    val module = LLVM.LLVMModuleCreateWithNameInContext("factorial", context)
    val builder = LLVM.LLVMCreateBuilderInContext(context)
    val i32Type = LLVM.LLVMInt32TypeInContext(context)
    val factorialType = LLVM.LLVMFunctionType(i32Type, i32Type,  /* argumentCount */1,  /* isVariadic */0)

    val factorial = LLVM.LLVMAddFunction(module, "factorial", factorialType)
    LLVM.LLVMSetFunctionCallConv(factorial, LLVM.LLVMCCallConv)

    val n = LLVM.LLVMGetParam(factorial,  /* parameterIndex */0)
    val zero = LLVM.LLVMConstInt(i32Type, 0,  /* signExtend */0)
    val one = LLVM.LLVMConstInt(i32Type, 1,  /* signExtend */0)
    val entry = LLVM.LLVMAppendBasicBlockInContext(context, factorial, "entry")
    val ifFalse = LLVM.LLVMAppendBasicBlockInContext(context, factorial, "if_false")
    val exit = LLVM.LLVMAppendBasicBlockInContext(context, factorial, "exit")

    LLVM.LLVMPositionBuilderAtEnd(builder, entry)
    val condition = LLVM.LLVMBuildICmp(builder, LLVM.LLVMIntEQ, n, zero, "condition = n == 0")
    LLVM.LLVMBuildCondBr(builder, condition, exit, ifFalse)

    LLVM.LLVMPositionBuilderAtEnd(builder, ifFalse)
    val nMinusOne = LLVM.LLVMBuildSub(builder, n, one, "nMinusOne = n - 1")
    val arguments = PointerPointer<Pointer?>(1)
        .put(0, nMinusOne)
    val factorialResult = LLVM.LLVMBuildCall2(
        builder,
        factorialType,
        factorial,
        arguments,
        1,
        "factorialResult = factorial(nMinusOne)"
    )
    val resultIfFalse = LLVM.LLVMBuildMul(builder, n, factorialResult, "resultIfFalse = n * factorialResult")
    LLVM.LLVMBuildBr(builder, exit)

    LLVM.LLVMPositionBuilderAtEnd(builder, exit)
    val phi = LLVM.LLVMBuildPhi(builder, i32Type, "result")
    val phiValues = PointerPointer<Pointer?>(2)
        .put(0, one)
        .put(1, resultIfFalse)
    val phiBlocks = PointerPointer<Pointer?>(2)
        .put(0, entry)
        .put(1, ifFalse)
    LLVM.LLVMAddIncoming(phi, phiValues, phiBlocks,  /* pairCount */2)
    LLVM.LLVMBuildRet(builder, phi)

    // Print generated LLVM-IR to console (optional)
    LLVM.LLVMDumpModule(module)

    // Stage 3: Verify the module using LLVMVerifier
    if (LLVM.LLVMVerifyModule(module, LLVM.LLVMPrintMessageAction, error) != 0) {
        LLVM.LLVMDisposeMessage(error)
        return
    }

    // Stage 4: Create a pass pipeline using the legacy pass manager
    val pm = LLVM.LLVMCreatePassManager()
    LLVM.LLVMRunPassManager(pm, module)

    // Stage 5: Execute the code using MCJIT
    val engine = LLVMExecutionEngineRef()
    val options = LLVMMCJITCompilerOptions()
    if (LLVM.LLVMCreateMCJITCompilerForModule(engine, module, options, 3, error) != 0) {
        System.err.println("Failed to create JIT compiler: " + error.getString())
        LLVM.LLVMDisposeMessage(error)
        return
    }

    val argument = LLVM.LLVMCreateGenericValueOfInt(i32Type, 10,  /* signExtend */0)
    val result = LLVM.LLVMRunFunction(engine, factorial,  /* argumentCount */1, argument)
    println()
    println("Running factorial(10) with MCJIT...")
    println("Result: " + LLVM.LLVMGenericValueToInt(result,  /* signExtend */0))

    // Stage 6: Dispose of the allocated resources
    LLVM.LLVMDisposeExecutionEngine(engine)
    LLVM.LLVMDisposePassManager(pm)
    LLVM.LLVMDisposeBuilder(builder)
    LLVM.LLVMContextDispose(context)
}
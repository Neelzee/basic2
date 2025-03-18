// Generated from /home/nmf/Documents/basic2/src/main/antlr/Basic2Tokens.g4 by ANTLR 4.13.2
import org.antlr.v4.runtime.Lexer;
import org.antlr.v4.runtime.CharStream;
import org.antlr.v4.runtime.Token;
import org.antlr.v4.runtime.TokenStream;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.misc.*;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast", "CheckReturnValue", "this-escape"})
public class Basic2Tokens extends Lexer {
	static { RuntimeMetaData.checkVersion("4.13.2", RuntimeMetaData.VERSION); }

	protected static final DFA[] _decisionToDFA;
	protected static final PredictionContextCache _sharedContextCache =
		new PredictionContextCache();
	public static final int
		WHITESPACE=1, STR_LIT=2, NUM_LIT=3, FLOAT_LIT=4, BOOL_LIT=5, TUPLE_STRT=6, 
		TUPLE_END=7, ARRAY_STRT=8, ARRAY_END=9, BODY_STRT=10, BODY_END=11, SEP=12, 
		PRINT_KW=13, INPUT_KW=14, FUNCTION_DECL=15, FUNCTION_IMPL=16, END_KW=17, 
		LET_KW=18, ASS_KW=19, PRIM_TYPES=20, IDENTIFIER=21;
	public static String[] channelNames = {
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	};

	public static String[] modeNames = {
		"DEFAULT_MODE"
	};

	private static String[] makeRuleNames() {
		return new String[] {
			"WHITESPACE", "STR_LIT", "NUM_LIT", "FLOAT_LIT", "BOOL_LIT", "TUPLE_STRT", 
			"TUPLE_END", "ARRAY_STRT", "ARRAY_END", "BODY_STRT", "BODY_END", "CHARS", 
			"SEP", "PRINT_KW", "INPUT_KW", "FUNCTION_DECL", "FUNCTION_IMPL", "END_KW", 
			"LET_KW", "ASS_KW", "PRIM_TYPES", "IDENTIFIER"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, null, null, null, null, null, "'('", "')'", "'['", "']'", "'{'", 
			"'}'", "','", "'PRINT'", "'INPUT'", "'DECL'", "'IMPL'", "';'", "'LET'", 
			"'='"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, "WHITESPACE", "STR_LIT", "NUM_LIT", "FLOAT_LIT", "BOOL_LIT", "TUPLE_STRT", 
			"TUPLE_END", "ARRAY_STRT", "ARRAY_END", "BODY_STRT", "BODY_END", "SEP", 
			"PRINT_KW", "INPUT_KW", "FUNCTION_DECL", "FUNCTION_IMPL", "END_KW", "LET_KW", 
			"ASS_KW", "PRIM_TYPES", "IDENTIFIER"
		};
	}
	private static final String[] _SYMBOLIC_NAMES = makeSymbolicNames();
	public static final Vocabulary VOCABULARY = new VocabularyImpl(_LITERAL_NAMES, _SYMBOLIC_NAMES);

	/**
	 * @deprecated Use {@link #VOCABULARY} instead.
	 */
	@Deprecated
	public static final String[] tokenNames;
	static {
		tokenNames = new String[_SYMBOLIC_NAMES.length];
		for (int i = 0; i < tokenNames.length; i++) {
			tokenNames[i] = VOCABULARY.getLiteralName(i);
			if (tokenNames[i] == null) {
				tokenNames[i] = VOCABULARY.getSymbolicName(i);
			}

			if (tokenNames[i] == null) {
				tokenNames[i] = "<INVALID>";
			}
		}
	}

	@Override
	@Deprecated
	public String[] getTokenNames() {
		return tokenNames;
	}

	@Override

	public Vocabulary getVocabulary() {
		return VOCABULARY;
	}


	public Basic2Tokens(CharStream input) {
		super(input);
		_interp = new LexerATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	@Override
	public String getGrammarFileName() { return "Basic2Tokens.g4"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public String[] getChannelNames() { return channelNames; }

	@Override
	public String[] getModeNames() { return modeNames; }

	@Override
	public ATN getATN() { return _ATN; }

	public static final String _serializedATN =
		"\u0004\u0000\u0015\u00af\u0006\uffff\uffff\u0002\u0000\u0007\u0000\u0002"+
		"\u0001\u0007\u0001\u0002\u0002\u0007\u0002\u0002\u0003\u0007\u0003\u0002"+
		"\u0004\u0007\u0004\u0002\u0005\u0007\u0005\u0002\u0006\u0007\u0006\u0002"+
		"\u0007\u0007\u0007\u0002\b\u0007\b\u0002\t\u0007\t\u0002\n\u0007\n\u0002"+
		"\u000b\u0007\u000b\u0002\f\u0007\f\u0002\r\u0007\r\u0002\u000e\u0007\u000e"+
		"\u0002\u000f\u0007\u000f\u0002\u0010\u0007\u0010\u0002\u0011\u0007\u0011"+
		"\u0002\u0012\u0007\u0012\u0002\u0013\u0007\u0013\u0002\u0014\u0007\u0014"+
		"\u0002\u0015\u0007\u0015\u0001\u0000\u0001\u0000\u0001\u0000\u0001\u0000"+
		"\u0001\u0001\u0001\u0001\u0005\u00014\b\u0001\n\u0001\f\u00017\t\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0002\u0003\u0002<\b\u0002\u0001\u0002"+
		"\u0001\u0002\u0005\u0002@\b\u0002\n\u0002\f\u0002C\t\u0002\u0001\u0003"+
		"\u0003\u0003F\b\u0003\u0001\u0003\u0005\u0003I\b\u0003\n\u0003\f\u0003"+
		"L\t\u0003\u0001\u0003\u0003\u0003O\b\u0003\u0001\u0003\u0005\u0003R\b"+
		"\u0003\n\u0003\f\u0003U\t\u0003\u0001\u0003\u0001\u0003\u0001\u0004\u0001"+
		"\u0004\u0001\u0004\u0001\u0004\u0001\u0004\u0001\u0004\u0001\u0004\u0001"+
		"\u0004\u0001\u0004\u0003\u0004b\b\u0004\u0001\u0005\u0001\u0005\u0001"+
		"\u0006\u0001\u0006\u0001\u0007\u0001\u0007\u0001\b\u0001\b\u0001\t\u0001"+
		"\t\u0001\n\u0001\n\u0001\u000b\u0001\u000b\u0001\f\u0001\f\u0001\r\u0001"+
		"\r\u0001\r\u0001\r\u0001\r\u0001\r\u0001\u000e\u0001\u000e\u0001\u000e"+
		"\u0001\u000e\u0001\u000e\u0001\u000e\u0001\u000f\u0001\u000f\u0001\u000f"+
		"\u0001\u000f\u0001\u000f\u0001\u0010\u0001\u0010\u0001\u0010\u0001\u0010"+
		"\u0001\u0010\u0001\u0011\u0001\u0011\u0001\u0012\u0001\u0012\u0001\u0012"+
		"\u0001\u0012\u0001\u0013\u0001\u0013\u0001\u0014\u0001\u0014\u0001\u0014"+
		"\u0001\u0014\u0001\u0014\u0001\u0014\u0001\u0014\u0001\u0014\u0001\u0014"+
		"\u0001\u0014\u0001\u0014\u0001\u0014\u0001\u0014\u0001\u0014\u0001\u0014"+
		"\u0003\u0014\u00a1\b\u0014\u0001\u0015\u0001\u0015\u0004\u0015\u00a5\b"+
		"\u0015\u000b\u0015\f\u0015\u00a6\u0001\u0015\u0001\u0015\u0005\u0015\u00ab"+
		"\b\u0015\n\u0015\f\u0015\u00ae\t\u0015\u0000\u0000\u0016\u0001\u0001\u0003"+
		"\u0002\u0005\u0003\u0007\u0004\t\u0005\u000b\u0006\r\u0007\u000f\b\u0011"+
		"\t\u0013\n\u0015\u000b\u0017\u0000\u0019\f\u001b\r\u001d\u000e\u001f\u000f"+
		"!\u0010#\u0011%\u0012\'\u0013)\u0014+\u0015\u0001\u0000\u0006\u0003\u0000"+
		"\t\n\r\r  \u0001\u0000\"\"\u0001\u000009\u0002\u0000AZaz\u0002\u0000-"+
		"-__\u0003\u0000--09__\u00bc\u0000\u0001\u0001\u0000\u0000\u0000\u0000"+
		"\u0003\u0001\u0000\u0000\u0000\u0000\u0005\u0001\u0000\u0000\u0000\u0000"+
		"\u0007\u0001\u0000\u0000\u0000\u0000\t\u0001\u0000\u0000\u0000\u0000\u000b"+
		"\u0001\u0000\u0000\u0000\u0000\r\u0001\u0000\u0000\u0000\u0000\u000f\u0001"+
		"\u0000\u0000\u0000\u0000\u0011\u0001\u0000\u0000\u0000\u0000\u0013\u0001"+
		"\u0000\u0000\u0000\u0000\u0015\u0001\u0000\u0000\u0000\u0000\u0019\u0001"+
		"\u0000\u0000\u0000\u0000\u001b\u0001\u0000\u0000\u0000\u0000\u001d\u0001"+
		"\u0000\u0000\u0000\u0000\u001f\u0001\u0000\u0000\u0000\u0000!\u0001\u0000"+
		"\u0000\u0000\u0000#\u0001\u0000\u0000\u0000\u0000%\u0001\u0000\u0000\u0000"+
		"\u0000\'\u0001\u0000\u0000\u0000\u0000)\u0001\u0000\u0000\u0000\u0000"+
		"+\u0001\u0000\u0000\u0000\u0001-\u0001\u0000\u0000\u0000\u00031\u0001"+
		"\u0000\u0000\u0000\u0005;\u0001\u0000\u0000\u0000\u0007E\u0001\u0000\u0000"+
		"\u0000\ta\u0001\u0000\u0000\u0000\u000bc\u0001\u0000\u0000\u0000\re\u0001"+
		"\u0000\u0000\u0000\u000fg\u0001\u0000\u0000\u0000\u0011i\u0001\u0000\u0000"+
		"\u0000\u0013k\u0001\u0000\u0000\u0000\u0015m\u0001\u0000\u0000\u0000\u0017"+
		"o\u0001\u0000\u0000\u0000\u0019q\u0001\u0000\u0000\u0000\u001bs\u0001"+
		"\u0000\u0000\u0000\u001dy\u0001\u0000\u0000\u0000\u001f\u007f\u0001\u0000"+
		"\u0000\u0000!\u0084\u0001\u0000\u0000\u0000#\u0089\u0001\u0000\u0000\u0000"+
		"%\u008b\u0001\u0000\u0000\u0000\'\u008f\u0001\u0000\u0000\u0000)\u00a0"+
		"\u0001\u0000\u0000\u0000+\u00a4\u0001\u0000\u0000\u0000-.\u0007\u0000"+
		"\u0000\u0000./\u0001\u0000\u0000\u0000/0\u0006\u0000\u0000\u00000\u0002"+
		"\u0001\u0000\u0000\u000015\u0005\"\u0000\u000024\b\u0001\u0000\u00003"+
		"2\u0001\u0000\u0000\u000047\u0001\u0000\u0000\u000053\u0001\u0000\u0000"+
		"\u000056\u0001\u0000\u0000\u000068\u0001\u0000\u0000\u000075\u0001\u0000"+
		"\u0000\u000089\u0005\"\u0000\u00009\u0004\u0001\u0000\u0000\u0000:<\u0005"+
		"-\u0000\u0000;:\u0001\u0000\u0000\u0000;<\u0001\u0000\u0000\u0000<=\u0001"+
		"\u0000\u0000\u0000=A\u0007\u0002\u0000\u0000>@\u0007\u0002\u0000\u0000"+
		"?>\u0001\u0000\u0000\u0000@C\u0001\u0000\u0000\u0000A?\u0001\u0000\u0000"+
		"\u0000AB\u0001\u0000\u0000\u0000B\u0006\u0001\u0000\u0000\u0000CA\u0001"+
		"\u0000\u0000\u0000DF\u0005-\u0000\u0000ED\u0001\u0000\u0000\u0000EF\u0001"+
		"\u0000\u0000\u0000FJ\u0001\u0000\u0000\u0000GI\u0007\u0002\u0000\u0000"+
		"HG\u0001\u0000\u0000\u0000IL\u0001\u0000\u0000\u0000JH\u0001\u0000\u0000"+
		"\u0000JK\u0001\u0000\u0000\u0000KN\u0001\u0000\u0000\u0000LJ\u0001\u0000"+
		"\u0000\u0000MO\u0005.\u0000\u0000NM\u0001\u0000\u0000\u0000NO\u0001\u0000"+
		"\u0000\u0000OS\u0001\u0000\u0000\u0000PR\u0007\u0002\u0000\u0000QP\u0001"+
		"\u0000\u0000\u0000RU\u0001\u0000\u0000\u0000SQ\u0001\u0000\u0000\u0000"+
		"ST\u0001\u0000\u0000\u0000TV\u0001\u0000\u0000\u0000US\u0001\u0000\u0000"+
		"\u0000VW\u0005f\u0000\u0000W\b\u0001\u0000\u0000\u0000XY\u0005T\u0000"+
		"\u0000YZ\u0005R\u0000\u0000Z[\u0005U\u0000\u0000[b\u0005E\u0000\u0000"+
		"\\]\u0005F\u0000\u0000]^\u0005A\u0000\u0000^_\u0005L\u0000\u0000_`\u0005"+
		"S\u0000\u0000`b\u0005E\u0000\u0000aX\u0001\u0000\u0000\u0000a\\\u0001"+
		"\u0000\u0000\u0000b\n\u0001\u0000\u0000\u0000cd\u0005(\u0000\u0000d\f"+
		"\u0001\u0000\u0000\u0000ef\u0005)\u0000\u0000f\u000e\u0001\u0000\u0000"+
		"\u0000gh\u0005[\u0000\u0000h\u0010\u0001\u0000\u0000\u0000ij\u0005]\u0000"+
		"\u0000j\u0012\u0001\u0000\u0000\u0000kl\u0005{\u0000\u0000l\u0014\u0001"+
		"\u0000\u0000\u0000mn\u0005}\u0000\u0000n\u0016\u0001\u0000\u0000\u0000"+
		"op\u0007\u0003\u0000\u0000p\u0018\u0001\u0000\u0000\u0000qr\u0005,\u0000"+
		"\u0000r\u001a\u0001\u0000\u0000\u0000st\u0005P\u0000\u0000tu\u0005R\u0000"+
		"\u0000uv\u0005I\u0000\u0000vw\u0005N\u0000\u0000wx\u0005T\u0000\u0000"+
		"x\u001c\u0001\u0000\u0000\u0000yz\u0005I\u0000\u0000z{\u0005N\u0000\u0000"+
		"{|\u0005P\u0000\u0000|}\u0005U\u0000\u0000}~\u0005T\u0000\u0000~\u001e"+
		"\u0001\u0000\u0000\u0000\u007f\u0080\u0005D\u0000\u0000\u0080\u0081\u0005"+
		"E\u0000\u0000\u0081\u0082\u0005C\u0000\u0000\u0082\u0083\u0005L\u0000"+
		"\u0000\u0083 \u0001\u0000\u0000\u0000\u0084\u0085\u0005I\u0000\u0000\u0085"+
		"\u0086\u0005M\u0000\u0000\u0086\u0087\u0005P\u0000\u0000\u0087\u0088\u0005"+
		"L\u0000\u0000\u0088\"\u0001\u0000\u0000\u0000\u0089\u008a\u0005;\u0000"+
		"\u0000\u008a$\u0001\u0000\u0000\u0000\u008b\u008c\u0005L\u0000\u0000\u008c"+
		"\u008d\u0005E\u0000\u0000\u008d\u008e\u0005T\u0000\u0000\u008e&\u0001"+
		"\u0000\u0000\u0000\u008f\u0090\u0005=\u0000\u0000\u0090(\u0001\u0000\u0000"+
		"\u0000\u0091\u0092\u0005I\u0000\u0000\u0092\u0093\u0005N\u0000\u0000\u0093"+
		"\u00a1\u0005T\u0000\u0000\u0094\u0095\u0005F\u0000\u0000\u0095\u0096\u0005"+
		"L\u0000\u0000\u0096\u0097\u0005O\u0000\u0000\u0097\u0098\u0005A\u0000"+
		"\u0000\u0098\u00a1\u0005T\u0000\u0000\u0099\u009a\u0005S\u0000\u0000\u009a"+
		"\u009b\u0005T\u0000\u0000\u009b\u00a1\u0005R\u0000\u0000\u009c\u009d\u0005"+
		"B\u0000\u0000\u009d\u009e\u0005O\u0000\u0000\u009e\u009f\u0005O\u0000"+
		"\u0000\u009f\u00a1\u0005L\u0000\u0000\u00a0\u0091\u0001\u0000\u0000\u0000"+
		"\u00a0\u0094\u0001\u0000\u0000\u0000\u00a0\u0099\u0001\u0000\u0000\u0000"+
		"\u00a0\u009c\u0001\u0000\u0000\u0000\u00a1*\u0001\u0000\u0000\u0000\u00a2"+
		"\u00a5\u0003\u0017\u000b\u0000\u00a3\u00a5\u0007\u0004\u0000\u0000\u00a4"+
		"\u00a2\u0001\u0000\u0000\u0000\u00a4\u00a3\u0001\u0000\u0000\u0000\u00a5"+
		"\u00a6\u0001\u0000\u0000\u0000\u00a6\u00a4\u0001\u0000\u0000\u0000\u00a6"+
		"\u00a7\u0001\u0000\u0000\u0000\u00a7\u00ac\u0001\u0000\u0000\u0000\u00a8"+
		"\u00ab\u0003\u0017\u000b\u0000\u00a9\u00ab\u0007\u0005\u0000\u0000\u00aa"+
		"\u00a8\u0001\u0000\u0000\u0000\u00aa\u00a9\u0001\u0000\u0000\u0000\u00ab"+
		"\u00ae\u0001\u0000\u0000\u0000\u00ac\u00aa\u0001\u0000\u0000\u0000\u00ac"+
		"\u00ad\u0001\u0000\u0000\u0000\u00ad,\u0001\u0000\u0000\u0000\u00ae\u00ac"+
		"\u0001\u0000\u0000\u0000\u000e\u00005;AEJNSa\u00a0\u00a4\u00a6\u00aa\u00ac"+
		"\u0001\u0006\u0000\u0000";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}
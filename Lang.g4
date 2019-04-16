grammar Lang;

// Keywords
LET: 'let';
FN: 'fn';
IMPL: 'impl';
FOR: 'for';
IF: 'if';
ELSE: 'else';
RETURN: 'return';
WHILE: 'while';
TRAIT: 'trait';
STRUCT: 'struct';
ENUM: 'enum';
IMPORT: 'import';
OR: 'or';
AND: 'and';
TRUE: 'true';
FALSE: 'false';

// TODO: how the hell do I lex user defined types??
TYPE:
	'i32'
	| 'i64'
	| 'f64'
	| 'f32'
	;
NUMBER: DIGIT+ (DOT DIGIT+)?;
DIGIT: [0-9];
IDENTIFIER: [A-Za-z0-9_]+;

// Other Symbols
RETURN_TYPE: '->';
COLON: ':';
RBRACE: '}';
LBRACE: '{';
RPAREN: ')';
LPAREN: '(';
ANY: '.*';
COMMA: ',';
BLOCK_COMMENT_BEGIN: '/*';
BLOCK_COMMENT_END: '*/';
LINE_COMMENT: '//';
SEMICOLON: ';';

PATH_SEPARATOR: '::';
STAR: '*';
EQUAL: '=';
DOT: '.';
EQUAL_EQUAL: '==';
BANG_EQUAL: '!=';
GREATER: '>';
GREATER_EQUAL: '>=';
LESS: '<';
LESS_EQUAL: '<=';
SUB: '-';
PLUS: '+';
DIV: '/';
BANG: '!';
RBRACKET: ']';
LBRACKET: '[';

DUB_QUOTE: '"';

STRING: '"' '.*'? '"';
WS: [ \t\n\r]+ -> skip;

program: declaration* EOF;

declaration:
	comment
	| structDecl
	| letDecl
	| enumDecl
	| functionDecl
	| statement
	| traitDecl
	| implTrait
	| implDecl;

statement:
	returnStatement
	| expressionStatement
	| forStatement
	| ifStatement
	| whileStatement
	| block
	| importStatement;

comment: blockComment | lineComment;

enumDecl: ENUM IDENTIFIER RBRACE enumItemList? LBRACE;

enumItemList: enumItem (COMMA enumItem)*;

enumItem: IDENTIFIER;

blockComment:
	BLOCK_COMMENT_BEGIN (blockComment | ANY) BLOCK_COMMENT_END;

lineComment: LINE_COMMENT ANY;

importStatement: IMPORT importPath SEMICOLON;

importPath: simplePath (PATH_SEPARATOR importPathSpecifier+)?;

importPathSpecifier:
	STAR
	| LBRACE IDENTIFIER (COMMA IDENTIFIER) RBRACE;

simplePath:
	PATH_SEPARATOR? pathSegment (PATH_SEPARATOR pathSegment)*;

pathSegment: IDENTIFIER;

expressionStatement: expression SEMICOLON;

implTrait:
	IMPL IDENTIFIER FOR IDENTIFIER LBRACE functionDecl* RBRACE;
forStatement:
	FOR LPAREN (letDecl | expressionStatement | SEMICOLON) expression? SEMICOLON expression? RPAREN
		statement;
ifStatement:
	IF LPAREN expression? RPAREN statement (ELSE statement)?;
returnStatement: RETURN expression? SEMICOLON;
whileStatement: WHILE LPAREN expression RPAREN statement;
block: LBRACE declaration? RBRACE;
traitDecl: TRAIT IDENTIFIER LBRACE traitFunctionDecl* RBRACE;
traitFunctionDecl:
	FN IDENTIFIER LPAREN parameters? RPAREN RETURN_TYPE TYPE SEMICOLON;
structDecl: STRUCT IDENTIFIER LBRACE structFields* RBRACE;
structFields: IDENTIFIER COLON TYPE COMMA?;
letDecl:
	LET IDENTIFIER COLON TYPE (EQUAL expression)? SEMICOLON;
implDecl: IMPL IDENTIFIER LBRACE functionDecl* RBRACE;
expression: assignment;
assignment: (call DOT)? IDENTIFIER EQUAL assignment | logicOr;
logicOr: logicAnd (OR logicAnd)*;
logicAnd: equality (AND equality)*;
equality: comparison ((BANG_EQUAL | EQUAL_EQUAL) comparison)*;
comparison:
	addition (
		(GREATER | GREATER_EQUAL | LESS | LESS_EQUAL) addition
	)*;
addition: multiplication ((SUB | PLUS) multiplication)*;
multiplication: unary ((DIV | STAR) unary)*;
unary: (BANG | SUB) unary | call;
call: primary (LPAREN arguments? RPAREN | DOT IDENTIFIER)*;
arguments: expression (COMMA expression)*;
primary:
	TRUE
	| FALSE
	| NUMBER
	| STRING 
	| LPAREN expression RPAREN
	| IDENTIFIER indexExpression
	| arrayExpr
	| IDENTIFIER
	| enumPrimary;
arrayExpr: LBRACKET arrayElements? RBRACKET;
arrayElements: (expression (COMMA expression)*);
indexExpression: LBRACKET expression RBRACKET;
enumPrimary: IDENTIFIER (PATH_SEPARATOR IDENTIFIER)*;
functionDecl: FN function;
function:
	IDENTIFIER LPAREN parameters? RPAREN RETURN_TYPE TYPE block;
parameters:
	IDENTIFIER COLON TYPE (COMMA IDENTIFIER COLON TYPE)*;

grammar LLMCore;

command: ACT OBJ QTY MOD paramBlock? EOF;

ACT:    'MOVE' | 'FETCH' | 'SEND' | 'ANALYZE' ;
OBJ:    'UI' | 'DATA' | 'MEDIA' | 'ROBOT' ;
QTY:    'ONE' | 'ALL' | 'TOP' | 'RANGE' ;
MOD:    'FAST' | 'SAFE' | 'RECURSE' ;
paramBlock: '[' params? ']';
params:    .+? ; // placeholder for JSON-like params

WS: [ \t\r\n]+ -> skip;

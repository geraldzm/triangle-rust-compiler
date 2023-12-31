

 pub trait Visitor {
 
   // Commands
//    fn visit_assign_command<T, U>(AssignCommand ast, t: T) -> U;
//    fn visitCallCommand<T, U>(CallCommand ast, t: T) -> U;
//    fn visitEmptyCommand<T, U>(EmptyCommand ast, t: T) -> U;
//    fn visitIfCommand<T, U>(IfCommand ast, t: T) -> U;
//    fn visitLetCommand<T, U>(LetCommand ast, t: T) -> U;
//    fn visitSequentialCommand<T, U>(SequentialCommand ast, t: T) -> U;
//    fn visitWhileCommand<T, U>(WhileCommand ast, t: T) -> U;
 
 
//    // Expressions
//    fn visitArrayExpression<T, U>(ArrayExpression ast, t: T) -> U;
//    fn visitBinaryExpression<T, U>(BinaryExpression ast, t: T) -> U;
//    fn visitCallExpression<T, U>(CallExpression ast, t: T) -> U;
//    fn visitCharacterExpression<T, U>(CharacterExpression ast, t: T) -> U;
//    fn visitEmptyExpression<T, U>(EmptyExpression ast, t: T) -> U;
//    fn visitIfExpression<T, U>(IfExpression ast, t: T) -> U;
//    fn visitIntegerExpression<T, U>(IntegerExpression ast, t: T) -> U;
//    fn visitLetExpression<T, U>(LetExpression ast, t: T) -> U;
//    fn visitRecordExpression<T, U>(RecordExpression ast, t: T) -> U;
//    fn visitUnaryExpression<T, U>(UnaryExpression ast, t: T) -> U;
//    fn visitVnameExpression<T, U>(VnameExpression ast, t: T) -> U;
 
//    // Declarations
//    fn visitBinaryOperatorDeclaration<T, U>(BinaryOperatorDeclaration ast, t: T) -> U;
//    fn visitConstDeclaration<T, U>(ConstDeclaration ast, t: T) -> U;
//    fn visitFuncDeclaration<T, U>(FuncDeclaration ast, t: T) -> U;
//    fn visitProcDeclaration<T, U>(ProcDeclaration ast, t: T) -> U;
//    fn visitSequentialDeclaration<T, U>(SequentialDeclaration ast, t: T) -> U;
//    fn visitTypeDeclaration<T, U>(TypeDeclaration ast, t: T) -> U;
//    fn visitUnaryOperatorDeclaration<T, U>(UnaryOperatorDeclaration ast, t: T) -> U;
//    fn visitVarDeclaration<T, U>(VarDeclaration ast, t: T) -> U;
 
//    // Array Aggregates
//    fn visitMultipleArrayAggregate<T, U>(MultipleArrayAggregate ast, t: T) -> U;
//    fn visitSingleArrayAggregate<T, U>(SingleArrayAggregate ast, t: T) -> U;
 
//    // Record Aggregates
//    fn visitMultipleRecordAggregate<T, U>(MultipleRecordAggregate ast, t: T) -> U;
//    fn visitSingleRecordAggregate<T, U>(SingleRecordAggregate ast, t: T) -> U;
 
//    // Formal Parameters
//    fn visitConstFormalParameter<T, U>(ConstFormalParameter ast, t: T) -> U;
//    fn visitFuncFormalParameter<T, U>(FuncFormalParameter ast, t: T) -> U;
//    fn visitProcFormalParameter<T, U>(ProcFormalParameter ast, t: T) -> U;
//    fn visitVarFormalParameter<T, U>(VarFormalParameter ast, t: T) -> U;
 
//    fn visitEmptyFormalParameterSequence<T, U>(EmptyFormalParameterSequence ast, t: T) -> U;
//    fn visitMultipleFormalParameterSequence<T, U>(MultipleFormalParameterSequence ast, t: T) -> U;
//    fn visitSingleFormalParameterSequence<T, U>(SingleFormalParameterSequence ast, t: T) -> U;
 
//    // Actual Parameters
//    fn visitConstActualParameter<T, U>(ConstActualParameter ast, t: T) -> U;
//    fn visitFuncActualParameter<T, U>(FuncActualParameter ast, t: T) -> U;
//    fn visitProcActualParameter<T, U>(ProcActualParameter ast, t: T) -> U;
//    fn visitVarActualParameter<T, U>(VarActualParameter ast, t: T) -> U;
 
//    fn visitEmptyActualParameterSequence<T, U>(EmptyActualParameterSequence ast, t: T) -> U;
//    fn visitMultipleActualParameterSequence<T, U>(MultipleActualParameterSequence ast, t: T) -> U;
//    fn visitSingleActualParameterSequence<T, U>(SingleActualParameterSequence ast, t: T) -> U;
 
//    // Type Denoters
//    fn visitAnyTypeDenoter<T, U>(AnyTypeDenoter ast, t: T) -> U;
//    fn visitArrayTypeDenoter<T, U>(ArrayTypeDenoter ast, t: T) -> U;
//    fn visitBoolTypeDenoter<T, U>(BoolTypeDenoter ast, t: T) -> U;
//    fn visitCharTypeDenoter<T, U>(CharTypeDenoter ast, t: T) -> U;
//    fn visitErrorTypeDenoter<T, U>(ErrorTypeDenoter ast, t: T) -> U;
//    fn visitSimpleTypeDenoter<T, U>(SimpleTypeDenoter ast, t: T) -> U;
//    fn visitIntTypeDenoter<T, U>(IntTypeDenoter ast, t: T) -> U;
//    fn visitRecordTypeDenoter<T, U>(RecordTypeDenoter ast, t: T) -> U;
 
//    fn visitMultipleFieldTypeDenoter<T, U>(MultipleFieldTypeDenoter ast, t: T) -> U;
//    fn visitSingleFieldTypeDenoter<T, U>(SingleFieldTypeDenoter ast, t: T) -> U;
 
//    // Literals, Identifiers and Operators
//    fn visitCharacterLiteral<T, U>(CharacterLiteral ast, t: T) -> U;
//    fn visitIdentifier<T, U>(Identifier ast, t: T) -> U;
//    fn visitIntegerLiteral<T, U>(IntegerLiteral ast, t: T) -> U;
//    fn visitOperator<T, U>(Operator ast, t: T) -> U;
 
//    // Value-or-variable names
//    fn visitDotVname<T, U>(DotVname ast, t: T) -> U;
//    fn visitSimpleVname<T, U>(SimpleVname ast, t: T) -> U;
//    fn visitSubscriptVname<T, U>(SubscriptVname ast, t: T) -> U;
 
//    // Programs
//    fn visitProgram<T, U>(Program ast, t: T) -> U;
 
 }
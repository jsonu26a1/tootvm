# dev notes
for review, here's the workflow of "executing" a script:
  1. parse string into token stream
  2. process token stream into abstract syntax tree
  3. verify and transform AST (type errors, dynamic typing sugar, etc)
  4. compile AST statements into bytecode
  5. finally, executing bytecode on the virtual machine

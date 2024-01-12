# dev notes
for review, here's the workflow of "executing" a script:
  1. parse string into token stream
  2. process token stream into abstract syntax tree
  3. verify and transform AST (type errors, dynamic typing sugar, etc)
  4. compile AST statements into bytecode
  5. finally, executing bytecode on the virtual machine


## 2024-01-12
I was just thinking... about how to implement the GUI, and my first idea was to use a
webview. It is a easy and portable route to take. Then my thinking eventually went
to about implementing the vm in JS, and what about compiling the scripting lang into
JS? I can see the appeal to running the vm in JS, but it wouldn't make sense when
trying to use OS apis. What about doing both? We could have this rust impl, and also
have an impl in JS.

I think I'll stick with rust for now, but will keep in mind JS (and python, others)
as a possible future platform the script langauge; an interpretter based vm would have
poor performance, so language compilation/translation might be the route for those
langs; but for the REPL work flow, the compiler/translator for each platform would
be implemented in that platform.

I did briefly consider how the script language compilation to JS might integrate into
the native JS namespace and runtime; but I think keeping them completely isolated
is the way to go. I want module importing/resolving to be easy and consistent
regardless of platform. I know that implementing it in this way will be a big challenge
and probably take multiple attempts to get right.

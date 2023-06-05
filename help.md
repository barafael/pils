Welcome to pils, a simple lisp :)

example expressions:
    * 1 2 3 4 ( + 3 4) 0
    ( / 100 3 10 )
    eval { tail ( list 1 2 3 4 ) }
    eval (tail {tail tail {5 6 7}})
    tail { tail join eval head }
    eval {head (list 1 2 3 4)}
    def {a b c} 1 2 3
    def {args} {a b c}
    def args 4 5 6

+, -, *, / work as prefix operators on numbers
    and s-expressions that evaluate to numbers.

'(' and ')' create an s-expression like so: '(* 1 2 3 )'
    An s-expression always starts with an operator and
    is followed by numbers or other s-expressions.

'{' and '}' create a q-expression like so: '{ 1 2 3 tail }'
    A q-expression is not evaluated and can contain anything.
    Special operators act on q-expressions:

'head' takes the first element of a q-expression.
'tail' takes all elements of a q-expression, except the first.
'join' takes a q-expression with q-expressions inside, and
    creates one q-expression with their contents.
'eval' pretends a q-expression is an s-expression and
    evaluates it normally.

'list' creates a q-expression from an s-expression.

For a detailed reference, see: https://buildyourownlisp.com/.
Thanks and credits to Daniel Holden for this brilliant resource.

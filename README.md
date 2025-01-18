# Lispen

A Lisp-like programming language written in Rust.

## Syntax

Lispen follows the same syntax as some Lisp dialects:

```lisp
(println "Hello World")
```

The example above is a `list`, which, by default, is interpreted as a function call if the first argument is an identifier.

## Defining variables

To define a variable in Lispen, you use the `set` keyword.

```lisp
(set x 10)
```

This will bind the value `10` to the variable `x` in the current scope.

### Types

- `num`
- `str`
- `bool`
- `list`
- `fn`
- `nil`

### Numbers

All numbers in Lispen have the same type, `num`, and it supports both integers and floats.

> Note: inside the interpreter this type is represented by a `f64`.

### Arithmetic

Lispen, like other Lisp dialects, uses the [Polish notation](https://en.wikipedia.org/wiki/Polish_notation) to represent arithmetic.

This is the syntax:

```lisp
(operator operand1 operand2)
```

Example of a sum:

```lisp
(+ 1 2)
```

### Lists

Like explained above, lists can represent function calls, if the first argument is an identifier.

But you can also make the list be interpreted as a literal list, even if the first argument is an identifier.

Use the quote `'` character before the list:

```lisp
'(println 10)
```

In this case, it'll be evaluated to a list with two elements, a function and a number.

### Functions

There are two ways of defining functions in Lispen: using the `defn` and the `fn` keywords.

Every function creates a new scope inside it.

#### `defn`

`defn` defines a function and also binds it to a name.

```lisp
(defn f(x) (+ x 1))
```

#### `fn`

`fn` only defines a function.

```lisp
(fn(x) (+ x 1))
```

To assign it to a name, you can use the `set` keyword.

```lisp
(set x (fn(x) (+ x 1)))
```

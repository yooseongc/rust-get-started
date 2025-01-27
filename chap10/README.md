
#### Generic Types, Traits, and Lifetimes

Every programming language has tools for effectively handing the duplication of concepts.

1. `generic`: abstract stand-ins for concrete types or other properties

Functions can take parameters of some generic type, instead of a concrete type like `i32` or `String`, in the same way they take parameters with unknown values to run the same code on multiple concrete values.

2. `traits` to define behavior in a generic way

You can combine traits with generic types to constrain a generic type to accept only those types that have a particular behavior, as opposed to just any type.

3. `lifetimes`: a variety of generics that give the compiler information about how references relate to each other

Lifetimes allow us to give the compiler enough information about borrowed values so that it can ensure references will be valid in more situations than it could without our help.


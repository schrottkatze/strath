# Strath - The String Math Language
## What is it?
Strath is an esolang that has a cryptic syntax and allows you to do math with strings, add them, multiply them, and even multiply with floats!

Rust developers hate it, Python developers fear it, and C developers make it blink.

## How to Strath
### Comments
`;` all text after a semicolon will be treated as a comment  
`&&` all text after a double and will be treated as a comment  
`]][[` an inline command, all text between the double square brackets will be treated as a comment  
\`\`´´ this makes a multiline comment

### If, else if/elif, else
`¿??/condition\? | ()` - if  
`?>/condition\? | ()` - elif/else if  
`\>?/condition\? | ()` - else  

After `??` and `?>` either a `$` or `€` to declare if all conditions are to be true or false respectivly  
Two variables will be compared by an if statment without "==" eg `¿??$/1 1\? | ()` compares if 1 == 1 and if that result needs to be true to run the code in between the two round brackets

### Variable Types
`}}type{{` uses the standard types like int, str, float, etc

Certain types have short declarations  
`ö` declares ints  
`Ö` declares floats  
`ä` declares char  
`Ä` declares string  
`ü` declares list  
`Ü` declares dict  

wavey brackets aren't needed in this case

#### Working With Variables
Variables are declared by defining the type, the name, and the value (optional) eg:  
`}}int{{ i < 1;`

`}int{ i < 1;` A single wavy bracket allows the type of the var to change dynamicly

Special behaviour:  
`j > i` will copy i to j and drops i  
`j < i` will copy i to j and maintain i

### In-build functions
`<=` will output a value  
`=>` takes an input and sets the next variable to it as the input

### Code
`¡!` are statments
`¿?` are expression

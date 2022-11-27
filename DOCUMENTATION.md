#Documentation

Hi, if you are reading this, you want to learn how to use FuckBrainFuck.

First you need to know the classic BrainFuck syntax.
You should read this course:
- [Brainfuck Language (English) (by roachhd)](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a)
- [Brainfuck Language (French) (by Astremy)](https://cdn.discordapp.com/attachments/815331771197030441/824402769397940234/brainfuck.pdf)


Now I'm going to explain how to use the new implementations of FuckBrainFuck.

###FuckBrainFuck already offers:

- "Safe" comments 
- Functions


## Comments
Safe comments allow you to use any characters in the comments as opposed to the classic brainfuck comments
where you can't use the characters used by the language

You can write comments between two `#` characters.

Example:
```
,# This is a safe comment. I can use <>+-[],. and
others characters of FuckBrainFuck here like :;
I can also skip lines. This program returns the character of the input #.
```


## Functions
Functions are a way to use the same code multiple times.
Functions are stored in a different array than the classic BrainFuck array.

You can define functions between `:` and `;`. To call it, use `x`.

You can use recursive functions. 
You can also define functions inside functions,
but you have to move the pointer to right or left
because you can't have multiple functions in the same cell.


Example:
```
:,++.;  # This functions asks the user and adds two to the cell and then prints the result. #
x # Execute the function. #
```

Nested Functions: 
```
:
    ,
    >:
        <++
    ; # This function go to left and adds 2 to the value of the cell.#
    x. # Execute the function and display the result.
         After executing the function, we are one square ahead.
         Here, the function is obviously useless since we use it only once.#
;
x
```
_In the example above, I indented the code. It makes it more readable, doesn't it?_

You can see that before declaring the second function, I moved my pointer to the right.
This allows me to avoid unexpected behavior.


Recursive Functions:
```
:
    [[->+>[->+>+<<]>>[-<<+>>]<<<<]>>[-]>[-<+>]<<[-<+>]<-x[-]]
; # This function calculates the factorial of a number. #
>>+<<
+++++  # Here is the input for the recursive function. #
x>>. # The output will be "x" because 120 is the ASCII value of "x".
       The output is always the remainder of the factorial divided by 255. #
```
_Thanks Astremy for the code!_


I think you now know all about the FuckBrainFuck's syntax.
Thanks for reading the documentation.
Maybe the people in [the BrainFuck channel on Graven's server](https://discord.gg/DTtXYNc3ct)
will help you with FuckBrainFuck (in French).
I hope everything goes well for you, goodbye
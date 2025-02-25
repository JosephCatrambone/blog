---
title: 'Matlab Sucks'
description: ""
published: 2012-09-20
redirect_from: 
            - https://www.josephcatrambone.com/?p=254
categories: "Rant"
hero: ../../../defaultHero.jpg
---
Among my frustrations with Matlab is this puzzling obfuscation. Consider the following: Since Matlab doesn't have a List type which can be used for random access, insertion, and removal, we can either use their Cell type, or rearrange a matrix and reshape it. If you decide to go with the cell option and want to, for instance, read off the first column of your data, the following won't work.Our variable 'a' is a set of cells, {\[1,2]}, {\[1,2]}, {\[1,2]}, {\[1,2]}

```
>> cell2mat(a)

ans =

     1     2     1     2     1     2     1     2
```

Oh god no. Let's try that again.

```
>> cell2mat(a')

ans =

     1     2
     1     2
     1     2
     1     2
```

Better. Let's pull out the first column of that data.

```
>> cell2mat(a')[1,:]
 cell2mat(a')[1,:]

Error: Unbalanced or unexpected parenthesis or bracket.
```

Huh? Oh yes. Matlab uses parenthesis for indexing instead of square brackets, though square brackets are used to create matrices and arrays.

```

>> cell2mat(a')(1,:)
Error: ()-indexing must appear last in an index expression.
```

Argh. The () indexing IS last! What the hell, matlab?

```
>> (cell2mat(a'))(1,:)
 (cell2mat(a'))(1,:)

Error: Unbalanced or unexpected parenthesis or bracket.
```

What? But... what?

```

>> b = cell2mat(a')

b =

     1     2
     1     2
     1     2
     1     2

>> b(1,:)

ans =

     1     2
```

Why do I need to create another item which stores the cell2mat output instead of using it directly?

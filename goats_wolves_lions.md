# The problem

There is a magical forest that contains only goats, wolves and lions. At each instant of time, exactly one of the following events must occur:

 - wolf devours a goat and subsequently turns into a lion
 - lion devours a goat and subsequently turns into a wolf
 - lion devours a wolf and subsequently turns into a goat
 - nothing happens

The last event can only occur when it is not possible for one animal to devour another. We say that the forest is **stable** when no more devourings can happen.

Write a program that finds the largest number of animals in a stable forest given an initial forest of $a$ goats, $b$ wolves, $c$ lions. (In the original puzzle which was designed to be solved by hand, $a = 17, b = 55 c = 6$).


# Analysis

The number of animals decreases by 1 at each step. Also, the forest reaches a stable state exactly when there's only one type of animal left in the forest. So we really seek the quickest way to a homogenous forest.

We can recast this abstractly: given a tuple $(a,b,c)$ of natural numbers, at each time step we pick two of the components to decrement and we increment the remaining component. We do this until the tuple has only one non-zero component. Such a tuple is called a **stable tuple**. We would like to know what is the highest number of all the stable tuples obtainable from starting at $(a,b,c)$

Note that the only way we can transition to a stable tuple is turning two non-zero components into zero components simultaneously. For example, the tuple $(1,1,n)$ for any $n$ would work. Note that if we end up in a tuple $(m,m,n)$ for some naturals $m$ and $n$, then we can just repeatedly combine the first two components to obtain a stable tuple.

We can define a function $p: (a,b,c) \mapstos (a mod 2, b mod 2, c mod)$, which assigns to each tuple its associated parity tuple. Each step results in the parity tuple being flipped. That means we can only equalize two components that have the same parity.

This fact completely determines our strategy for tuples whose components do not all have the same parity. Let $i,j,k \in \{0, 1, 2\}$ be all different, and suppose $a_i$ and $a_j$ have equal parity but $a_k$ is different. Supposing that $a_i < a_j$, if $a_k \neq 0$, then we can combine $j$ and $k$ to increment $i$. If $a_k = 0$, we have to generate more $k$'s first before we can attempt to further equalize $i$ and $j$. In this case our pseudocode looks like:


But what should our strategy be when all components do have the same parity?


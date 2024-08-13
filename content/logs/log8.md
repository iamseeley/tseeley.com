---
title: log8 
date: 2024-08-13
---

**notes from [sicp](https://mitp-content-server.mit.edu/books/content/sectbyfn/books_pres_0/6515/sicp.zip/index.html)**

thought i'd make a note about **iterative vs. recursive processes** because i found it a little confusing.

so, first i think it's important to highlight the difference between a recursive procedure and a recursive process. recursive procedure means that the procedure either directly or indirectly refers to itself. when talking about a recursive process, let's say a linear recursive process, we are talking about how the process evolves. we can refer to a recursive procedure as creating an iterative process.

okay, what's the difference between a recursive and iterative process? they are both recursive procedures, but what makes them different is how they evolve. let's look at two different procedure's from sicp exercise [1.9](https://github.com/iamseeley/sicp/blob/main/ch1/1.09.scm) to get a better look at how the different processes evolve.

each of these procedures defines a method for adding two positive integers. `inc` increments its arguments by 1, and `dec` decrements its arguments by 1. to visualize the computation for each procedure we can use the substitution model! this should make it more clear which process is which. 

```scheme
(define (+ a b)
    (if (= a 0)
        b
        (inc (+ (dec a) b))))
```

```scheme
; let's add some integers using the procedure above

(+ 4 7)
(inc (+ 3 7))
(inc (inc (+ 2 7)))
(inc (inc (inc (+ 1 7))))
(inc (inc (inc (inc (+ 0 7)))))
(inc (inc (inc (+ 1 7))))
(inc (inc (+ 2 7)))
(inc (+ 3 7))
(+ 4 7)

; 11
```

```scheme
(define (+ a b)
    (if (= a 0)
        b
        (+ (dec a) (inc b))))
```

```scheme
; okay now let's look at the next one

(+ 4 7)
(+ 3 8)
(+ 2 9)
(+ 1 10)
(+ 0 11)

; 11
```

can you tell which one is a recursive process and which one is iterative?

there's a noticeable difference in the substitution models for each procedure that illustrates the difference pretty well.

for the first procedure the substitution model illustrates an expansion and contraction of the process. the process expands and builds up a chain of deferred operations. the process contracts when the operations are actually performed.

when a process builds up a chain of deferred operations it is labeled as a recursive process. 

for the second procedure the substitution model does not illustrate expansion or contraction of the process. at each step the interpreter only needs to keep track of the variables a and b until it meets the base case. 

this is usually how an iterative process is defined: "the state can be summarized by a fixed number of state variables, together with a fixed rule that describes how the state variables should be updated as the process moves from state to state and an (optional) end test that specifies conditions under which the process should terminate."

i think what initially tripped me up was the terminology.

**in other happenings**

i finished the first draft of a write-up i'm doing on my [simpl-site](https://github.com/iamseeley/simpl-site) website builder! i'll be making some revisions and posting soon.

i have not abandoned my web analytics tracker project! planning on finishing that up soon too.

i'll leave you with a photo of my grandma showing me the socks she got for her birthday.

![butterfly socks](https://res.cloudinary.com/dcwnusepx/image/upload/v1723558435/tseeley/IMG_0678_zv5gx3.jpg)

✌️

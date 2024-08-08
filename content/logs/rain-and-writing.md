---
title: rain and writing
date: 2024-08-07 
---

it's been raining for five days straight...

inside computer vibes are strong.

wanted to post a little recap of today.

over the past few weeks i've made it a habit to start my day with some reading and a challenging coding problem (shouldn't take me longer than 15 minutes to complete)

i read some more of sicp and completed another [exercise](https://github.com/iamseeley/sicp/blob/main/ch1/1.6.scm). it was about the `if` special form in scheme and potentially defining it as an ordinary procedure using `cond` like:

```scheme
(define (new-if predicate then-clause else-clause)
    (cond (predicate then-clause)
        (else else-clause)))
```

and what would happen if we used it in this program:

```scheme
(define (sqrt-iter guess x)
    (new-if (good-enough? guess x)
        guess
        (sqrt-iter (improve guess x)
                    x)))
```

if we were to run `sqrt-iter` it would recursively call itself indefintely and not terminate. 

this happens because of the `cond` special form's evaluation method. `cond` evaluates its conditions one by one (applicative order) and returns the result of the first true condition. 

when `new-if` is called, all its arguments are evaluated before the `cond` expression is executed. this causes `sqrt-iter` to recursively call itself even when the base condition is met. 

in scheme the `if` special form only evaluates either the `alternative` or the `consequent` based on the result of the `predicate`.

**the coding problem of the day was pretty challenging :/**

it was "Products of Array Except Self". The goal was to return an array where each element is the product of all other elements in the given integer array. You have to do this in O(n) without using division. 

My solution involved creating two arrays `left` and `right`, to store the cumulative products from the left and right sides of the array. I initialized the arrays to 1, and we compute the `left` and `right` products in two separate passes. 

Then, in a third pass we generate the output array by multiplying the corresponding elements from the `left` and `right` arrays.

here's the solution:

```python
class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:

        n = len(nums)
        if n == 0:
            return []
        
        # Initialize arrays
        left = [1] * n
        right = [1] * n
        output = [1] * n
        
        # Compute left products
        for i in range(1, n):
            left[i] = left[i - 1] * nums[i - 1]
        
        # Compute right products
        for i in range(n - 2, -1, -1):
            right[i] = right[i + 1] * nums[i + 1]
        
        # Compute the output array
        for i in range(n):
            output[i] = left[i] * right[i]
        
        return output
            
```

**then i moved to my main project work for the day**

a few weeks ago i made a website builder(?framework) called [simpl-site](https://github.com/iamseeley/simpl-site). this website is using it! 

i got motivated to do a in-depth write up of the project, going over the background and doing a deep dive into the code. 

so, i spent most of the day working on a draft! it's going to be my first "technical" blog post and definteley the longest piece of writing i've posted.

looking forward to sharing it.

**other projects i'm still working on:**

web analytics tracker (cleaning up dashboard styles)

a [color picker](https://www.val.town/v/iamseeley/dictionaryOfColors) for colors from "a dictionary of color combinations" (working on color swatch grid transitions/animation)

one more thing i'm very stoked on is a book i've been looking forward to reading came in the mail!


it's [*the new media reader*](http://www.newmediareader.com/about.html)

![the new media reader cover](http://www.newmediareader.com/graphics/nmrfront300.jpg)

it even came with a cd haha.

![new media reader cd](https://res.cloudinary.com/dcwnusepx/image/upload/v1723135986/tseeley/IMG_1902_fezywy.jpg)

excited to get into it!

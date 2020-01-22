


# Deriving A General EC Point Addition Formula
An elliptic curve is a curve of the form y^2 = x^3 + ax^2 + b. Points on the curve are denoted Pi = (xi, yi) where i is a counting number. To add two points P1 and P2, we draw a line through the points. As long as these points don't share an x-coordinate, this line will intersect the curve at exactly one other point (Don't worry, we'll come back and handle the special cases in a minute). The sum of P1 and P2 is this third point, reflected over the x axis. 

In order to obtain the addition formula, note that:
1. P3 lies on the the line through P1 and P2. That line has some slope, s. Thus P3.y = P1.y + s*(P3.x - P1.x). For simplicity, we'll call P2's coordinates X2 and Y2 and P1's coordinates X1 and X2 in the future. Thus, **Y3 = Y1 + s(X3 - X1)**
1. The slope of the line through P1 and P2 can be found with the pre-algebra formula, rise over run. Thus, **s = (Y2 - Y1) / (X2 - X1)**

Got it? Let's start the proof: 
Recall that for all points on the curve, **Y^2 = X^3 + aX + b** by the elliptic curve definition
Also recall from pre-algebra that all points which lie on the line between P1 and P2 are given by the equation, **Y = s*(X - X1) + Y1** where s is the slope of the line from P1 to P2. (For example, if (3, 5) is a point on a line with slope 2, then (4, 7) and(2.5, 4) are both on the line as well.)

Combining these two formulas will give us an equation for all points that are on both the line between P1 and P2 and whatever elliptic curve we're interested in. More formally, this is the equation for all points at which the line intersects the curve:
**(s(X - X1) + Y1)^2 = X^3 + aX + b**

Expanding that out we get:
=> (s(X-X1))^2 + 2sY1(X - X1) + Y1^2 = X^3 + aX + b

=> s^2(X^2) - 2sX(X1) + s^2(X1^2) + 2sY1(X) - 2sY1(X1) + Y1^2 = X^3 + aX + b

Subtracting (s^2(X^2) - 2sX(X1) + s^2(X1^2) + 2sY1(X) - 2sY1(X1) + Y1^2) from both sides:

=> X^3 + aX + b - s^2(X^2) + 2sX(X1) - s^2(X1^2) - 2sY1(X) + 2sY1(X1) - Y1^2 = 0

Gathering all terms with factors of X at the front: 

=> X^3 - s^2(X^2) + (a + 2sX1 - 2sY1)X - s^2(X1^2) + 2sY1(X1) - Y1^2 + b


Now, note that X1, X2, and X3 are roots of the above equation. Put another way, when X is one of those three values, Y^2 = X^3 + aX + b, so 
X^3 + aX + b - Y^2 = 0.

Thus, 
(X - X1)(X - X2)(X - X3) = 0 

=> (X^2 - X2(X) - X1(X) + X1(X2))(X - X3) = 0

=> X^3 - X^2(X2) - X^2(X1) + X(X1)(X2) - X^2(X3) + X(X2)(X3) + X(X1)X3 - (X1)(X2)(X3) = 0

=> X^3 - X^2(X1 + X2 + X3) + X((X1)(X2) + (X2)(X3) + (X1)(X3)) - (X1)(X2)(X3) = 0 = X^3 - s^2(X^2) + (a + 2sX1 - 2sY1)X - s^2(X1^2) + 2sY1(X1) - Y1^2 + b

In order for this equality to hold, the coefficients on each of the variable terms (i.e. terms containing X, X^2, or X^3) must be equal. Thus

=> (X1 + X2 + X3) = s^2

=> **s^2 - X1 - X2 = X3**

And voila... just like that we have a formula expressing the unknown X3 in terms of s, X1, and X2, all of which are known.

Now that we know the X coordinate, calculating the Y coordinate is simple. We simply find the distance between X3 and X1 and multiply by the slope (s) to find a vertical offset: dY = s(X3-X1)  To turn the offset into an actual coordinate, we simply add Y1 (this gets us to a point on the curve), and then reflect over the y axis

=> Y3 = -s(X3-X1) - Y1

=> **Y3 = s(X1 - X3) - Y1**

That's it! These formulas for X3 and Y3 will hold for all additions P1 + P2, as long as P1 and P2 don't share an x-coordinate. One more time
- **s = (Y2 - Y1) / (X2 - X1)**
- **X3 = s^2 - X1 - X2**
- **Y3 = s(X1 - X3) - Y1**


## A Few Special Cases

Now that we have a general formula for addition, let's handle the case where the x-coordinates of P1 and P2 are equal. This can happen in one of two ways
1. P1 and P2 are the same point
1. P1 and P2 lie on a vertical line, one directly above the other

### Case 1: P1 and P2 are the same point
In the first case, we can reuse our formula from above with one slight modification. Remember how we used s = (Y2 - Y1) / (X2 - X1) for the slope? Since X1 == X2, this will give us zero in the denominator, which isn't allowed. Instead, we'll need to use calculus to find the slope of the curve at the point P1 (which is the same as the slope at P2, since P1 and P2 are the same point). Thus...

Y^2 = X^3 + aX + b

=> 2Y dy = (3X^2 + a) dx

=> 2Y dy/dx = 3X^2 + a

=> dy/dx = 3X^2 + a / 2Y

Plugging in Y1 and X1, we get **s = (3X1^2 + a) / 2Y1**

### Case 2:  P1 and P2 lie on a vertical line
In the second case, the slope of the line between P1 and P2 is easy to calculate. Since one is above the other, the slope is infinite (or, perhaps, infinitely negative). The trouble is, this line isn't exactly going to intersect with our curve at some convenient point. To solve this, we'll simply make up one extra point, which we call "The point at infinity", and say that it's the sum of P1 and P2. Then, to make life easy, we define the point at infinity to be the arithmetic identity (Zero). Thus if P1, P2, and P3 are points such that P1 is directly above P2 we have

 (P1 + P2) + P3

= (Point at Infinity) + P3
= (0) + P3
= P3



# Putting the Cryptography in Elliptic Curve Cryptography
Now that we have a formula for point addition, let's build a working crypto system. First, let's define multiplication by example...

2 * P1 = P1 + P1

3 * P1 = (P1 + P1) + P1 = **2 * P1 + P1**

5 * P1 = P1 + P1 + P1 + P1 + P1 = (P1 + P1) + (P1 + P1) + P1 = (2 * P1) + (2 * P1) + P1 = **2 * (2 * P1) + P1**

Now, let's add one wrinkle to our scheme. Rather than defining an elliptic curve over the field of real numbers, we'll define it over a finite field. As is often the case, swapping out the field over which an object is defined has no effect on its governing equations. The formulas we've outlined for addition and multiplication will hold exactly. The key advantage of using a finite field is that it makes the "log problem" (which for curve points, could just as well be called the "divison problem") extremely difficult. To get an intuition about why this is, let's take the example of integers (rather than curve points) over a finite field. 

lets say that we have some finite field, maybe {0 .. 223}. 

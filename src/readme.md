


# EC Point Addition Formula
An elliptic curve is a curve of the form y^2 = x^3 + ax^2 + b. Points on the curve are denoted Pi = (xi, yi) where i is a counting number. To add two points P1 and P2, we draw a line through the points. As long as these points don't share an x-coordinate, this line will intersect the curve at exactly one other point (Don't worry, we'll come back and handle the special case in a minute). The sum of P1 and P2 is this third point, reflected over the x axis. 

In order to obtain the addition formula, note that:
1. P3 lies on the the line through P1 and P2. That line has some slope, s. Thus P3.y = P2.y + s*(P3.x - P2.x). 
1. The slope of the line through P1 and P2 can be found with the pre-algebra formula, rise over run. Thus, s = (P2.y - P1.y) / (P2.x - P1.x)

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

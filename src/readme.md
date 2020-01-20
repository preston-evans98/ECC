


# EC Point Addition Formula
An elliptic curve is a curve of the form y^2 = x^3 + ax^2 + b. Points on the curve are denoted Pi = (xi, yi) where i is a counting number. To add two points P1 and P2, we draw a line through the points. As long as these points don't share an x-coordinate, this line will intersect the curve at exactly one other point (Don't worry, we'll come back and handle the special case in a minute). The sum of P1 and P2 is this third point, reflected over the x axis. 

In order to obtain the addition formula, note that:
1. P3 lies on the the line through P1 and P2. That line has some slope, s. Thus P3.y = P2.y + s*(P3.x - P2.x). 
1. The slope of the line through P1 and P2 can be found with the pre-algebra formula, rise over run. Thus, s = (P2.y - P1.y) / (P2.x - P1.x)

Got it? Let's start the proof: 
Recall that for all points on the curve, **Y^2 = X^3 + aX + b** by the elliptic curve definition
Also recall from pre-algebra that all points which lie on the line between P1 and P2 are given by the equation, **Y = s*(X - X1) + Y1** where s is the slope of the line from P1 to P2. (For example, if (3, 5) is a point on a line with slope 2, then (4, 7) and(2.5, 4) are both on the line as well.)

Combining these two formulas will give us an equation for all points that are on both the line between P1 and P2 and whatever elliptic curve we're interested in. More formally, this is the equation for all points at which the line intersects the curve:
**(s.(X - X1) + Y1)^2 = X^3 + aX + b**

Expanding that out we get:
s(X-X1)^2 + 2sY1(X - X1) + Y1^2 = X^3 + aX + b






Substituting from (1), we get =>  (P2.y + s*(P3.x - P2.x))^2 = P3.x^3 + a*P3.x + b 

Expanding the left hand side => P2.y^2 + 2*P2.y*s*(P3.x - P2.x) + (s*(P3.x - P2.x))^2 = P3.x^3 + a*P3.x + b 

Expanding the subtracted terms => P2.y^2 + 2*P2.y*s*P3.x - 2*P2.y*s*P2.x +| s^2*P3.x^2 - 2 * P2.x*P3.x + p2.x^2 = P3.x^3 + a*P3.x + b 

P2.y^2 - 2*P2.y*s*P2.x +|   + p2.x^2 = P3.x^3 + a*P3.x + b 

Moving all the x3s to one side => P3.x^3 + a*P3.x - 2*P2.y*s*P3.x - s^2*P3.x^2 + - 2 * P2.x*P3.x = 





=> P2.y^2  + p2.x^2 - b - 2*P2.y*s*P2.x = P3.x^3 + a*P3.x - 2 * P2.x*P3.x - s^2*P3.x^2  - 2*P2.y*s*P3.x 
=> P2.y^2  + p2.x^2 - b - 2*P2.y*s*P2.x = P3.x * (P3.x^2 + a - 2*P2.x - s^2*P3.x - 2*P2.y)

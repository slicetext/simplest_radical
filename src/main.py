import math
squares=[]
def gen_squares(n):
    for i in range(2,n+1):
        squares.append(i*i)
gen_squares(30)
remainder=0
# Source: https://stackoverflow.com/questions/6239967/determining-whether-an-value-is-a-whole-number-in-python
def isInteger(n):
    """Return True if argument is a whole number, False if argument has a fractional part.

    Note that for values very close to an integer, this test breaks. During
    superficial testing the closest value to zero that evaluated correctly
    was 9.88131291682e-324. When dividing this number by 10, Python 2.7.1 evaluated
    the result to zero"""

    if n%2 == 0 or (n+1)%2 == 0:
        return True
    return False
def square_root(num):
    global remainder
    if(isInteger(math.sqrt(num))):
        return math.sqrt(num)
    for i in squares:
        if(num/i>1 and isInteger(num/i)):
            return square_root(i)*square_root(num/i)
    remainder=remainder+num
    return 1

while True:
    num=float(input("Get root of number:"))
    print(str(square_root(num))+" √"+str(remainder))
    remainder=0

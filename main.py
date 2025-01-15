import math
squares=[]
tree={0:[""]}
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
def square_root(num,index=0,index2=1):
    global remainder
    if(index not in tree):
        tree[index]=[""]
    if(index+1 not in tree):
        tree[index+1]=[""]
    while(len(tree[index])<index2+1):
        tree[index].append("")
    while(len(tree[index+1])<index2+1):
        tree[index+1].append("")
    tree[index][index2]=int(num)
    if(isInteger(math.sqrt(num))):
        tree[index+1][index2]=str(int(math.sqrt(num)))+"*"
        return math.sqrt(num)
    for i in squares:
        if(num/i>1 and isInteger(num/i)):
            return square_root(int(i),index+1,index2-1)*square_root(int(num/i),index+1,index2+1)
    remainder=remainder+num
    return 1

while True:
    num=float(input("Get root of number:"))
    print(str(square_root(num))+" √"+str(remainder))
    longest=0
    num_cols=0
    cols_row={}
    for i in range(len(tree)):
        if(len(tree[i])>num_cols):
            num_cols=len(tree[i])
        col_count=0
        cols_row[i]=0
        for j in tree[i]:
            if(len(str(j))>longest):
                longest=len(str(j))
            cols_row[i]=cols_row[i]+len(str(j))
    #print(" "*int(longest*num_cols/2),end="")
    for i in range(len(tree)):
        cols=1
        chars=cols_row[i]
        #print(" "*(longest*num_cols-chars-1),end="")
        for j in tree[i]:
            print(j,end=" "*(longest-len(str(j))))
            cols=cols+1
        print("")
    remainder=0
    tree={0:[]}

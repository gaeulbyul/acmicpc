while 1:
 i=input()
 a,b,c=map(lambda x:int(x[::-1]),i.replace('+','=').split('='))
 print(a+b==c)
 if i=='0+0=0':break
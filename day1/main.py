arraypart1 = []
sub_arraypart2 = []
arraypart2 = []
num_strings = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine', 'zero']
num_ints = [1,2,3,4,5,6,7,8,9,0]
file_path = 'input.txt'

with open(file_path, 'r') as file:
    for line in file:
        linenum = ''
        # part 1
        for char in line:
            if char.isdigit():
                linenum += char
                break
        for char in reversed(line):
            if char.isdigit():
                linenum += char
                break
        
        # part 2        
        positions = []
        linelen = len(line)
        for string in num_strings:
            if string in line:
                strlen = len(string)
                for i in range(linelen - strlen):
                    linecut = line[i:strlen+i]
                    if linecut == string:
                        positions.append({'position': i, 'digit': string})

        for i in range(linelen):
            if line[i].isdigit():
                positions.append({'position': i, 'digit': line[i]})


        sub_arraypart2.append(positions)
        if linenum != '':
            arraypart1.append(int(linenum))

for all in sub_arraypart2:
    for each in all:
        if isinstance(each['digit'], str) and not each['digit'].isdigit():
            for i in range(len(num_strings)):
                if each['digit'] == num_strings[i]:
                    each['digit'] = num_ints[i]
                    break

for lst in sub_arraypart2:
    sorted_array = sorted(lst, key=lambda x: x['position'])
    firstvalue = sorted_array[0]['digit'] 
    lastvalue =  sorted_array[-1]['digit']
    added = str(firstvalue) + str(lastvalue)
    arraypart2.append(int(added))

print(sum(arraypart1))  
print(sum(arraypart2))
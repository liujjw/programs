import sys
# import numpy as np
# import pandas as pd
# from sklearn import ...

rows = []
for line in sys.stdin:
    # adj list to adj matrix
    # undirected graph
    # 0 indexed vertices 
    rows.append(line)

# for debugging, just manually hardcode some inputs?
# this is doubless faster than typing things in using an input() command
# and so then my dev loop is gonna have to be dev doing all dev on vscode first?
# nah its faster to get quick feedback on the service
# but using the hardcoded vscode debug mode and python3 interpreter is the fallback

# DO NOT USE the * syntactic sugar for 2D array generation

num_vertices = len(rows)
adj_matrix = []
for i in range(num_vertices):
  adj_matrix.push([])
  for j in range(num_vertices):
    adj_matrix.push(0)

for row in rows:
    nums_as_str = row.split(' ')
    row_idx = int(nums_as_str[0])
    for i in range(1, len(nums_as_str)):
        col_idx = int(nums_as_str[i])
        adj_matrix[row_idx][col_idx] = 1
        
for i in range(len(adj_matrix)):
    for j in range(len(adj_matrix)):
        if j == num_vertices - 1:
            print(adj_matrix[i][j], end='\n')
        else:
            print(adj_matrix[i][j], end=" ")
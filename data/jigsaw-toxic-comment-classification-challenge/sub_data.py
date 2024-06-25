import pandas as pd

input_filename = 'train.csv'
output_filename = f'sub1k_{input_filename}'
df = pd.read_csv(input_filename)
df.iloc[:1000].to_csv(output_filename, index=False)

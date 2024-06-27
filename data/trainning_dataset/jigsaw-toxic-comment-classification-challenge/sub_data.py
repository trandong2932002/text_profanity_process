import pandas as pd

input_filename = 'train.csv'
output_filename = f'sub100_{input_filename}'
df = pd.read_csv(input_filename)
df.iloc[:100].to_csv(output_filename, index=False)

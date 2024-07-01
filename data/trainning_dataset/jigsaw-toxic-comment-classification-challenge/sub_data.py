import pandas as pd

input_filename = 'train.csv'
output_filename = f'sub_{input_filename}'
df = pd.read_csv(input_filename)
df[['id', 'comment_text']].iloc[:100].to_csv(output_filename, index=False)

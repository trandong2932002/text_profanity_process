import json
import string

words = set()
trans_table = str.maketrans({punc: ' ' for punc in string.punctuation.replace('-','')})
with open('./words.txt', 'r', encoding='utf-8') as f:
    for line in f.readlines():
        word = json.loads(line)['text'].strip().lower()
        word = word.translate(trans_table)
        for w in word.split():
            words.add(w)

with open('./words_alpha.txt', 'w+', encoding='utf-8') as f:
    for word in list(sorted(words)):
        f.write(f'{word}\n')
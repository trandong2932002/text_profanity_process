from operator import itemgetter
from unidecode import unidecode
import string
freq_dict = {}

with open('./vi_50k.txt', mode='r', encoding='utf-8') as f:
    for line in f:
        word, freq = line.split()
        freq = int(freq)
        no_accents_word = unidecode(word)
        if no_accents_word not in freq_dict:
            freq_dict[no_accents_word] = freq
        else:
            freq_dict[no_accents_word] += freq
with open('./vi_50k_no_accent.txt', mode='w+') as f:
    for word, freq in sorted(freq_dict.items(), key=itemgetter(1), reverse=True):
        if all([c not in string.ascii_lowercase for c in word]):
            # print(word)
            continue
        if ' ' in word:
            continue
        f.write(f'{word} {freq}\n')
        
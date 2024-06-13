import os

data_root = '/shared/new_dataset/'

vocab = open(os.path.join(data_root, "typst_vocab.txt")).readlines()
formulae = open(os.path.join(data_root, "formulas.norm.lst"), 'r').readlines()

char_to_idx = {x.split('\n')[0]:i for i, x in enumerate(vocab)}
char_to_idx['#UNK'] = len(char_to_idx)
char_to_idx['#START'] = len(char_to_idx)
char_to_idx['#END'] = len(char_to_idx)
idx_to_char = {y:x for x, y in char_to_idx.items()}

file_name = 'train.lst'
data_path = os.path.join(data_root, file_name)
file_list = open(data_path, 'r')

image_dir = os.path.join(data_root, 'images_processed')

set_list = []
missing = {}
for i, line in enumerate(file_list):
    form = formulae[int(line.split()[1])].strip().split()
    
    out_form = [char_to_idx['#START']]
    for c in form:
        try:
            out_form += [char_to_idx[c]]
        except:
            if c not in missing:
                print(c, " not found!")
                missing[c] = 1
            else:
                missing[c] += 1
                
            out_form += [char_to_idx['#UNK']]
            
    out_form += [char_to_idx['#END']]
    set_list.append([line.split()[0], out_form])
    
    image_file_name = line.split()[0]
    label = out_form

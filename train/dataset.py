import os
import numpy as np
import tensorflow as tf
from PIL import Image

class TypstDataset:
    def __init__(self, data_root = "/shared/new_dataset", set='train', batch_size=32):
        self.data_root = data_root
        self.set = set
        self.batch_size = batch_size
        self.train_dict = np.load(os.path.join(data_root, set + '_buckets.npy'), allow_pickle=True).tolist()
        self.data_length = np.sum([len(self.train_dict[x]) for x in self.train_dict.keys()])
        print("Length of %s data: " % set, self.data_length)
    
    def __len__(self):
        return self.data_length
    
    def __iter__(self):
        for keys in self.train_dict.keys():
            train_list = self.train_dict[keys]
            N_FILES = (len(train_list) // self.batch_size) * self.batch_size
            for batch_idx in range(0, N_FILES, self.batch_size):
                train_sublist = train_list[batch_idx:batch_idx + self.batch_size]
                imgs = []
                input_tokens = []
                label_tokens = []
                for x, y in train_sublist:
                    img = Image.open(os.path.join(self.data_root, 'images_processed/') + x)
                    img = img.resize((160, 160))
    
                    img = np.asarray(img)[:,:,0][:,:,None] / 255.0
                    
                    imgs.append(img)
                    input_tokens.append(y[:-1])
                    label_tokens.append(y[1:])

                imgs = np.asarray(imgs, dtype=np.float32).transpose(0,1,2,3)
                lens = [len(x) for x in input_tokens]

                Y_input_tokens = np.zeros((self.batch_size, max(lens)), dtype=np.int32)
                for i, input_token in enumerate(input_tokens):
                    Y_input_tokens[i, :len(input_token)] = input_token

                Y_label_tokens = np.zeros((self.batch_size, max(lens)), dtype=np.int32)
                for i, label_token in enumerate(label_tokens):
                    Y_label_tokens[i, :len(label_token)] = label_token

                yield imgs, Y_input_tokens, Y_label_tokens
    
    __call__ = __iter__



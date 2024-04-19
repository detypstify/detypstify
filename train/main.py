#!/usr/bin/env python3

import pandas as pd
import torch
from torch.utils.data import Dataset
from PIL import Image
from transformers import VisionEncoderDecoderModel, PreTrainedModel, TrOCRProcessor
from sklearn.model_selection import train_test_split

class TypstDataSet(Dataset):
    def __init__(self, df, processor, max_target_length = 128):
        self.df = df
        self.processor = processor
        self.max_target_length = max_target_length

    def __len__(self):
        return len(self.df)

    # add type signature
    def __getitem__(self, idx):
        file_name = self.df['file_name'][idx]
        text = self.df['text'][idx]
        image = Image.open(file_name).convert('RGB')
        pixel_values = self.processor(image, return_tensors="pt").pixel_values
        labels = self.processor.tokenizer(text, padding="max_length", max_length = self.max_target_length).input_ids
        labels = [label if label != self.processor.tokenizer.pad_token_id else -100 for label in labels]
        encoding = {"pixel_values" : pixel_values.squeeze(), "labels" : torch.tensor(labels)}
        return encoding



#
# print(
#         "CUDA AVAILABILITY: " + str(torch.cuda.is_available())
#         )
#
# device = torch.device("cuda")
# model = VisionEncoderDecoderModel.from_pretrained("microsoft/trocr-base-stage1")
# model.to(device)
#
#
# model.config.decoder_start_token_id = processor.tokenizer.cls_token_id


def read_in_dataset():
    formulas = []
    image_paths = []
    image_path_prefix = "../scraper/OUT/images"
    path = "../scraper/OUT/out"
    with open(path, 'r') as file:
        lines = file.readlines()
        i = 0
        while i < len(lines):
            formula = lines[i].strip()
            # should always exist
            j = i + 1
            while lines[j].strip()[-3:] != 'svg':
                formula += "\n" + lines[j].strip()
                j += 1
            i = j
            # print(lines[j].strip().split(','))
            image = lines[j].strip().split(',')[1]

            i = i + 1

            formulas.append(formula)
            image_paths.append(image_path_prefix + image)
    df = pd.DataFrame({'text': formulas, 'file_name': image_paths})
    return df

df = read_in_dataset()
train_df, test_df = train_test_split(df, test_size = 0.2)
train_df.reset_index(drop=True, inplace=True)
test_df.reset_index(drop=True, inplace=True)

processor = TrOCRProcessor.from_pretrained("microsoft/trocr-base-handwritten")
train_dataset = TypstDataSet(df = train_df, processor = processor)
eval_dataset = TypstDataSet(df = test_df, processor = processor)
encoding = train_dataset[0]
for k,v in encoding.items():
    print(k, v.shape)










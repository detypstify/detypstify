#!/usr/bin/env python3

import os
import pandas as pd
import torch
from torch.utils.data import Dataset
from PIL import Image
from transformers import VisionEncoderDecoderModel, PreTrainedModel, TrOCRProcessor
from sklearn.model_selection import train_test_split
from torch.utils.data import DataLoader
from datasets import load_metric
from torch.optim import AdamW
from tqdm.notebook import tqdm

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


def read_in_dataset():
    formulas = []
    image_paths = []
    image_path_prefix = "../scraper/OUT/images/"
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
            image_png = image[0:-3] + 'png'
            # check if the file exists on the filesystem
            if os.path.exists(image_path_prefix + image):
                formulas.append(formula)
                image_paths.append(image_path_prefix + image_png)
            i = i + 1

    df = pd.DataFrame({'text': formulas, 'file_name': image_paths})
    return df

df = read_in_dataset()
print(df.size)
train_df, test_df = train_test_split(df, test_size = 0.2)
train_df.reset_index(drop=True, inplace=True)
test_df.reset_index(drop=True, inplace=True)

processor = TrOCRProcessor.from_pretrained("microsoft/trocr-base-printed")
train_dataset = TypstDataSet(df = train_df, processor = processor)
eval_dataset = TypstDataSet(df = test_df, processor = processor)
encoding = train_dataset[0]
for k,v in encoding.items():
    print(k, v.shape)

train_dataloader = DataLoader(train_dataset, batch_size=4, shuffle=True)
eval_dataloader = DataLoader(eval_dataset, batch_size = 4)

print(
        "CUDA AVAILABILITY: " + str(torch.cuda.is_available())
        )

print(
        "CUDA VERSION: " + str(torch.version.cuda)
        )

device = torch.device("cuda")
model = VisionEncoderDecoderModel.from_pretrained("microsoft/trocr-base-stage1")
model.to(device)
# set special tokens used for creating the decoder_input_ids from the labels
model.config.decoder_start_token_id = processor.tokenizer.cls_token_id
model.config.pad_token_id = processor.tokenizer.pad_token_id
model.config.vocab_size = model.config.decoder.vocab_size
model.config.eos_token_id = processor.tokenizer.sep_token_id
model.config.max_length = 5000
model.config.early_stopping = True
model.config.no_repeat_ngram_size = 3
model.config.length_penalty = 0.2
model.config.num_beams = 4

cer_metric = load_metric("cer")

def compute_cer(pred_ids, label_ids, p, metric):
    pred_str = p.batch_decode(pred_ids, skip_special_tokens=True)
    label_ids[label_ids == -100] = p.tokenizer.pad_token_id
    label_str = p.batch_decode(label_ids, skip_special_tokens=True)

    cer = metric.compute(predictions=pred_str, references=label_str)
    return cer

optimizer = AdamW(model.parameters(), lr=5e-5)

for epoch in range(10):  # loop over the dataset multiple times
    print("EPOCH: " + str(epoch))
    # train
    model.train()
    print("trained")
    train_loss = 0.0
    for batch in tqdm(train_dataloader):
        print("doing batch")
        # get the inputs
        for k,v in batch.items():
            print("sending batch to device")
            batch[k] = v.to(device)

       # forward + backward + optimize
        print("tf is this doing")
        outputs = model(**batch)
        print("umm")
        loss = outputs.loss
        print("going backward")
        loss.backward()
        print("optimizer")
        optimizer.step()
        print("grad")
        optimizer.zero_grad()

        print("traloss")
        train_loss += loss.item()

    print(f"Loss after epoch {epoch}:", train_loss/len(train_dataloader))

    # evaluate
    model.eval()
    valid_cer = 0.0
    with torch.no_grad():
        for batch in tqdm(eval_dataloader):
            # run batch generation
            outputs = model.generate(batch["pixel_values"].to(device))
            # compute metrics
            cer = compute_cer(outputs, batch["labels"], processor, cer_metric)
            valid_cer += cer

    print("Validation CER:", valid_cer / len(eval_dataloader))

model.save_pretrained(".")







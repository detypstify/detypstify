#!/usr/bin/env python3

import os
import signal

import tensorflow as tf
print(tf.__version__)

from dataset import *
from model import *
from train import *

data_root = "/shared/new_dataset"
properties = np.load(os.path.join(data_root, 'properties.npy'), allow_pickle=True).tolist()
vocab = open(os.path.join(data_root, "typst_vocab.txt")).readlines()

word_to_index = {x.split('\n')[0]:i for i, x in enumerate(vocab)}
word_to_index['#UNK'] = len(word_to_index)
word_to_index['#START'] = len(word_to_index)
word_to_index['#END'] = len(word_to_index)


image_size = 160
patch_size = 6
num_patches = (image_size // patch_size) ** 2
projection_dim = 512
num_heads = 4
transformer_units = [projection_dim * 2, projection_dim]
transformer_layers = 8
mlp_head_units = [2048, 1024]
set = 'test'

test_dict = np.load(os.path.join(data_root, set + '_buckets.npy'), allow_pickle=True).tolist()
test_dict.keys()
train_ds_gen = TypstDataset(set='train', batch_size=16) 
train_ds = tf.data.Dataset.from_generator(train_ds_gen, (tf.float32, tf.int32, tf.int32))

test_ds_gen = TypstDataset(set='test', batch_size=16) 
test_ds = tf.data.Dataset.from_generator(test_ds_gen, (tf.float32, tf.int32, tf.int32))

vit_classifier = create_vit_classifier()

vocabulary_size = len(word_to_index)
output_layer = TokenOutput(vocabulary_size, banned_tokens=('', '#UNK', '#START'))

model = Captioner(vocabulary_size, feature_extractor=create_vit_classifier(), output_layer=output_layer,
                  units=256, dropout_rate=0.5, num_layers=4, num_heads=8)


learning_rate = tf.keras.optimizers.schedules.PolynomialDecay(1e-4, int(100000 / 32.0 * 1000), 1e-6)
optimizer = tf.keras.optimizers.AdamW(learning_rate=learning_rate, weight_decay=0.0001)
writer = tf.summary.create_file_writer("tensorboard")
callbacks = [GenerateText(test_dict), tf.keras.callbacks.EarlyStopping(patience=5, restore_best_weights=True)]

for epoch in range(0, 1000000):
    print("epoch: ", epoch)

    train_losses = []
    step = 0
    for ds_train in train_ds:
        loss = train(ds_train, model, optimizer)
        train_losses.append(loss)
        step += 1

    model.save_weights('model/model_' + str(epoch))

    mean_loss_train = np.mean(train_losses)
    mean_perp_train = np.mean(list(map(lambda x: np.power(np.e, x), train_losses)))

    test_accuracies = []
    for ds_test in test_ds:
        accuracy = test(ds_test, model)
        test_accuracies.append(accuracy)

    mean_accuracy_test = np.mean(test_accuracies)

    print("Mean train loss:", mean_loss_train, ", Mean train perplexity:", mean_perp_train, "Mean test Accuracy:", mean_accuracy_test)
    with writer.as_default():
        tf.summary.scalar("mean_loss_train", mean_loss_train, step=epoch)
        tf.summary.scalar("mean_perp_train", mean_perp_train, step=epoch)
        tf.summary.scalar("mean_accuracy_test", mean_accuracy_test, step=epoch)
        writer.flush()

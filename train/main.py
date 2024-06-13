#!/usr/bin/env python3

import os
import signal

import tensorflow as tf
print(tf.__version__)

from dataset import *
from model import *

def signal_handler(sig, frame):
    print("Ctrl-C pressed. Program paused. Press 'c' to continue and q to quit")
    signal.signal(signal.SIGINT, signal.SIG_IGN)  # Ignore Ctrl-C during pause
    while True:
        user_input = input()
        if user_input.lower() == 'c':
            print("Resuming program...")
            signal.signal(signal.SIGINT, signal_handler)  # Restore Ctrl-C handler
            break
        elif user_input == 'q':
            print("Quitting program...")
            exit()

def masked_loss(labels, preds):
  loss = tf.nn.sparse_softmax_cross_entropy_with_logits(labels, preds)

  mask = (labels != 0) & (loss < 1e8)
  mask = tf.cast(mask, loss.dtype)

  loss = loss * mask
  loss = tf.reduce_sum(loss)/tf.reduce_sum(mask)

  return loss


def masked_acc(labels, preds):
  mask = tf.cast(labels != 0, tf.float32)
  preds = tf.argmax(preds, axis=-1)
  labels = tf.cast(labels, tf.int64)
  match = tf.cast(preds == labels, mask.dtype)
  acc = tf.reduce_sum(match * mask) / tf.reduce_sum(mask)

  return acc

class GenerateText(tf.keras.callbacks.Callback):
  def __init__(self):
    ex_path = test_dict[(400, 160)][0][0]
    image_dir = os.path.join(data_root, 'images_processed')
    image_dir = os.path.join(image_dir, ex_path)

    self.image = Image.open(image_dir).convert('YCbCr')
    self.image = self.image.resize((image_size, image_size))
    self.image = np.asarray(self.image)[:,:,0][:,:,None]

  def on_epoch_end(self, epochs=None, logs=None):
    print()
    print()
    for t in (0.0, 0.5, 1.0):
      result = self.model.simple_gen(self.image, temperature=t)
      print(result)

    print()


@tf.function
def train(ds):
    labels = ds[2]

    with tf.GradientTape() as tape:
        logits = model(ds[0], ds[1])

        loss = tf.nn.sparse_softmax_cross_entropy_with_logits(labels, logits)

        mask = (labels != 0) & (loss < 1e8)
        mask = tf.cast(mask, loss.dtype)

        loss = loss * mask
        loss = tf.reduce_sum(loss) / tf.reduce_sum(mask)

    grads = tape.gradient(loss, model.trainable_variables)
    #(grads, _) = tf.clip_by_global_norm(grads, clip_norm=1.0)
    optimizer.apply_gradients(zip(grads, model.trainable_variables))

    return loss


@tf.function
def test(ds):
    labels = ds[2]

    with tf.GradientTape() as tape:
        preds = model(ds[0], ds[1])

        mask = tf.cast(labels != 0, tf.float32)
        preds = tf.argmax(preds, axis=-1)
        labels = tf.cast(labels, tf.int64)
        match = tf.cast(preds == labels, mask.dtype)
        acc = tf.reduce_sum(match * mask) / tf.reduce_sum(mask)

    return acc

signal.signal(signal.SIGINT, signal_handler)  # Register Ctrl-C handler

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
g = GenerateText()
g.model = model
g.on_epoch_end(0)

callbacks = [GenerateText(), tf.keras.callbacks.EarlyStopping(patience=5, restore_best_weights=True)]

for epoch in range(0, 1000000):
    print("epoch: ", epoch)

    train_losses = []
    step = 0
    for ds_train in train_ds:
        loss = train(ds_train)
        train_losses.append(loss)
        step += 1

    model.save_weights('model/model_' + str(epoch))

    mean_loss_train = np.mean(train_losses)
    mean_perp_train = np.mean(list(map(lambda x: np.power(np.e, x), train_losses)))

    test_accuracies = []
    for ds_test in test_ds:
        accuracy = test(ds_test)
        test_accuracies.append(accuracy)

    mean_accuracy_test = np.mean(test_accuracies)

    print("Mean train loss:", mean_loss_train, ", Mean train perplexity:", mean_perp_train, "Mean test Accuracy:", mean_accuracy_test)
    with writer.as_default():
        tf.summary.scalar("mean_loss_train", mean_loss_train, step=epoch)
        tf.summary.scalar("mean_perp_train", mean_perp_train, step=epoch)
        tf.summary.scalar("mean_accuracy_test", mean_accuracy_test, step=epoch)
        writer.flush()

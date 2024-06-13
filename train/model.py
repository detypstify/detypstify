import tensorflow as tf
import os

image_size = 160
patch_size = 6
num_patches = (image_size // patch_size) ** 2
projection_dim = 512
num_heads = 4
transformer_units = [projection_dim * 2, projection_dim]
transformer_layers = 8
mlp_head_units = [2048, 1024]
data_root = "/shared/new_dataset"
vocab = open(os.path.join(data_root, "typst_vocab.txt")).readlines()

word_to_index = {x.split('\n')[0]:i for i, x in enumerate(vocab)}
word_to_index['#UNK'] = len(word_to_index)
word_to_index['#START'] = len(word_to_index)
word_to_index['#END'] = len(word_to_index)
index_to_word = {y:x for x, y in word_to_index.items()}

index_to_words = lambda Y: ' '.join(map(lambda x: properties['idx_to_char'][x], Y))


def mlp(x, hidden_units, dropout_rate):
    for units in hidden_units:
        x = tf.keras.layers.Dense(units, activation=tf.keras.activations.gelu)(x)
        x = tf.keras.layers.Dropout(dropout_rate)(x)
    return x


class Patches(tf.keras.layers.Layer):
    def __init__(self, patch_size):
        super().__init__()
        self.patch_size = patch_size

    def call(self, images):
        batch_size = tf.shape(images)[0]
        patches = tf.image.extract_patches(images=images, sizes=[1, self.patch_size, self.patch_size, 1],
            strides=[1, self.patch_size, self.patch_size, 1], rates=[1, 1, 1, 1], padding="VALID")
        
        patch_dims = patches.shape[-1]
        patches = tf.reshape(patches, [batch_size, -1, patch_dims])
        
        return patches


class PatchEncoder(tf.keras.layers.Layer):
    def __init__(self, num_patches, projection_dim):
        super().__init__()
        self.num_patches = num_patches
        self.projection = tf.keras.layers.Dense(units=projection_dim)
        self.position_embedding = tf.keras.layers.Embedding(input_dim=num_patches, output_dim=projection_dim)

    def call(self, patch):
        positions = tf.range(start=0, limit=self.num_patches, delta=1)
        encoded = self.projection(patch) + self.position_embedding(positions)
        
        return encoded
    

def create_vit_classifier():
    inputs = tf.keras.Input(shape=(image_size, image_size, 1))
    
    patches = Patches(patch_size)(inputs)
    encoded_patches = PatchEncoder(num_patches, projection_dim)(patches)

    for _ in range(transformer_layers):
        x1 = tf.keras.layers.LayerNormalization(epsilon=1e-6)(encoded_patches)
        attention_output = tf.keras.layers.MultiHeadAttention(num_heads=num_heads, key_dim=projection_dim, dropout=0.1)(x1, x1)
        x2 = tf.keras.layers.Add()([attention_output, encoded_patches])
        x3 = tf.keras.layers.LayerNormalization(epsilon=1e-6)(x2)
        x3 = mlp(x3, hidden_units=transformer_units, dropout_rate=0.1)
        encoded_patches = tf.keras.layers.Add()([x3, x2])

    representation = tf.keras.layers.LayerNormalization(epsilon=1e-6)(encoded_patches)
    model = tf.keras.Model(inputs=inputs, outputs=representation)
    
    return model


class SeqEmbedding(tf.keras.layers.Layer):
  def __init__(self, vocab_size, max_length, depth):
    super().__init__()

    self.pos_embedding = tf.keras.layers.Embedding(input_dim=max_length, output_dim=depth)
    self.token_embedding = tf.keras.layers.Embedding(input_dim=vocab_size, output_dim=depth, mask_zero=True)
    self.add = tf.keras.layers.Add()

  def call(self, seq):
    seq = self.token_embedding(seq) # (batch, seq, depth)

    x = tf.range(tf.shape(seq)[1])  # (seq)
    x = x[tf.newaxis, :]  # (1, seq)
    x = self.pos_embedding(x)  # (1, seq, depth)

    return self.add([seq, x])

class CausalSelfAttention(tf.keras.layers.Layer):
  def __init__(self, **kwargs):
    super().__init__()
    self.mha = tf.keras.layers.MultiHeadAttention(**kwargs)
    self.add = tf.keras.layers.Add()
    self.layernorm = tf.keras.layers.LayerNormalization()

  def call(self, x):
    attn = self.mha(query=x, value=x, use_causal_mask=True)
    x = self.add([x, attn])

    return self.layernorm(x)


class CrossAttention(tf.keras.layers.Layer):
  def __init__(self,**kwargs):
    super().__init__()
    self.mha = tf.keras.layers.MultiHeadAttention(**kwargs)
    self.add = tf.keras.layers.Add()
    self.layernorm = tf.keras.layers.LayerNormalization()

  def call(self, x, y, **kwargs):
    # x.shape:  TensorShape([1, 29, 256])
    # y.shape:  TensorShape([1, 1600, 512])
    #tf.print("x.shape: ", x.shape)
    #tf.print("y.shape: ", y.shape)

    attn, attention_scores = self.mha(query=x, value=y, return_attention_scores=True)
    self.last_attention_scores = attention_scores
    x = self.add([x, attn])

    return self.layernorm(x)


class FeedForward(tf.keras.layers.Layer):
  def __init__(self, units, dropout_rate=0.1):
    super().__init__()
    self.seq = tf.keras.Sequential([
        tf.keras.layers.Dense(units=2*units, activation='relu'),
        tf.keras.layers.Dense(units=units),
        tf.keras.layers.Dropout(rate=dropout_rate),
    ])

    self.layernorm = tf.keras.layers.LayerNormalization()

  def call(self, x):
    x = x + self.seq(x)
    return self.layernorm(x)


class DecoderLayer(tf.keras.layers.Layer):
  def __init__(self, units, num_heads=1, dropout_rate=0.1):
    super().__init__()

    self.self_attention = CausalSelfAttention(num_heads=num_heads, key_dim=units, dropout=dropout_rate)
    self.cross_attention = CrossAttention(num_heads=num_heads,key_dim=units, dropout=dropout_rate)
    self.ff = FeedForward(units=units, dropout_rate=dropout_rate)

  def call(self, inputs, training=False):
    in_seq, out_seq = inputs

    out_seq = self.self_attention(out_seq)
    out_seq = self.cross_attention(out_seq, in_seq)

    self.last_attention_scores = self.cross_attention.last_attention_scores
    out_seq = self.ff(out_seq)

    return out_seq

class TokenOutput(tf.keras.layers.Layer):
  def __init__(self, vocabulary_size, banned_tokens=('', '[UNK]', '[START]'), **kwargs):
    super().__init__()

    self.dense = tf.keras.layers.Dense(units=vocabulary_size, **kwargs)
    self.banned_tokens = banned_tokens

  def call(self, x):
    x = self.dense(x)
    return x

class Captioner(tf.keras.Model):
  def __init__(self, vocabulary_size, feature_extractor, output_layer, num_layers=1,
               units=256, max_length=200, num_heads=1, dropout_rate=0.1):
    super().__init__()
    self.feature_extractor = feature_extractor

    vocab = open(os.path.join(data_root, "typst_vocab.txt")).readlines()
    self.word_to_index = {x.split('\n')[0]:i for i, x in enumerate(vocab)}
    self.word_to_index['#UNK'] = len(self.word_to_index)
    self.word_to_index['#START'] = len(self.word_to_index)
    self.word_to_index['#END'] = len(self.word_to_index)
    self.index_to_word = {y:x for x, y in self.word_to_index.items()}

    self.seq_embedding = SeqEmbedding(vocab_size=vocabulary_size, depth=units, max_length=max_length)

    self.decoder_layers = [
        DecoderLayer(units, num_heads=num_heads, dropout_rate=dropout_rate) for n in range(num_layers)]

    self.output_layer = output_layer

  def call(self, image, txt):
    #image.shape 1:  TensorShape([1, 160, 640, 1])

    image = self.feature_extractor(image)
    #image.shape 2:  TensorShape([1, 20, 80, 512])

    #image = einops.rearrange(image, 'b h w c -> b (h w) c')
    #image.shape 3:  TensorShape([1, 1600, 512])

    txt = self.seq_embedding(txt)

    for dec_layer in self.decoder_layers:
      txt = dec_layer(inputs=(image, txt))

    txt = self.output_layer(txt)

    return txt

  def simple_gen(self, image, temperature=1):
    initial = self.word_to_index['#START'] # (batch, sequence)
    initial = tf.expand_dims(initial, 0)
    initial = tf.expand_dims(initial, 0)

    tokens = initial # (batch, sequence)

    for n in range(50):
      preds = self(image[tf.newaxis, ...], tokens).numpy()  # (batch, sequence, vocab)
      preds = preds[:,-1, :]  #(batch, vocab)
      if temperature == 0:
          next = tf.argmax(preds, axis=-1)[:, tf.newaxis]  # (batch, 1)
      else:
          next = tf.random.categorical(preds / temperature, num_samples=1)  # (batch, 1)

      next = tf.cast(next, tf.int32)

      tokens = tf.concat([tokens, next], axis=1) # (batch, sequence)

      if next[0] == self.word_to_index['#END']:
        break

    words = []
    for token in tokens[0, 1:-1]:
        word = index_to_word[token.numpy()]
        words.append(word)

    result = tf.strings.reduce_join(words, axis=-1, separator=' ')

    return result.numpy().decode()

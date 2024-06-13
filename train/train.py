import tensorflow as tf

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



@tf.function
def train(ds, model, optimizer):
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
def test(ds, model):
    labels = ds[2]

    with tf.GradientTape() as tape:
        preds = model(ds[0], ds[1])

        mask = tf.cast(labels != 0, tf.float32)
        preds = tf.argmax(preds, axis=-1)
        labels = tf.cast(labels, tf.int64)
        match = tf.cast(preds == labels, mask.dtype)
        acc = tf.reduce_sum(match * mask) / tf.reduce_sum(mask)

    return acc

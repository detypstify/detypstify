import random

model.load_weights('model/model_464')

plt.figure(figsize=(40, 40))

data_root = 'latex_data'
set = 'test'
test_dict = np.load(os.path.join(data_root, set + '_buckets.npy'), allow_pickle=True).tolist()
data_length = np.sum([len(test_dict[x]) for x in test_dict.keys()])
print("Length of %s data: " % set, data_length)

key_list = list(test_dict.keys())
key = random.choice(key_list)
test_list = test_dict[key]
test_image_info = random.choice(test_list)

img = Image.open(os.path.join(data_root, 'images_processed/') + test_image_info[0])
img = img.resize((image_size * 4, image_size))
img = np.asarray(img)[:,:,0][:,:,None] / 255.0
plt.imshow(img, cmap="gray")

Y = np.array(test_image_info[1])

preds_chars = index_to_words(Y[1:]).replace('$','')
preds_chars = preds_chars.split('#END')[0]

print("Label: ")
displayPreds(preds_chars)

result = model.simple_gen(img, temperature=0.0)
print("Prediction: ")
displayPreds(result)

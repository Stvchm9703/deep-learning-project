# %% [markdown]
# author : Ho Man Cheng
# description : for training a facial type model

# %%
# import dataset
import pandas as pd
from util import (DATASET_PATH )
import tensorflow
from tensorflow.keras import (
    models,
    layers,
    optimizers,
    callbacks,
    applications,
    regularizers,
    initializers,
)
from tensorflow.keras.callbacks import ModelCheckpoint
from tensorflow.keras.preprocessing.image import ImageDataGenerator

import os

os.environ["KERAS_BACKEND"] = "tensorflow"  # Or "jax" or "torch"!

import keras_cv

# %%
def load_batch_datagen(bat_num):
    train_datagen = ImageDataGenerator(
        rotation_range=40,
        width_shift_range=0.2,
        height_shift_range=0.2,
        shear_range=0.2,
        zoom_range=0.2,
        horizontal_flip=True,
        fill_mode="nearest",
        validation_split=0.2,
    )

    test_datagen = ImageDataGenerator()

    train_generator = train_datagen.flow_from_directory(
        directory=f"{DATASET_PATH}/face_shape__batch_{bat_num}/training",
        subset="training",
        target_size=(256, 256),
        class_mode="categorical",
    )

    validation_generator = train_datagen.flow_from_directory(
        directory=f"{DATASET_PATH}/face_shape__batch_{bat_num}/training",
        subset="validation",
        target_size=(256, 256),
        class_mode="categorical",
    )

    test_generator = test_datagen.flow_from_directory(
        directory=f"{DATASET_PATH}/face_shape__batch_{bat_num}/testing",
        target_size=(256, 256),
        class_mode="categorical",
    )

    return train_generator, validation_generator, test_generator

# %%
class InceptionV4B_ApplMod(tensorflow.keras.Model):
    def __init__(self, **kwarg):
        super(InceptionV4B_ApplMod, self).__init__(name="")
        input_shape = kwarg.get("input_shape", (256, 256, 3))
        self.__channel_axis = (
            1
            if tensorflow.keras.backend.image_data_format() == "channels_first"
            else -1
        )
        # base input and stem
        self.input_layer = models.Sequential(
            [
                layers.Input(input_shape),
                layers.Conv2D(32, (3, 3), strides=(2, 2), activation="relu"),
                layers.Conv2D(32, (3, 3), activation="relu"),
                layers.Conv2D(64, (3, 3), activation="relu"),
            ],
            name="input_layer",
        )

        self.stem_0_a = models.Sequential(
            [
                layers.Conv2D(64, (3, 3), activation="relu", padding="valid"),
                layers.Conv2D(96, (3, 3), activation="relu", padding="valid"),
                layers.Conv2D(128, (3, 3), activation="relu", padding="valid"),
                layers.Conv2D(128, (3, 3), activation="relu", padding="valid"),
                # 117 x 117 x 128
                # layers.Conv2D(128, (3, 3), activation="relu", padding="valid"),
            ],
            name="stem0_a",
        )
        self.stem_0_b = models.Sequential(
            [
                layers.Conv2D(32, (1, 1), activation="relu", padding="valid"),
                layers.Conv2D(128, (9, 9), activation="relu", padding="valid"),
                # 122 x 122 x 128
            ],
            name="stem0_b",
        )

        self.stem_1_a = models.Sequential(
            [
                layers.Conv2D(64, (8, 8), activation="relu"),
                layers.Conv2D(32, (1, 1), activation="relu"),
            ],
            name="stem1_a",
        )
        self.stem_1_b = models.Sequential(
            [
                layers.Conv2D(64, (1, 1), activation="relu"),
                layers.Conv2D(96, (3, 3), activation="relu"),
                layers.Conv2D(120, (4, 4), activation="relu"),
                layers.Conv2D(32, (3, 3), activation="relu"),
            ],
            name="stem1_b",
        )

        self.stem_a1 = models.Sequential(
            [
                layers.Conv2D(64, (1, 1), activation="relu"),
                layers.Conv2D(96, (3, 3), activation="relu"),
                layers.Conv2D(120, (4, 4), activation="relu"),
                layers.Conv2D(32, (3, 3), activation="relu"),
            ],
            name="stem1_b",
        )

        # self.stem_2_a =
        self.classifier = models.Sequential(
            [
                layers.AveragePooling2D((8, 8), padding="valid"),
                layers.Flatten(),
                # layers.Dense(2048, activation="relu"),
                layers.Dense(5, activation="softmax"),
            ],
            name="classifier",
        )

    def call(self, input_tensor, training=False):

        # x = layers.Input((256,256,3))(input_tensor)
        x = self.input_layer(input_tensor)
        x_1 = self.stem_0_a(x)
        x_2 = self.stem_0_b(x)
        x = layers.concatenate([x_1, x_2], axis=-1)
        x_1 = self.stem_1_a(x)
        x_2 = self.stem_1_b(x)
        x = layers.concatenate([x_1, x_2], axis=-1)

        x = self.classifier(x)
        return x

with tensorflow.device('/CPU:0'):
    model_base = InceptionV4B_ApplMod()


# %%

    model_base.compile(
        loss="categorical_crossentropy",
        optimizer="adam",
        metrics=["acc"],
    )

    filepath = "./weights/face_shape__InceptionV4B_ApplMod__modified.{epoch:02d}-{val_loss:.2f}.keras"

    checkpoint = callbacks.ModelCheckpoint(
        filepath,
        monitor="val_loss",
        verbose=0,
        save_best_only=False,
        save_weights_only=False,
        mode="auto",
        save_freq="epoch",
    )

    # today = datetime.now()

    cv_logging = callbacks.CSVLogger(f"./logs/training.csv", separator=",", append=False)

# %%
# from IPython.core.interactiveshell import InteractiveShell
# InteractiveShell.ast_node_interactivity = "all"
# for batch in [0,1,2]:
    batch = 0
    train_generator, validation_generator, test_generator = load_batch_datagen(batch)

    history = model_base.fit(
        train_generator,
        steps_per_epoch=50,  # 2000 images = batch_size * steps
        epochs=50,
        validation_data=validation_generator,
        validation_steps=10,  # 1000 images = batch_size * steps
        callbacks=[checkpoint],
    )

    # %%
    model_base.summary()

    # %%
    loss = history.history["loss"]
    val_loss = history.history["val_loss"]

    epochs = range(len(loss))
    import matplotlib.pyplot as plt
    plt.figure()
    plt.plot(epochs, loss, "r", label="Training loss")
    plt.plot(epochs, val_loss, "b", label="Validation loss")
    plt.title("Training and validation loss")
    plt.legend()

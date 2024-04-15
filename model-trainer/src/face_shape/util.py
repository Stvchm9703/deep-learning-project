import os
from os import path, environ
import zipfile
from sqids import Sqids

import pandas as pd
import numpy as np
import os

# import tensorflow as tf
import glob
import cv2
from sklearn.model_selection import train_test_split

from pathlib import Path
import tensorflow as tf
import uuid
import  skimage

# from PIL import Image

GEN_SQIDS = Sqids()

PROJECT_BASE_PATH = (
    os.getenv("PORJECT_BASE_PATH")
    if "PORJECT_BASE_PATH" in environ.keys()
    else "/home/ghost/git_src/deep-learninf-project/model-trainer"
)

DATASET_PATH = (
    os.getenv("PORJECT_DATASET_PATH")
    if "PORJECT_DATASET_PATH" in environ.keys()
    else path.join(PROJECT_BASE_PATH, "dataset")
)

SIZE_SETTING = 256


def extract_zip_data(input_path: str, dataset_name: str):
    # list the existed
    tmp_folder = "/tmp/dl-model-trainer"

    # extract the zip file
    with zipfile.ZipFile(input_path, "r") as file:
        # extract the file
        tmp_path = path.join(tmp_folder, GEN_SQIDS.encode([len(dataset_name)]))
        file.extractall(tmp_path)
        return tmp_path
    pass


def load_image_dataset(folder_path: str, dataset_name: str):
    data_list = []
    tmp_file_list = []
    image_type = ("jpg", "png")
    for tmp_type in image_type:
        tmp_file_list.extend(
            glob.glob(f"{folder_path}/**/*.{tmp_type}", recursive=True)
        )

    for file_inst in tmp_file_list:
        pathlist = file_inst.replace(folder_path, "").split("/")
        feature_label = list(
            filter(
                lambda x: (
                    (x + "").__contains__("train")
                    or (x + "").__contains__("test")
                    or (x + "").__contains__("valid")
                    or (x + "").__contains__(".jpg")
                    or (x + "").__contains__(".png")
                )
                == False,
                pathlist,
            )
        )[-1]
        feature_label = feature_label.lower()
        feature_label = "oval" if feature_label == "ovale" else feature_label
        tar_name = f"{uuid.uuid3(uuid.NAMESPACE_OID, dataset_name + '_' + file_inst )}.jpg"
        if path.isfile(f"{DATASET_PATH}/{dataset_name}/{tar_name}"):
            os.remove(f"{DATASET_PATH}/{dataset_name}/{tar_name}")

        try:
            _ = skimage.io.imread(file_inst)
            img = cv2.imread(file_inst)
            if img is not None:
                cv2.imwrite(f"{DATASET_PATH}/{dataset_name}/{tar_name}", img)
                data_list.append(
                    {
                        "label": feature_label,
                        "name": tar_name,
                        "file": f"{DATASET_PATH}/{dataset_name}/{tar_name}",
                    }
                )
            # Do stuff with img
        except Exception as e:
            print('bad file, skip ', file_inst)
            continue

    return pd.DataFrame(data_list)


def merge_and_shuffle_data(dataset_list: list[pd.DataFrame]):
    df = pd.concat(dataset_list, ignore_index=True)

    return train_test_split(df, test_size=0.3)


def clean_up_zip():
    import shutil

    shutil.rmtree("/tmp/dl-model-trainer")


def convert_tf_set_2d(x_set, y_set):
    """
    Convert the 2D data set to TensorFlow tensors.

    Args:
        x_set (pandas.DataFrame): The input data set.
        y_set (pandas.DataFrame): The target data set.

    Returns:
        tuple: A tuple containing the converted input and target data sets as TensorFlow tensors.
    """
    x_data = []
    for data in x_set["data"]:
        x_data.append(tf.convert_to_tensor(data))
    x_set = tf.convert_to_tensor(x_data)
    y_set = tf.convert_to_tensor(y_set["label"])

    return x_set, y_set

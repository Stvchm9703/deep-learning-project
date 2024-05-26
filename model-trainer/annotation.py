# STEP 1: Import the necessary modules.
import numpy as np
import mediapipe as mp
from mediapipe.tasks import python
from mediapipe.tasks.python import vision
import PIL
import numpy as np


def init():

    model_file = open("blaze_face_short_range.tflite", "rb")
    model_data = model_file.read()
    model_file.close()

    # STEP 2: Create an FaceDetector object.
    base_options = python.BaseOptions(model_asset_buffer=model_data)
    options = vision.FaceDetectorOptions(base_options=base_options)
    detector = vision.FaceDetector.create_from_options(options)

    return detector


def create_categories(cat_type):
    """
    {
        "id": 1,
        "name": "Ready",
        "supercategory": ""
    },
    """
    return {"name": cat_type, "supercategory": ""}


def convert_annotation(detector, img_path, cat_id):
    """
    annotation{
        "id": int,
        "image_id": int,
        "category_id": int,
        "segmentation": RLE or [polygon],
        "area": float,
        "bbox": [x,y,width,height],
        "iscrowd": 0 or 1,
    }

    """
    # STEP 3: Load the input image.
    image = mp.Image.create_from_file(img_path)
    image_ = PIL.Image.open(img_path)
    image_width, image_height = image_.size
    # STEP 4: Detect faces in the input image.
    detection_result = detector.detect(image)
    if len(detection_result.detections) == 0:
        return None

    detection_list = []
    for detection in detection_result.detections:
        bbox = detection.bounding_box
        bbox.origin_x = np.clip(bbox.origin_x - int(bbox.width * 0.125), 0, image_width)
        bbox.origin_y = np.clip(
            bbox.origin_y - int(bbox.height * 0.125), 0, image_height
        )
        bbox.width = np.clip(int(1.25 * bbox.width), 0, image_width)
        bbox.height = np.clip(int(1.25 * bbox.height), 0, image_height)

        detection_list.append(
            {
                # "id": int,
                # "image_id": int,
                "image_name": img_path,
                "category_id": cat_id,
                "segmentation": None,
                "area": 0,
                "bbox": [bbox.origin_x, bbox.origin_y, bbox.width, bbox.height],
                "iscrowd": 0,
            }
        )
    return detection_list


def create_image_info(img_path):
    # {
    #     "id": 1,
    #     "width": 1920,
    #     "height": 1080,
    #     "file_name": "100520.jpg",
    #     "license": 0,
    #     "flickr_url": "",
    #     "coco_url": "",
    #     "date_captured": 0,
    # },
    image_ = PIL.Image.open(img_path)
    return {
        "width": image_.size[0],
        "height": image_.size[1],
        "file_name": img_path,
        "license": 0,
        "flickr_url": "",
        "coco_url": "",
        "date_captured": 0,
    }


import pandas as pd
import glob
import re


def process_main(dataset_name, dataset_type):
    detector = init()
    file_list = glob.glob(f"dataset/{dataset_name}/{dataset_type}/**/*.jpg")
    # regex = re.compile("dataset/face_shape/()/**/*.jpg")
    pattern = r"dataset/([^/]+)/(?P<dataset_type>[^/]+)/(?P<face_shape_type>[^/]+)/(?P<file_name>[^/]+\.jpg)"

    img_list = []
    cat_list = []
    ann_list = []

    for file_path in file_list:
        # Regex pattern
        # Using re.match to apply the regex pattern
        match = re.match(pattern, file_path)
        if match:
            dataset_type = match.group("dataset_type")
            face_shape_type = match.group("face_shape_type")
            file_name = match.group("file_name")

        image_info = create_image_info(file_path)
        image_info["raw_path"] = file_path
        if file_name:
            image_info["file_name"] = f"{face_shape_type}/{file_name}"
        img_list.append(image_info)
        detected = convert_annotation(detector, file_path, face_shape_type)
        ann_list.extend(detected)
        cat_list.append(create_categories(face_shape_type))

    # print(img_list)
    img_list_df = pd.DataFrame(img_list)
    img_list_df["image_id"] = img_list_df.index + 1

    cat_list_df = pd.DataFrame(cat_list)
    cat_list_df = cat_list_df.drop_duplicates()
    cat_list_df = cat_list_df.reset_index()
    cat_list_df["id"] = cat_list_df.index + 1

    ann_list_df = pd.DataFrame(ann_list)
    ann_list_df = ann_list_df.merge(
        img_list_df[["image_id", "raw_path"]], left_on="image_name", right_on="raw_path"
    )
    ann_list_df = ann_list_df.merge(
        cat_list_df[["name", "id"]], left_on="category_id", right_on="name"
    )
    ann_list_df["category_id"] = ann_list_df["id"]

    ann_list_df["area"] = ann_list_df["bbox"].apply(lambda x: x[2] * x[3])
    ann_list_df = ann_list_df.drop(["image_name", "name", "raw_path"], axis=1)
    ann_list_df["id"] = ann_list_df.index + 1

    img_list_df = img_list_df.drop(["raw_path"], axis=1)
    img_list_df = img_list_df.rename(columns={"image_id": "id"})

    annotation_obj = {
        "info": {
            "contributor": "",
            "date_created": "18-05-2024",
            "description": "face-shape dataset",
            "url": "NA",
            "version": "1",
            "year": "2024",
        },
        "licenses": [
            {
                "name": "CC BY 4.0",
                "id": 0,
                "url": "https://creativecommons.org/licenses/by/4.0/",
                "modification": "This dataset is modified by UTS for Academic Purpose and does not allow sharing or replication.",
            }
        ],
        "categories": cat_list_df.to_dict("records"),
        "images": img_list_df.to_dict("records"),
        "annotations": ann_list_df.to_dict("records"),
    }
    return annotation_obj


import json
from os import path


class NpEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, np.integer):
            return int(obj)
        if isinstance(obj, np.floating):
            return float(obj)
        if isinstance(obj, np.ndarray):
            return obj.tolist()
        return super(NpEncoder, self).default(obj)


if __name__ == "__main__":

    dataset_mapping = [
        ("face_shape__batch_0", "testing"),
        ("face_shape__batch_0", "training"),
        ("face_shape__batch_1", "testing"),
        ("face_shape__batch_1", "training"),
        ("face_shape__batch_2", "testing"),
        ("face_shape__batch_2", "training"),
    ]
    for dataset_name, dataset_split in dataset_mapping:
        pcr = process_main(dataset_name, dataset_split)
        dump_str = json.dumps(
            pcr,
            indent=2,
            ensure_ascii=False,
            cls=NpEncoder,
        )

        with open(
            path.join("dataset", dataset_name, dataset_split, "annotations.json"),
            "w",
            encoding="utf-8",
        ) as f:
            f.write(dump_str)

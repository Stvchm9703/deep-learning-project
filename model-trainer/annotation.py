# STEP 1: Import the necessary modules.
import numpy as np
import mediapipe as mp
from mediapipe.tasks import python
from mediapipe.tasks.python import vision
import PIL 

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

    # STEP 4: Detect faces in the input image.
    detection_result = detector.detect(image)
    if len(detection_result.detections) == 0 : 
        return None 

    detection_list = []
    for detection in detection_result.detections:
        bbox = detection.bounding_box
        detection_result.append({
            # "id": int,
            # "image_id": int,
            "category_id": cat_id,
            "segmentation": None,
            "area": 0,
            "bbox": [bbox.origin_x, bbox.origin_y, bbox.width, bbox.height],
            "iscrowd": 0,
        })
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


import glob 
import re
def main():
    detector = init()
    file_list = glob.glob('dataset/face_shape/**/**/*.jpg')
    # regex = re.compile("dataset/face_shape/()/**/*.jpg")
    pattern = r"dataset/face_shape/(?P<dataset_type>[^/]+)/(?P<face_shape_type>[^/]+)/(?P<file_name>[^/]+\.jpg)"


    img_list = []
    cat_list = []
    ann_list = []
    

    for file_path in file_list:
        # Regex pattern
        # Using re.match to apply the regex pattern
        match = re.match(pattern, file_path)
        if match:
            dataset_type = match.group('dataset_type')
            face_shape_type = match.group('face_shape_type')
            file_name = match.group('file_name')

        image_info = create_image_info(file_path)
        if file_name :
            image_info.file_name = face_shape_type + "/" + file_name + ".jpg"

        img_list.append(image_info)


main()

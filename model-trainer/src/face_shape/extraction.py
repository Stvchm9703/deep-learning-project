# %%
#  env
import sys
import os

# SCRIPT_DIR = "/home/ghost/git_src/deep-learninf-project/model-trainer/src"

# sys.path.append(SCRIPT_DIR)

from util import (
    extract_zip_data,
    load_image_dataset,
    # snapshot_data,
    clean_up_zip,
    merge_and_shuffle_data,
    DATASET_PATH,
)


DATASET_BASE = "face_shape"

# %%

set_1 = "/home/ghost/git_src/deep-learninf-project/model-trainer/raw_set/face-shape-dataset.zip"
set_2 = "/home/ghost/git_src/deep-learninf-project/model-trainer/raw_set/faceshape-master.zip"
set_3 = "/home/ghost/git_src/deep-learninf-project/model-trainer/raw_set/man-face-1.zip"

# %%

if __name__ == "__main__":
    set_list = (set_1, set_2, set_3)
    dataset_list = []
    if os.path.exists(os.path.join(DATASET_PATH, DATASET_BASE)) is False:
        os.mkdir(os.path.join(DATASET_PATH, DATASET_BASE))

    for set_path in set_list:
        print("try path " + set_path)
        tmp_path = extract_zip_data(set_path, DATASET_BASE)
        tmp_image_dataset = load_image_dataset(tmp_path, DATASET_BASE)
        dataset_list.append(tmp_image_dataset)

    print(len(dataset_list))
    # %%
    train, test = merge_and_shuffle_data(dataset_list)
    for ind in train.index:
        new_path = os.path.dirname(train["file"][ind])
        new_path = os.path.join(new_path, "training", train["label"][ind])
        if os.path.isdir(new_path) == False:
            os.makedirs(new_path, exist_ok=True)
        if os.path.isfile(train["file"][ind]):
            os.rename(train["file"][ind], os.path.join(new_path, train["name"][ind]))

    for ind in test.index:
        new_path = os.path.dirname(test["file"][ind])
        new_path = os.path.join(new_path, "testing", test["label"][ind])
        if os.path.isdir(new_path) == False:
            os.makedirs(new_path, exist_ok=True)
        if os.path.isfile(test["file"][ind]):
            os.rename(test["file"][ind], os.path.join(new_path, test["name"][ind]))

    train.to_parquet(f"{DATASET_PATH}/face_shape/train.parq.gzip", compression="gzip")
    test.to_parquet(f"{DATASET_PATH}/face_shape/test.parq.gzip", compression="gzip")

    # graceful clean up
    print("graceful clean up tmp file")
    clean_up_zip()


# %%

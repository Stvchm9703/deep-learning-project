# check the python version
prerequirement:
	@python3 --version
	@pip3 --version
	@conda --version
	# if pip is exist , pip install requirements.txt
	@pip3 install -r requirements.txt



FACE_SHAPE_DIR = src/face_shape

# run extraction 
run-extraction:
	# face_shape related data
	cd ${FACE_SHAPE_DIR}; \
	python3 extraction.py; \
	cd ../.. ; \

clean-tmp:
	rm -rf /tmp/dl-model-trainer;


mmd-trainer_batch-0:
	python ./model/mmdetection/tools/train.py ./model/mmdetection/config__face_shape_batch_0.py \
    --work-dir ./model/mmdetection/output/yolox_tiny__face_shape \
    ;


mmd-trainer_batch-1:
	python ./model/mmdetection/tools/train.py ./model/mmdetection/config__face_shape_batch_1.py \
    --work-dir ./model/mmdetection/output/yolox_tiny__face_shape \
    ;

mmd-trainer_batch-2:
	python ./model/mmdetection/tools/train.py ./model/mmdetection/config__face_shape_batch_2.py \
    --work-dir ./model/mmdetection/output/yolox_tiny__face_shape \
    ;

mmd-deploy_batch-0:
	python ./model/mmdeploy/tools/deploy.py \
    ./model/mmdeploy/configs/mmdet/detection/detection_onnxruntime_static.py \
    ./model/mmdetection/config__face_shape_batch_0.py \
    ./model/mmdetection/output/best_coco_bbox_mAP_epoch_120___batch_0_v2.pth \
		./dataset/face_shape/testing/heart/1bf0e579-dd44-337c-a360-e51026f32949.jpg \
    --work-dir ./model/mmdetection/output/yolox_tiny__face_shape \
    --dump-info

mmd-deploy_batch-1:
	python ./model/mmdeploy/tools/deploy.py \
    ./model/mmdeploy/configs/mmdet/detection/detection_onnxruntime_dynamic.py \
    ./model/mmdetection/config__face_shape_batch_1.py \
    ./model/mmdetection/output/best_coco_bbox_mAP_epoch_120___batch_1.pth \
		./dataset/face_shape/testing/heart/1bf0e579-dd44-337c-a360-e51026f32949.jpg \
    --work-dir ./model/mmdeploy_output/b1/yolox_tiny__face_shape  \
    --dump-info


mmd-deploy_batch-2:
	python ./model/mmdeploy/tools/deploy.py \
    ./model/mmdeploy/configs/mmdet/detection/detection_onnxruntime_dynamic.py \
    ./model/mmdetection/config__face_shape_batch_2.py \
    ./model/mmdetection/output/best_coco_bbox_mAP_epoch_120___batch_2.pth \
		./dataset/face_shape/testing/heart/1bf0e579-dd44-337c-a360-e51026f32949.jpg \
    --work-dir ./model/mmdeploy_output/b2/yolox_tiny__face_shape \
    --dump-info


mmd-test-bth-1:
	python  ./model/mmdeploy/tools/test.py\
	 	./model/test-config.py \
		./model/mmdetection/config__face_shape_batch_1.py \
		--model=./model/mmdeploy_output/b1/yolox_tiny__face_shape/end2end.onnx \
		--device cpu \
		--speed-test ;
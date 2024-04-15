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



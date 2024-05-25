import * as ort from "onnxruntime-web";
import type * as Jimp from "jimp";
// import {} from "lodash-es";

/** global **/ window["__WEB_CAMERA_MODEL_SESSION__"] = null;
ort.env.wasm.wasmPaths = "/public/ort-dist/";

export const PREDICTION_CATEGORIES = [
  { name: "square", id: 1 }, //
  { name: "oblong", id: 2 },
  { name: "round", id: 3 }, //
  { name: "heart", id: 4 },
  { name: "rectangular", id: 5 }, //
  { name: "oval", id: 6 }, //
];
export const initVideoProcess = async () => {
  let video = document.getElementById("render_view_video");
  if (!video) {
    console.warn("video element not found");
    return;
  }
  video.setAttribute("playsinline", "");
  video.setAttribute("autoplay", "");
  video.setAttribute("muted", "");
  /* Setting up the constraint */
  var constraints = {
    audio: false,
    video: {
      facingMode: "user",
    },
  };

  navigator.mediaDevices
    .getUserMedia(constraints)
    .then(function success(stream) {
      video.srcObject = stream;
    });
  window["__WEB_CAMERA_MODEL_SESSION__"] = await initModel();
  await testLoadLinkAndPredict();

  /// listen to the upload input change event

  const updloadInputElm = document.getElementById("render_view_input");
  const resultElm = document.getElementById(
    "render_result",
  ) as HTMLInputElement;
  if (updloadInputElm) {
    updloadInputElm.addEventListener("change", async (e) => {
      // console.log("upload input change event");
      // console.log(e);
      const file = e?.target.files[0];
      const image = await loadImageAndConvertToImageData(
        URL.createObjectURL(file),
      );
      const imageData = resizeImageData(image, 416, 416);
      const tensor = preprocessImage(imageData);
      const { labels } = await predict(
        window["__WEB_CAMERA_MODEL_SESSION__"],
        tensor,
      );
      let cat_index = Number(labels.data.at(0));
      console.log(cat_index);
      let output_label = PREDICTION_CATEGORIES[cat_index];
      resultElm.value = output_label.name;
      resultElm?.dispatchEvent(new Event("change"));
    });
  }
};
export const initModel = async () =>
  await ort.InferenceSession.create("./public/end2end_b1_dyn.onnx");

const predict = async (
  session: ort.InferenceSession,
  input_tensor: ort.Tensor,
) => {
  const inputName = session.inputNames[0];
  const outputName = session.outputNames[0];
  // console.log({
  //   inputName: session.inputNames,
  //   outputName: session.outputNames,
  // });
  // let input = new ort.Tensor("float32", input, [1, 3, 256, 256]);
  let output = await session.run({ [inputName]: input_tensor }, [
    ...session.outputNames,
  ]);
  return output;
};

// Capture a frame from the video stream
function captureImage() {
  const video = document.getElementById("render_view_video");
  const canvas = document.getElementById("render_view_canvas");

  const context = canvas.getContext("2d");
  canvas.width = video.videoWidth;
  canvas.height = video.videoHeight;

  context.drawImage(video, 0, 0, canvas.width, canvas.height);
  let image_data = context.getImageData(0, 0, canvas.width, canvas.height);
  return image_data;
}

async function loadImageAndConvertToImageData(filePath: string): Promise<Jimp> {
  const width = 416,
    height = 416;

  var imageData = await Jimp.read(filePath).then((imageBuffer: Jimp) => {
    return imageBuffer.scaleToFit(
      width,
      height,
      Jimp.HORIZONTAL_ALIGN_CENTER | Jimp.VERTICAL_ALIGN_MIDDLE,
    );
  });

  return imageData;
}

// Resize the image data to the target dimensions (416x416) with padding
function resizeImageData(imageData, targetWidth, targetHeight) {
  const canvas = document.getElementById("render_view_canvas");

  const { width, height, data } = imageData;

  // Calculate the scale to fit the image within the target dimensions
  const scale = Math.min(targetWidth / width, targetHeight / height);
  const scaledWidth = Math.round(width * scale);
  const scaledHeight = Math.round(height * scale);

  // Create a new canvas to resize the image
  const tempCanvas = document.createElement("canvas");
  // tempCanvas.width = targetWidth;
  // tempCanvas.height = targetHeight;
  const tempContext = tempCanvas.getContext("2d");

  // Fill the canvas with black (to pad the sides)
  tempContext.fillStyle = "black";
  tempContext.fillRect(0, 0, targetWidth, targetHeight);

  // Draw the resized image on the canvas
  const offsetX = Math.round((targetWidth - scaledWidth) / 2);
  const offsetY = Math.round((targetHeight - scaledHeight) / 2);
  tempContext.drawImage(
    canvas,
    0,
    0,
    width,
    height,
    offsetX,
    offsetY,
    scaledWidth,
    scaledHeight,
  );
  let image_data = tempContext.getImageData(0, 0, targetWidth, targetHeight);
  // Get the resized image data
  // return tempContext.getImageData(0, 0, targetWidth, targetHeight);
  //
  //
  tempCanvas.remove();

  return image_data;
}

// Preprocess the image: normalize and convert to tensor
function preprocessImage(imageData) {
  console.log(imageData);
  const { data, width, height } = imageData;
  const input = new Float32Array(width * height * 3);

  for (let i = 0; i < width * height; i++) {
    input[i * 3] = data[i * 4] / 255; // Red
    input[i * 3 + 1] = data[i * 4 + 1] / 255; // Green
    input[i * 3 + 2] = data[i * 4 + 2] / 255; // Blue
  }

  // Create tensor
  const tensor = new ort.Tensor("float32", input, [1, 3, width, height]);
  return tensor;
}

function imageDataToTensor(image: Jimp, dims: number[]): ort.Tensor {
  // 1. Get buffer data from image and create R, G, and B arrays.
  var imageBufferData = image.bitmap.data;
  const [redArray, greenArray, blueArray] = new Array(
    new Array<number>(),
    new Array<number>(),
    new Array<number>(),
  );

  // 2. Loop through the image buffer and extract the R, G, and B channels
  for (let i = 0; i < imageBufferData.length; i += 4) {
    redArray.push(imageBufferData[i]);
    greenArray.push(imageBufferData[i + 1]);
    blueArray.push(imageBufferData[i + 2]);
    // skip data[i + 3] to filter out the alpha channel
  }

  // 3. Concatenate RGB to transpose [224, 224, 3] -> [3, 224, 224] to a number array
  const transposedData = redArray.concat(greenArray).concat(blueArray);

  // 4. convert to float32
  let i,
    l = transposedData.length; // length, we need this for the loop
  // create the Float32Array size 3 * 224 * 224 for these dimensions output
  const float32Data = new Float32Array(dims[1] * dims[2] * dims[3]);
  for (i = 0; i < l; i++) {
    float32Data[i] = transposedData[i] / 255.0; // convert to float
  }
  // 5. create the tensor object from onnxruntime-web.
  const inputTensor = new ort.Tensor("float32", float32Data, dims);
  return inputTensor;
}

// Event listener for capture button

export async function captureAndPredict() {
  const resultElm = document.getElementById(
    "render_result",
  ) as HTMLInputElement;

  let image_data = captureImage();
  image_data = resizeImageData(image_data, 416, 416);
  let input_tensor = preprocessImage(image_data);
  console.log(window["__WEB_CAMERA_MODEL_SESSION__"]);

  let { labels } = await predict(
    window["__WEB_CAMERA_MODEL_SESSION__"],
    input_tensor,
  );
  let cat_index = Number(labels.data.at(0));
  console.log(cat_index);

  let output_label = PREDICTION_CATEGORIES[cat_index];
  resultElm.value = output_label.name;
  resultElm?.dispatchEvent(new Event("change"));
}

export async function testLoadLinkAndPredict() {
  const file_list = [
    // "./public/sample-male-rectangular.jpg",
    "./public/sample-woman-heart.jpg",
    "./public/sample-woman-oblong.jpg",
    "./public/sample-woman-oval.jpg",
    "./public/sample-woman-round.jpg",
    "./public/sample-woman-square.jpg",
  ];
  file_list.forEach(async (file) => {
    let image_data = await loadImageAndConvertToImageData(file);
    let input_tensor = imageDataToTensor(image_data, [1, 3, 416, 416]);
    // console.log(input_tensor);
    let prediction = await predict(
      window["__WEB_CAMERA_MODEL_SESSION__"],
      input_tensor,
    );
    // console.log(prediction);
    // console.log(typeof prediction);
  });
  return [];
}

// export async function uploadImageAndPredict() {
//   const file = document.getElementById("file_input").files[0];
//   let image_data = await loadImageAndConvertToImageData(file);
//   let input_tensor = imageDataToTensor(image_data, [1, 3, 416, 416]);
//   let prediction = await predict(
//     window["__WEB_CAMERA_MODEL_SESSION__"],
//     input_tensor,
//   );
//   console.log(prediction);
// }
export function echo() {
  console.log("echo");
  return "echo";
}

export function onTriggerUpload() {
  document.getElementById("render_view_input")?.click();
}

#![allow(clippy::new_without_default)]

static ARTIFACT_DIR: &str = "/tmp/burn-mylogs";
static DATA_DIR: &str = "simple_data";
use burn::{backend::ndarray::NdArrayDevice, module::Module, nn::{conv::{Conv2d, Conv2dConfig}, pool::{MaxPool2d, MaxPool2dConfig}, Dropout, DropoutConfig, Initializer, Linear, LinearConfig, PaddingConfig2d, Relu}, tensor::{activation::relu, backend::Backend, Float, Int, Tensor}};

const NUM_LABELS: usize = 10;
const LEARNING_RATE: f64 = 1e-5;

// temporary stopgap
#[derive(Module, Clone, Debug, Default)]
pub struct Tanh {}

impl Tanh {
    /// Create the module.
    pub fn new() -> Self {
        Self {}
    }
    /// Applies the forward pass on the input tensor.
    ///
    /// # Shapes
    ///
    /// - input: `[..., any]`
    /// - output: `[..., any]`
    pub fn forward<B: Backend, const D: usize>(&self, input: Tensor<B, D>) -> Tensor<B, D> {
        burn::tensor::activation::tanh(input)
    }
}

pub fn nll_todo<S>() -> S {
    None.unwrap()
}

// model = Sequential([
//     Conv2D(
    //     filters: 32,
    //     kernel_size: (3,3),
    //     activation='relu',
    //     input_shape = np.shape(train_input[0])
    //     ),
//     MaxPooling2D(pool_size = (2,2)),
//     Conv2D(32, (3,3), activation='relu'),
//     MaxPooling2D(pool_size = (2,2)),
//     Flatten(), is implicit
//     Dense(1024, activation = 'tanh'),
//     Dropout(0.5),  #to reduce overfitting
//     Dense(num_classes, activation='softmax')
// ])

#[derive(Module, Debug)]
pub struct HandwritingModel<B: Backend> {
    conv_2d_1: Conv2d<B>,
    activation_1: Relu,
    max_pooling_2d_1: MaxPool2d,
    conv_2d_2: Conv2d<B>,
    activation_2: Relu,
    max_pooling_2d_2: MaxPool2d,
    // flatten: Flatten<B>, -- implicit
    dense_1: Linear<B>,
    activation_3: Tanh,
    dropout: Dropout,
    dense_2: Linear<B>
}

impl<B: Backend> HandwritingModel<B> {
    pub fn new(device: &B::Device) -> Self {
        // TODO resolve activation, input_shape, channels, and filters
        let conv_2d_1_config = Conv2dConfig {
            kernel_size: [3, 3],
            stride: [1, 1], // keras default
            dilation: [1, 1],
            groups: 1,
            padding: PaddingConfig2d::Valid,
            bias: true,
            initializer: Initializer::XavierUniform {
                gain: 1.0,
            },
            channels: nll_todo(),
        };

        let activation_1 = Relu::new();

        let max_pooling_2d_1_config = MaxPool2dConfig {
            kernel_size: [2, 2], // assuming kernel_size == pool_size
            strides: [2, 2], // defaults to pool_size
            padding: PaddingConfig2d::Valid,
            dilation: [1, 1],
        };

        let max_pooling_2d_1 = max_pooling_2d_1_config.init();

        let conv_2d_1 = conv_2d_1_config.init::<B>(device);

        // repeats of those

        let dropout_config = DropoutConfig {
            prob: 0.5,
        };

        let dropout = dropout_config.init();

        let dense_1_config = LinearConfig {
            d_input: nll_todo(),
            d_output: 1024,
            bias: true,
            initializer: Initializer::XavierUniform {
                gain: 1.0,
            },
        };
        let activation_2 = Relu::new();

        let activation_3 = Tanh::new();

        let dense_1 = dense_1_config.init::<B>(device);

        let dense_2_config = LinearConfig {
            d_input: nll_todo(),
            d_output: NUM_LABELS,
            bias: true,
            initializer: Initializer::XavierUniform {
                gain: 1.0,
            },
        };

        let dense_2 = dense_2_config.init::<B>(device);



        HandwritingModel {
            conv_2d_1,
            activation_1,
            max_pooling_2d_1,
            conv_2d_2: nll_todo(),
            max_pooling_2d_2: nll_todo(),
            activation_2,
            activation_3,
            dense_1,
            dropout,
            dense_2,
        }
    }


    // dims: [batch_size, channels, height, width]
    pub fn forward(&self, input: Tensor<B, 4, Float>) -> Tensor<B, 2> {
        let x = self.conv_2d_1.forward(input);
        let x = self.activation_1.forward(x);
        let x = self.max_pooling_2d_1.forward(x);
        let x = self.conv_2d_2.forward(x);
        let x = self.activation_2.forward(x);
        let x = self.max_pooling_2d_2.forward(x);
        let x = Tensor::flatten(x, 1, 3);
        let x = self.dense_1.forward(x);
        let x = self.activation_3.forward(x);
        let x = self.dropout.forward(x);
        self.dense_2.forward(x)
    }
}


pub type DEVICE = burn::backend::ndarray::NdArray;
fn main() {
    let device = NdArrayDevice::Cpu;

    let fake_random_data = Tensor::<DEVICE, 1, Int>::zeros(
        [5],
        &device,
    );


    println!("Hello, world!");
}

use ringbuf::{HeapRb, Producer, Consumer};

pub struct AudioBuffer {
    producer: Producer<f32, std::sync::Arc<HeapRb<f32>>>,
    consumer: Consumer<f32, std::sync::Arc<HeapRb<f32>>>,
}

impl AudioBuffer {
    pub fn new(capacity: usize) -> Self {
        let rb = HeapRb::<f32>::new(capacity);
        let (prod, cons) = rb.split();
        Self {
            producer: prod,
            consumer: cons,
        }
    }

    pub fn push(&mut self, sample: f32) -> Result<(), f32> {
        self.producer.push(sample)
    }

    pub fn pop(&mut self) -> Option<f32> {
        self.consumer.pop()
    }
}

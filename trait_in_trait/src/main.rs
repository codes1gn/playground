
struct TensorA {}
struct TensorB {}
trait TensorLike {}
impl TensorLike for TensorA {}
impl TensorLike for TensorB {}
impl TensorA {
    pub fn new() -> Self {
        Self {}
    }
}
impl TensorB {
    pub fn new() -> Self {
        Self {}
    }
}


// Executors
struct ExecutorA {}
struct ExecutorB {}
trait ExecutorLike {
    fn compute<TensorType: TensorLike>(&self, inp: TensorType) -> TensorType;
}
// impl ExecutorLike for ExecutorA {}
// impl ExecutorLike for ExecutorB {}

impl ExecutorA {
    pub fn new() -> Self {
        Self {}
    }
    // fn compute(&self, inp: TensorA) -> TensorA { inp }
}
impl ExecutorB {
    pub fn new() -> Self {
        Self {}
    }
}

impl ExecutorLike for ExecutorA {
    fn compute<TensorType: TensorLike>(&self, inp: TensorType) -> TensorType { inp }
}
impl ExecutorLike for ExecutorB {
    fn compute<TensorType: TensorLike>(&self, inp: TensorType) -> TensorType { inp }
}



// Actor
struct Actor<T> 
where
    T: ExecutorLike,
{
    executor: T,
}

// fix: use special syntax to define associated type for traits
// Trait<AsType = ConcType>
impl<T> Actor<T> 
where
    T: ExecutorLike,
{
    pub fn new(exe: T) -> Self {
        Self { executor: exe, }
    }
    pub fn on_compute<U: TensorLike>(&self, inp: U) -> U { self.executor.compute::<U>(inp) }
}


fn main() {
    let exe_a = ExecutorA::new();
    let actor = Actor::<ExecutorA>::new(exe_a);
    let tensor1 = TensorA::new();
    let ret = actor.on_compute::<TensorA>(tensor1);

    let exe_b = ExecutorB::new(); 
    let actor = Actor::<ExecutorB>::new(exe_b);
    let tensor2 = TensorB::new();
    let ret = actor.on_compute::<TensorB>(tensor2);

    println!("Hello, world!");
}

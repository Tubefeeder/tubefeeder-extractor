#[derive(Clone, Debug)]
pub struct Subscription {
    id: String,
}

impl Subscription {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn new(id: &str) -> Self {
        Subscription { id: id.to_owned() }
    }
}

impl tf_core::Subscription for Subscription {
    type Generator = crate::generator::Generator;
    fn generator(&self) -> Self::Generator {
        crate::generator::Generator::new(&self)
    }
}

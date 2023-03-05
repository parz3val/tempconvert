use self::{f::F, c::C, k::K};

pub mod f;
pub mod c;
pub mod k;

impl Clone for F {
    fn clone(&self) -> Self {
        return F(self.0);
    }
}

impl Clone for C {
    fn clone(&self) -> Self {
        return C(self.0);
    }
}

impl Clone for K {
    fn clone(&self) -> Self {
        return K(self.0);
    }
}
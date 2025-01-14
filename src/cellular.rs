use crate::simple_enum;

simple_enum! {
    enum DistanceFn {
        #[default]
        Euclidean,
        EuclideanSquared,
        Manhattan,
        Hybrid,
        MaxAxis,
    }
}

simple_enum! {
    enum CellIndex {
        #[default]
        I0 = 0,
        I1 = 1,
        I2 = 2,
        I3 = 3,
    }
}

simple_enum! {
    enum DistanceReturnType {
        #[default]
        Index0,
        Index0Add1,
        Index0Sub1,
        Index0Mul1,
        Index0Div1,
    }
}

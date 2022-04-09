pub trait Operation<Return> {
    fn execute(&self) -> Return;

    fn rollback(&self, data: &Return) {}
}

pub trait OperationSet {
    fn resolve_conflicts(ops: Vec<Self>) -> ConflictResolution<Self>
    where
        Self: Sized;

    fn get_subject_id(&self) -> String;
}

pub struct ConflictResolution<T> {
    pub resolved: Vec<T>,
    pub rollback: Vec<T>,
}

#[derive(Debug)]
pub enum Type {
    NoAvailableAltPath,
    MasterPathNotRecognized,
    FirstEmptyLineNotRecognized,
    TargetPathNotRecognized,
    ExecutionPathNotRecognized,
    FamilyPriorityNotRecognized,
}

#[derive(Debug)]
pub struct AlternativeResolveError {
    pub error_type: Type,
}

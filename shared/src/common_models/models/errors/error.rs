#[derive(Debug)]
pub enum Type {
    NoAvailableAltPath,
    MasterPathNotRecognized,
    FirstEmptyLineNotRecognized,
    TargetPathNotRecognized,
    ExecutionPathNotRecognized,
    FamilyPriorityNotRecognized,
    MasterPathNotFound,
    SlavePathNotFound,
    DifferentMasterPathWithName,
    EmptyItemListInGroup,
    EmptyGroupListInConfig,
    FilenameNotFound,
}

#[derive(Debug)]
pub struct AlternativeResolveError {
    pub error_type: Type,
}

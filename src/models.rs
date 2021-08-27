use diesel::Queryable;

#[derive(Queryable)]
/// Represents a user who may or may not be the author of any amount
/// of problems
pub struct UserAccount {
    pub id: u32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Queryable)]
/// Represents a problem designed by a user for other users
/// to solve
pub struct Problem {
    pub id: u32,
    /// User ID of the user who created this problem
    pub author_id: u32,
    pub name: String,
    pub description: String,
    /// Path pointing to a zip file on the file-system containing
    /// a set of .in and .out files corresponding to test-cases
    /// for this problem
    pub test_case_file: String,
}

// https://stackoverflow.com/questions/74726116/how-to-skip-serde-serialization-with-skip-serializing-if-for-a-boolean-field
pub(crate) fn is_false(b: &bool) -> bool {
    !(*b)
}

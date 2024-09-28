pub enum Operand {
    Variable(String),
    Constant(i64),
    /// NamedLabel
    /// Goto label;
    /// Ifz value Goto label
    Label(String),
}

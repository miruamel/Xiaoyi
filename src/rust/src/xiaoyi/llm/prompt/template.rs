/// Prompt template helper.
///
/// @brief Simple prompt templating
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::llm::prompt
pub fn render(template: &str, vars: &std::collections::HashMap<&str, &str>) -> String {
    let mut out = template.to_string();
    for (k, v) in vars {
        out = out.replace(&format!("{{{{{k}}}}}"), v);
    }
    out
}

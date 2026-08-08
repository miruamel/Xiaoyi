use crate::xiaoyi::monitoring::finops::cost::CostEstimate;

/// Pricing catalog lookup.
///
/// @brief Model pricing lookup helper
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::monitoring::finops::cost
pub fn lookup_price(model: &str) -> Option<(f64, f64)> {
    match model {
        "gpt-4o" => Some((5.0, 15.0)),
        "gpt-4o-mini" => Some((0.15, 0.6)),
        _ => None,
    }
}

/// Estimate cost from token usage.
///
/// @brief Calculate cost estimate from usage
/// @param model Model identifier
/// @param prompt_tokens Prompt token count
/// @param completion_tokens Completion token count
/// @return Cost estimate or error
/// @since 0.1.0
/// @author Miruamel
/// @see CostEstimate
pub fn estimate_cost(
    model: &str,
    prompt_tokens: u64,
    completion_tokens: u64,
) -> Result<CostEstimate, crate::xiaoyi::core::error::XiaoyiError> {
    let Some((prompt_price, completion_price)) = lookup_price(model) else {
        return Err(crate::xiaoyi::core::error::XiaoyiError::new(
            crate::xiaoyi::core::error::ErrorKind::Llm,
            format!("Unknown model pricing: {model}"),
        ));
    };
    let cost_usd = (prompt_tokens as f64) * prompt_price / 1_000_000.0
        + (completion_tokens as f64) * completion_price / 1_000_000.0;
    Ok(CostEstimate {
        prompt_tokens,
        completion_tokens,
        model: model.into(),
        cost_usd,
    })
}

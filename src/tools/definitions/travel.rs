use crate::tools::models::{ParameterType, ToolCategory, ToolDefinition, ToolParameter};

/// Search flights using Amadeus API
pub fn search_flights() -> ToolDefinition {
    ToolDefinition::new(
        "search_flights",
        "Search for flights using Amadeus API",
        ToolCategory::Travel,
    )
    .with_parameter(ToolParameter::required(
        "origin",
        ParameterType::String,
        "Origin airport code (e.g., JFK)",
    ))
    .with_parameter(ToolParameter::required(
        "destination",
        ParameterType::String,
        "Destination airport code (e.g., LAX)",
    ))
    .with_parameter(ToolParameter::required(
        "date",
        ParameterType::String,
        "Departure date (YYYY-MM-DD)",
    ))
    .with_parameter(ToolParameter::optional(
        "adults",
        ParameterType::Integer,
        "Number of adult passengers",
        Some(serde_json::json!(1)),
    ))
    .with_parameter(ToolParameter::optional(
        "max_results",
        ParameterType::Integer,
        "Maximum number of results to return",
        Some(serde_json::json!(10)),
    ))
    .with_credential("amadeus/api_key")
    .with_credential("amadeus/api_secret")
    .with_python_path("activities.travel.amadeus.search_flights")
    .with_duration(60)
    .with_cost(0.01)
}

/// Book a flight using Amadeus API
pub fn book_flight() -> ToolDefinition {
    ToolDefinition::new(
        "book_flight",
        "Book a flight using Amadeus API",
        ToolCategory::Travel,
    )
    .with_parameter(ToolParameter::required(
        "flight_offer",
        ParameterType::Object,
        "Flight offer object from search results",
    ))
    .with_parameter(ToolParameter::required(
        "passenger_name",
        ParameterType::String,
        "Passenger name (First Last)",
    ))
    .with_parameter(ToolParameter::optional(
        "contact_email",
        ParameterType::String,
        "Contact email for booking confirmation",
        None,
    ))
    .with_credential("amadeus/api_key")
    .with_credential("amadeus/api_secret")
    .with_python_path("activities.travel.amadeus.book_flight")
    .with_duration(120)
    .with_cost(0.5)
}

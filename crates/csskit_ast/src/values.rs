use csskit_proc_macro::{initial, value};

// global values
#[value(" css || scss || cks ")]
#[initial("css")]
pub struct LangStyleValue;

// @lint values
#[value(" correctness | suspicious | complexity | perf | style | pedantic | restriction | nursery | deprecated ")]
#[initial("nursery")]
pub enum CategoryStyleValue {}

// NodeRule values
#[value(" none | <string># ")]
#[initial("none")]
pub enum MessageStyleValue<'a> {}

#[value(" none | <string># ")]
#[initial("none")]
pub enum HelpStyleValue<'a> {}

// @fix values
#[value(" none | replace | delete ")]
#[initial("none")]
pub enum ActionStyleValue {}

// @test / @pass values
#[value(" none | <string># ")]
#[initial("none")]
pub enum ContentStyleValue<'a> {}

#[value(" none | <string># ")]
#[initial("none")]
pub enum SpanStyleValue<'a> {}

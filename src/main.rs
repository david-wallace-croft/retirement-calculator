//! version: 2022-07-30
//! since: 2022-07-16

use std::{iter::Rev, str::Chars};

use croftsoft::core::math::finance_lib::annual_savings_needed;
use web_sys::HtmlInputElement;
use yew::prelude::*;

static INVESTMENT_INTEREST: &str = "10.0";
static INVESTMENT_YEARS: &str = "50.0";
static RETIREMENT_INCOME: &str = "100000.0";
static RETIREMENT_INFLATION: &str = "1.0";
static RETIREMENT_INTEREST: &str = "10.0";
static RETIREMENT_TAX_RATE: &str = "10.0";

#[derive(Debug)]
enum Msg {
  OnChange,
}

#[derive(Debug)]
struct Model {
  annual_savings: f64,
  investment_interest: NodeRef,
  investment_years: NodeRef,
  retirement_income: NodeRef,
  retirement_inflation: NodeRef,
  retirement_interest: NodeRef,
  retirement_tax_rate: NodeRef,
}

impl Model {
  fn calculate(&mut self) {
    let investment_interest =
      parse_or_reset(&self.investment_interest, INVESTMENT_INTEREST);
    let investment_years =
      parse_or_reset(&self.investment_years, INVESTMENT_YEARS);
    let retirement_income =
      parse_or_reset(&self.retirement_income, RETIREMENT_INCOME);
    let retirement_inflation =
      parse_or_reset(&self.retirement_inflation, RETIREMENT_INFLATION);
    let retirement_interest =
      parse_or_reset(&self.retirement_interest, RETIREMENT_INTEREST);
    let retirement_tax_rate =
      parse_or_reset(&self.retirement_tax_rate, RETIREMENT_TAX_RATE);
    self.annual_savings = calculate_required_annual_investment(
      retirement_income,
      investment_years,
      investment_interest / 100.0,
      retirement_interest / 100.0,
      retirement_tax_rate / 100.0,
      retirement_inflation / 100.0,
    );
    log::debug!("annual_savings: {:?}", self.annual_savings);
  }
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_context: &Context<Self>) -> Self {
    log::trace!("create()");
    let annual_savings = calculate_required_annual_investment(
      RETIREMENT_INCOME.parse::<f64>().unwrap(),
      INVESTMENT_YEARS.parse::<f64>().unwrap(),
      INVESTMENT_INTEREST.parse::<f64>().unwrap() / 100.0,
      RETIREMENT_INTEREST.parse::<f64>().unwrap() / 100.0,
      RETIREMENT_TAX_RATE.parse::<f64>().unwrap() / 100.0,
      RETIREMENT_INFLATION.parse::<f64>().unwrap() / 100.0,
    );
    Self {
      annual_savings,
      investment_interest: NodeRef::default(),
      investment_years: NodeRef::default(),
      retirement_income: NodeRef::default(),
      retirement_inflation: NodeRef::default(),
      retirement_interest: NodeRef::default(),
      retirement_tax_rate: NodeRef::default(),
    }
  }

  fn rendered(
    &mut self,
    _ctx: &Context<Self>,
    first_render: bool,
  ) {
    log::trace!("rendered()");
    if !first_render {
      return;
    }
    self
      .investment_interest
      .cast::<HtmlInputElement>()
      .unwrap()
      .set_value(INVESTMENT_INTEREST);
    self
      .investment_years
      .cast::<HtmlInputElement>()
      .unwrap()
      .set_value(INVESTMENT_YEARS);
    let retirement_income =
      self.retirement_income.cast::<HtmlInputElement>().unwrap();
    retirement_income.set_value(RETIREMENT_INCOME);
    let _result = retirement_income.focus();
    self
      .retirement_inflation
      .cast::<HtmlInputElement>()
      .unwrap()
      .set_value(RETIREMENT_INFLATION);
    self
      .retirement_interest
      .cast::<HtmlInputElement>()
      .unwrap()
      .set_value(RETIREMENT_INTEREST);
    self
      .retirement_tax_rate
      .cast::<HtmlInputElement>()
      .unwrap()
      .set_value(RETIREMENT_TAX_RATE);
  }

  fn update(
    &mut self,
    _context: &Context<Self>,
    msg: Self::Message,
  ) -> bool {
    log::trace!("update()");
    match msg {
      Msg::OnChange => {
        self.calculate();
        true
      },
    }
  }

  fn view(
    &self,
    context: &Context<Self>,
  ) -> Html {
    log::trace!("view()");
    let link = context.link();
    let onchange_callback = link.callback(|_: Event| Msg::OnChange);
    html! {
    <center>
    <h1>
    { "Retirement Calculator" }
    </h1>
    <p>
    <a target="_blank" href="http://www.croftsoft.com/people/david/">
    { "David Wallace Croft" }</a>
    </p>
    <p>
    { "Version 2022-07-30" }
    </p>
    // <form>
    <table>
    <tr>
    <td>
    { "Desired annual retirement income (present value, after taxes)" }
    </td>
    <td>
    <input
      onchange={&onchange_callback}
      ref={self.retirement_income.clone()}
      size="10"
      type="text"/>
    { " dollars ($)" }
    </td>
    </tr>
    <tr>
    <td>
    { "Years until retirement (usually at 67 years of age)" }
    </td>
    <td>
    <input
      onchange={&onchange_callback}
      ref={self.investment_years.clone()}
      size="10"
      type="text"/>
    { " years" }
    </td>
    </tr>
    <tr>
    <td>
    { "Annual investment growth rate before retirement (tax-deferred)" }
    </td>
    <td>
    <input
      onchange={&onchange_callback}
      ref={self.investment_interest.clone()}
      size="10"
      type="text"/>
    { " percent (%)" }
    </td>
    </tr>
    <tr>
    <td>
    { "Annual interest earned on retirement savings during retirement" }
    </td>
    <td>
    <input
      onchange={&onchange_callback}
      ref={self.retirement_interest.clone()}
      size="10"
      type="text"/>
    { " percent (%)" }
    </td>
    </tr>
    <tr>
    <td>
    { "Tax rate during retirement on savings interest" }
    </td>
    <td>
    <input
      onchange={&onchange_callback}
      ref={self.retirement_tax_rate.clone()}
      size="10"
      type="text"/>
    { " percent (%)" }
    </td>
    </tr>
    <tr>
    <td>
    { "Estimated annual inflation" }
    </td>
    <td>
    <input
      onchange={&onchange_callback}
      ref={self.retirement_inflation.clone()}
      size="10"
      type="text"/>
    { " percent (%)" }
    </td>
    </tr>
    </table>
    // </form>
    if self.annual_savings < 0.0 {
    <p>
    <font color="red">
    { "The interest rate on retirement savings must exceed the annual inflation rate." }
    </font>
    </p>
    } else {
    <p>
    <font color="green">
    { "You would need to invest " }
    { to_dollars(self.annual_savings) }
    { " each year." }
    </font>
    </p>
    }
    <p>
    { "This calculator does not factor in social security income." }
    <br/>
    { "Click " }
    <a
      target="_blank"
      href="https://www.bankrate.com/retirement/retirement-plan-calculator/">
    { "here" }
    </a>
    { " for a calculator that includes social security income." }
    </p>
    </center>
    }
  }
}

fn calculate_required_annual_investment(
  desired_savings_interest_income: f64,
  years_of_saving: f64,
  investment_interest_rate: f64,
  savings_interest_rate: f64,
  tax_rate: f64,
  inflation_rate: f64,
) -> f64 {
  let savings: f64 = desired_savings_interest_income * (1.0 + inflation_rate)
    / (savings_interest_rate * (1.0 - tax_rate) - inflation_rate);
  if years_of_saving == 0.0 {
    return savings;
  }
  let future_value_savings =
    savings * (1.0 + inflation_rate).powf(years_of_saving);
  annual_savings_needed(
    future_value_savings,
    investment_interest_rate,
    years_of_saving,
  )
}

fn main() {
  wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
  yew::start_app::<Model>();
}

fn parse_or_reset(
  node_ref: &NodeRef,
  default_value: &str,
) -> f64 {
  let element = node_ref.cast::<HtmlInputElement>().unwrap();
  if let Ok(parsed_value) = element.value().parse::<f64>() {
    return parsed_value;
  }
  element.set_value(default_value);
  default_value.parse::<f64>().unwrap()
}

fn to_comma_separated(value: u64) -> String {
  let value_as_string: String = value.to_string();
  let reversed_without_commas: Rev<Chars> = value_as_string.chars().rev();
  let mut reversed_with_commas: String = "".to_string();
  for (i, c) in reversed_without_commas.enumerate() {
    if (i > 0) && (i % 3 == 0) {
      reversed_with_commas.push(',');
    }
    reversed_with_commas.push(c);
  }
  let comma_separated: String = to_reverse_string(reversed_with_commas);
  comma_separated
}

fn to_dollars(amount: f64) -> String {
  let rounded_amount: f64 = amount.round();
  let integer_amount: i64 = rounded_amount as i64;
  let positive_amount: u64 = integer_amount.unsigned_abs();
  let comma_separated_string: String = to_comma_separated(positive_amount);
  let mut dollars: String = "".to_owned();
  if integer_amount.is_negative() {
    dollars.push('-');
  }
  dollars.push('$');
  dollars += &comma_separated_string;
  dollars
}

fn to_reverse_string(s: String) -> String {
  s.chars().rev().collect::<String>()
}

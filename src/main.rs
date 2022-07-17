/// since: 2022-07-16
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
    let onchange_investment_interest = link.callback(|_: Event| Msg::OnChange);
    let onchange_investment_years = link.callback(|_: Event| Msg::OnChange);
    let onchange_retirement_income = link.callback(|_: Event| Msg::OnChange);
    let onchange_retirement_interest = link.callback(|_: Event| Msg::OnChange);
    let onchange_retirement_inflation = link.callback(|_: Event| Msg::OnChange);
    let onchange_retirement_tax_rate = link.callback(|_: Event| Msg::OnChange);
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
    { "Version 2022-07-16" }
    </p>
    // <form>
    <table>
    <tr>
    <td>
    { "Desired annual retirement income (present value, after taxes)" }
    </td>
    <td>
    <input
      onchange={onchange_retirement_income}
      ref={self.retirement_income.clone()}
      size="10"
      type="text"/>
    { " dollars" }
    </td>
    </tr>
    <tr>
    <td>
    { "Years until retirement (usually at 67 years of age)" }
    </td>
    <td>
    <input
      onchange={onchange_investment_years}
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
      onchange={onchange_investment_interest}
      ref={self.investment_interest.clone()}
      size="10"
      type="text"/>
    { " %" }
    </td>
    </tr>
    <tr>
    <td>
    { "Annual interest earned on retirement savings during retirement" }
    </td>
    <td>
    <input
      onchange={onchange_retirement_interest}
      ref={self.retirement_interest.clone()}
      size="10"
      type="text"/>
    { " %" }
    </td>
    </tr>
    <tr>
    <td>
    { "Tax rate during retirement on savings interest" }
    </td>
    <td>
    <input
      onchange={onchange_retirement_tax_rate}
      ref={self.retirement_tax_rate.clone()}
      size="10"
      type="text"/>
    { " %" }
    </td>
    </tr>
    <tr>
    <td>
    { "Estimated annual inflation" }
    </td>
    <td>
    <input
      onchange={onchange_retirement_inflation}
      ref={self.retirement_inflation.clone()}
      size="10"
      type="text"/>
    { " %" }
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
    { self.annual_savings }
    { " each year." }
    </font>
    </p>
    }
    </center>
    }
  }
}

//  /*********************************************************************
//  * Calculates the annual savings necessary to accumulate a specified
//  * value in the future.
//  *
//  * @param f Future value desired.
//  * @param r Annual interest.
//  * @param t Number of years of savings.
//  *********************************************************************/
fn annual_savings_needed(
  f: f64,
  r: f64,
  t: f64,
) -> f64 {
  f * r / ((1.0 + r).powf(t) - 1.0)
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

fn main() {
  wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
  yew::start_app::<Model>();
}

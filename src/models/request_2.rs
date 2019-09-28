/* 
 * Brainrex API Explorer
 *
 * Welcome to the Brainrex API explorer, we make analytics tools for crypto and blockchain. Our currently propiertary models offer sentiment analysis, market making, blockchain monitoring and face-id verification. This AI models can be consumed from this API. We also offer integrations to open data and propietary data providers, as well as free test data we collect. There is a collection of data transformation tools. Join our Telegram group to get the latest news and ask questions [https://t.me/brainrex, #brainrex](https://t.me/brainrex). More about Brainrex at [https://brainrex.com](http://brainrex.com). Full Documentation can be found at [https://brainrexapi.github.io/docs](https://brainrexapi.github.io/docs)
 *
 * OpenAPI spec version: 0.1.1
 * Contact: support@brainrex.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request2 {
  /// Name of the exchange
  #[serde(rename = "exchange")]
  exchange: Option<String>,
  /// Name of the currency pair
  #[serde(rename = "market")]
  market: Option<String>,
  /// Name of the data format availables (standard)
  #[serde(rename = "data_format")]
  data_format: Option<String>,
  /// Name of the market
  #[serde(rename = "orient")]
  orient: Option<String>,
  /// Name of the market
  #[serde(rename = "candle_size")]
  candle_size: Option<String>,
  /// Start date in YYYY/MM/DD
  #[serde(rename = "start_date")]
  start_date: Option<String>,
  /// End date YYYY/MM/DD
  #[serde(rename = "end_date")]
  end_date: Option<String>
}

impl Request2 {
  pub fn new() -> Request2 {
    Request2 {
      exchange: None,
      market: None,
      data_format: None,
      orient: None,
      candle_size: None,
      start_date: None,
      end_date: None
    }
  }

  pub fn set_exchange(&mut self, exchange: String) {
    self.exchange = Some(exchange);
  }

  pub fn with_exchange(mut self, exchange: String) -> Request2 {
    self.exchange = Some(exchange);
    self
  }

  pub fn exchange(&self) -> Option<&String> {
    self.exchange.as_ref()
  }

  pub fn reset_exchange(&mut self) {
    self.exchange = None;
  }

  pub fn set_market(&mut self, market: String) {
    self.market = Some(market);
  }

  pub fn with_market(mut self, market: String) -> Request2 {
    self.market = Some(market);
    self
  }

  pub fn market(&self) -> Option<&String> {
    self.market.as_ref()
  }

  pub fn reset_market(&mut self) {
    self.market = None;
  }

  pub fn set_data_format(&mut self, data_format: String) {
    self.data_format = Some(data_format);
  }

  pub fn with_data_format(mut self, data_format: String) -> Request2 {
    self.data_format = Some(data_format);
    self
  }

  pub fn data_format(&self) -> Option<&String> {
    self.data_format.as_ref()
  }

  pub fn reset_data_format(&mut self) {
    self.data_format = None;
  }

  pub fn set_orient(&mut self, orient: String) {
    self.orient = Some(orient);
  }

  pub fn with_orient(mut self, orient: String) -> Request2 {
    self.orient = Some(orient);
    self
  }

  pub fn orient(&self) -> Option<&String> {
    self.orient.as_ref()
  }

  pub fn reset_orient(&mut self) {
    self.orient = None;
  }

  pub fn set_candle_size(&mut self, candle_size: String) {
    self.candle_size = Some(candle_size);
  }

  pub fn with_candle_size(mut self, candle_size: String) -> Request2 {
    self.candle_size = Some(candle_size);
    self
  }

  pub fn candle_size(&self) -> Option<&String> {
    self.candle_size.as_ref()
  }

  pub fn reset_candle_size(&mut self) {
    self.candle_size = None;
  }

  pub fn set_start_date(&mut self, start_date: String) {
    self.start_date = Some(start_date);
  }

  pub fn with_start_date(mut self, start_date: String) -> Request2 {
    self.start_date = Some(start_date);
    self
  }

  pub fn start_date(&self) -> Option<&String> {
    self.start_date.as_ref()
  }

  pub fn reset_start_date(&mut self) {
    self.start_date = None;
  }

  pub fn set_end_date(&mut self, end_date: String) {
    self.end_date = Some(end_date);
  }

  pub fn with_end_date(mut self, end_date: String) -> Request2 {
    self.end_date = Some(end_date);
    self
  }

  pub fn end_date(&self) -> Option<&String> {
    self.end_date.as_ref()
  }

  pub fn reset_end_date(&mut self) {
    self.end_date = None;
  }

}




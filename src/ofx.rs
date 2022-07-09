use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct OFX {
    #[serde(rename = "BANKMSGSRSV1")]
    pub message: BankMessageResponse,
}

#[derive(Debug, Deserialize)]
pub struct BankMessageResponse {
    #[serde(rename = "STMTTRNRS")]
    pub response: StatementResponse,
}

#[derive(Debug, Deserialize)]
pub struct StatementResponse {
    #[serde(rename = "STMTRS")]
    pub aggregate: StatementResponseAggregate,
}

#[derive(Debug, Deserialize)]
pub struct StatementResponseAggregate {
    #[serde(rename = "BANKTRANLIST")]
    pub transaction_list: BankTransactionList,
    #[serde(rename = "BANKACCTFROM")]
    pub account: Account,

    #[serde(rename = "AVAILBAL")]
    pub available_balance: AvailableBalance,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    #[serde(rename = "BANKID")]
    pub id: String,
    #[serde(rename = "ACCTID")]
    pub account_number: String,
    #[serde(rename = "ACCTTYPE")]
    pub account_type: String,
}
#[derive(Debug, Deserialize)]
pub struct BankTransactionList {
    #[serde(rename = "STMTTRN")]
    pub transactions: Vec<StatementTransaction>,
}

#[derive(Debug, Deserialize)]
pub struct StatementTransaction {
    #[serde(rename = "FITID")]
    pub id: String,
    #[serde(rename = "DTPOSTED")]
    pub date_posted: String,
    #[serde(rename = "TRNTYPE")]
    pub transaction_type: String,
    #[serde(rename = "TRNAMT")]
    pub amount: String,
    #[serde(rename = "NAME")]
    pub description: String,
    #[serde(rename = "DTUSER")]
    pub user: String,
}

#[derive(Debug, Deserialize)]
pub struct AvailableBalance {
    #[serde(rename = "DTASOF")]
    pub date: String,
    #[serde(rename = "BALAMT")]
    pub amount: String,
}

pub fn load(source: &str) -> Result<OFX, &'static str> {
    let f = File::open(source).expect(&format!("Cannot open file {}", source));
    let r = BufReader::new(f);
    let ofx: OFX = serde_xml_rs::de::from_reader(r).unwrap();
    return Ok(ofx);
}

use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
#[derive(Debug, Deserialize)]
pub struct OFX {
    #[serde(rename = "BANKMSGSRSV1")]
    message: BankMessageResponse,
}

#[derive(Debug, Deserialize)]
pub struct BankMessageResponse {
    #[serde(rename = "STMTTRNRS")]
    response: StatementResponse,
}

#[derive(Debug, Deserialize)]
pub struct StatementResponse {
    #[serde(rename = "STMTRS")]
    aggregate: StatementResponseAggregate,
}

#[derive(Debug, Deserialize)]
pub struct StatementResponseAggregate {
    #[serde(rename = "BANKTRANLIST")]
    transaction_list: BankTransactionList,
    #[serde(rename = "BANKACCTFROM")]
    account: Account,

    #[serde(rename = "AVAILBAL")]
    available_balance: AvailableBalance,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    #[serde(rename = "BANKID")]
    id: String,
    #[serde(rename = "ACCTID")]
    account_number: String,
    #[serde(rename = "ACCTTYPE")]
    account_type: String,
}
#[derive(Debug, Deserialize)]
pub struct BankTransactionList {
    #[serde(rename = "STMTTRN")]
    transactions: Vec<StatementTransaction>,
}

#[derive(Debug, Deserialize)]
pub struct StatementTransaction {
    #[serde(rename = "FITID")]
    id: String,
    #[serde(rename = "DTPOSTED")]
    date_posted: String,
    #[serde(rename = "TRNTYPE")]
    transaction_type: String,
    #[serde(rename = "TRNAMT")]
    amount: String,
    #[serde(rename = "NAME")]
    description: String,
    #[serde(rename = "DTUSER")]
    user: String,
}

#[derive(Debug, Deserialize)]
pub struct AvailableBalance {
    #[serde(rename = "DTASOF")]
    date: String,
    #[serde(rename = "BALAMT")]
    amount: String,
}

pub fn Load(source: &str) -> Result<OFX, &'static str> {
    let f = File::open(source).expect(&format!("Cannot open file {}", source));
    let r = BufReader::new(f);
    let ofx: OFX = serde_xml_rs::de::from_reader(r).unwrap();
    println!("{:?}", ofx.message.response.aggregate.account);
    for tx in ofx
        .message
        .response
        .aggregate
        .transaction_list
        .transactions
        .iter()
    {
        println!("{:?}", tx);
    }
    return Ok(ofx);
}

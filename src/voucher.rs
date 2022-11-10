
use futures::TryStreamExt;
use mongodb::bson::{Bson, doc};
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

use types::common::VoucherType;
use crate::error::{Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VoucherConfig {
#[serde(rename_all = "camelCase")]
Payment {
    cash_register_enabled: bool,
    print_after_save: bool,
    open_cheque_book_detail: bool,
    expense_only: bool
},
#[serde(rename_all = "camelCase")]
Receipt {
    cash_register_enabled: bool,
    print_after_save: bool,
    income_only: bool
},
#[serde(rename_all = "camelCase")]
Journal {
    cash_register_enabled: bool,
    print_after_save:  bool,
    allow_gst:  bool,
    allow_customer:  bool,
    allow_vendor: bool
},
#[serde(rename_all = "camelCase")]
Contra {
    cash_register_enabled: bool,
    print_after_save: bool,
    cash_account_only: bool,
    bank_account_only: bool
},
#[serde(rename_all = "camelCase")]
Sale {
    cash_register_enabled: bool,
    warehouse_enabled: bool,
    hide_rack: bool,
    hide_mrp_in_batch_model: bool,
    rate_editable: bool,
    tax_editable: bool,
    discount_editable: bool,
    unit_editable: bool,
    bill_discount_editable: bool,
    set_focus_on_inventory: bool,
    auto_select_batch: bool,
    set_default_qty: bool,
    print_after_save: bool,
    default_print_template: bool,
    enable_silent_print_mode: bool,
    allow_credit_customer: bool
},
#[serde(rename_all = "camelCase")]
Purchase {
    cash_register_enabled: bool,
    warehouse_enabled: bool,
    print_after_save: bool,
    s_rate_mrp_required: bool,
    s_rate_as_mrp: bool,
    default_print_template: bool,
    enable_silent_print_mode: bool,
    allow_credit_vendor: bool
},
#[serde(rename_all = "camelCase")]
SaleReturn {
    cash_register_enabled: bool,
    warehouse_enabled: bool,
    rate_editable: bool,
    tax_editable: bool,
    discount_editable: bool,
    unit_editable: bool,
    bill_discount_editable: bool,
    print_after_save: bool,
    default_print_template: bool,
    enable_silent_print_mode: bool,
    allow_credit_customer: bool
},
#[serde(rename_all = "camelCase")]
PurchaseReturn {
    cash_register_enabled: bool,
    warehouse_enabled: bool,
    print_after_save: bool,
    rate_editable: bool,
    tax_editable: bool,
    discount_editable: bool,
    default_print_template: bool,
    enable_silent_print_mode: bool,
    allow_credit_vendor: bool
},
#[serde(rename_all = "camelCase")]
StockJournal {
    print_after_save: bool,
    default_print_template: bool,
    enable_silent_print_mode: bool,
}
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoucherName {
    #[serde(rename = "_id")]
    id: ObjectId,
    voucher_type: VoucherType,
    voucher_name: String,
    prefix: String,
    config: VoucherConfig,
    default: bool
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoucherNameInput {
    pub voucher_type: VoucherType,
    pub voucher_name: String,
    pub prefix: String,
    pub config: VoucherConfig,
    pub default: bool
}
impl VoucherName {
    pub fn collection(db: &Database) -> Collection<Self> {
        db.collection("voucher_names")
    }

    pub async fn create_voucher_name(
        db: &Database,
        data: VoucherNameInput,
    ) -> Result<VoucherName> {

        let voucher_name = VoucherName {
            id: ObjectId::new(),
            voucher_type: data.voucher_type,
            voucher_name: data.voucher_name,
            prefix: data.prefix,
            config: data.config,
            default: data.default
        };
        VoucherName::collection(db).insert_one(&voucher_name, None).await.unwrap();
        Ok(voucher_name)
    }

    pub async fn list_voucher_name(db: &Database) -> Result<Vec<VoucherName>> {
        let out = VoucherName::collection(db).find(doc!{}, None).await.unwrap().try_collect::<Vec<VoucherName>>().await.unwrap();
        Ok(out)
    }
    pub async fn get_voucher_name(db: &Database, id: ObjectId) -> Result<VoucherName> {
        println!("1");
        let out = VoucherName::collection(db).find_one(doc!{"_id": &id}, None).await.unwrap();
        Ok(out.unwrap())
    }

}
//استيراد ما نحتاجه من بيئة العمل
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{env, near_bindgen, PanicOnDefault};
near_sdk::setup_alloc!();

//لاستيراد
//Writing ستركتشر
//من ملف
//models.rs
mod models;
use models::Writing;

//the main contract struct definition
//تعريف العقد الرئيسي للبرنامج الخاص بنا
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  //تخزين قائمة الرسائل
  writing_list: Vector<Writing>,
}

//the main contract struct implementation
//كود الدوال (الفنكشنز) الخاصة بالعقد الرئيسي
#[near_bindgen]
impl Contract {
  //فنكشن لكتابة رسالة لصديق
  pub fn write_something(&mut self, message: String, receiver: String) -> Writing {
    //الحصول على المستخدم صاحب هذه العملية - الذي قام باستدعاء الفنكشن
    let sender: String = env::signer_account_id();

    let writing = Writing {
      message,
      sender,
      receiver,
    };
    self.writing_list.push(&writing);
    return writing;
  }
  //فنكشن لاسترجاع الرسائل الخاصة بالمستخدم الذى استدعى هذه الفنكشن
  pub fn list_writings(&self) -> Vec<Writing> {
    let mut data = Vec::new();

    for w in self.writing_list.iter() {
      //إضافة الرسائل المرسلة و المستقبلة من المستخدم الذى قام باستدعاء هذه الدالة
      if w.receiver == env::signer_account_id() || w.sender == env::signer_account_id() {
        data.push(w);
      }
    }
    return data;
  }

  //هذه الدالة هامة جدا لتجهيز الأوبجيكت الخاص بقائمة الرسائل و يجب استدعائها أولاً مرة واحدة
  //قبل البدء فى إضافة أو عرض الرسائل
  #[init]
  pub fn init() -> Self {
    let this = Self {
      //تجهيز فيكتور قائمة الرسائل
      writing_list: Vector::new("messages".try_to_vec().unwrap()),
    };

    this
  }
}

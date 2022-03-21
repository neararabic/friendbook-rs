# مشروع كتاب الأصدقاء Friendbook in Rust
مشروع بسيط  يمكن المستخدمين من ارسال رسائل لأصدقائهم على حساباتهم فى نيــر باستخدام لغة البرمجة Rust

## لتشغيل المشروع

يحتاج المشروع لتثبيت أدوات التطوير الخاصة بلغة رست و ويب أسمبلي rust and web assembly tool chain
يمكن اتباع خطوات تثبيتهم هنــاhttps://www.near-sdk.io/

1- قم بنسخ المشروع لجهازك clone

2- من داخل مجلد المشروع فى شاشة الأوامر terminal قم بكتابة الأمر التالي

```
env 'RUSTFLAGS=-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
```

- ملحوظة: الجزء `env 'RUSTFLAGS=-C link-arg=-s'` يقوم بعمل تحسين لعملية تحويل الكود و يؤثر جذرياً على حجم الملف الناتج و بالتالى تكلفة رفع الملف و استضافته على البلوك تشين. إذا كان الملف الناتج من العملية السابقة أكبر من 200 كيلو بايت لديك (14 ميجا بايت مثلاً أو أكثر) فهذا يعنى أن الأمر لم يعمل بشكل صحيح و يجب عليك فحص إعدادات رست و تفعيله.

3- ستلاحظ وجود ملفات جديدة تم انشائها داخل مجلد target و منها الملف `friendbookrs.wasm`

4- يمكنك رفع هذا الملف مباشرة على البلوك تشين بالطرق المعتادة مثل

```
 near dev-deploy .\target\wasm32-unknown-unknown\release\friendbookrs.wasm
```

5- يجب استدعاء الدالة init  أولاً
6- يمكنك استعدعاء الدوال الأخري بعد ذلك 
```
 near call $CONTRACT list_writings --accountId $ACCOUNT
 near call $CONTRACT write_something '{"message":"sample message","receiver":"myaccount.testnet"}' --accountId $ACCOUNT
 
```

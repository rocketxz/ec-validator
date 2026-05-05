use wasm_bindgen::prelude::*;
use crate::{cedula, ruc, iban};

#[wasm_bindgen]
pub fn validate_cedula(input: &str) -> bool {
    cedula::validate(input).is_ok()
}

#[wasm_bindgen]
pub fn validate_ruc(input: &str) -> JsValue {
    let is_valid = ruc::validate(input).is_ok();
    let ruc_type_opt = if is_valid { ruc::ruc_type(input) } else { None };
    
    let type_str = if let Some(t) = ruc_type_opt {
        match t {
            crate::ruc::RucType::NaturalPerson => "natural_person",
            crate::ruc::RucType::JuridicalEntity => "juridical_entity", 
            crate::ruc::RucType::PublicEntity => "public_entity",
        }
    } else { "" };
    
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"valid".into(), &is_valid.into()).unwrap();
    js_sys::Reflect::set(&obj, &"type".into(), &type_str.into()).unwrap();
    js_sys::Reflect::set(&obj, &"error".into(), &JsValue::NULL).unwrap();
    obj.into()
}

#[wasm_bindgen]
pub fn validate_iban(input: &str) -> bool {
    iban::validate(input).is_ok()
}

#[wasm_bindgen]
pub fn format_iban(input: &str) -> JsValue {
    match iban::format(input) {
        Some(formatted) => JsValue::from_str(&formatted),
        None => JsValue::NULL,
    }
}
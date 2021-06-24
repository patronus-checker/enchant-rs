extern crate enchant_sys;

use enchant_sys::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::rc::Rc;

pub struct Dict {
    dict: *mut EnchantDict,
    broker_holder: Rc<BrokerHolder>,
    data: DictData,
}

#[derive(Debug, Default)]
pub struct DictData {
    pub lang: String,
    pub provider: ProviderData,
}

impl Drop for Dict {
    fn drop(&mut self) {
        unsafe {
            enchant_broker_free_dict(self.broker_holder.broker, self.dict);
        }
    }
}

impl Dict {
    fn new(dict: *mut EnchantDict, broker_holder: Rc<BrokerHolder>) -> Self {
        extern "C" fn describe_fn(
            lang: *const c_char,
            provider_name: *const c_char,
            provider_desc: *const c_char,
            provider_file: *const c_char,
            user_data: *mut c_void,
        ) {
            unsafe {
                let mut dict = user_data as *mut DictData;
                (*dict).lang = CStr::from_ptr(lang).to_string_lossy().into_owned();
                (*dict).provider.name =
                    CStr::from_ptr(provider_name).to_string_lossy().into_owned();
                (*dict).provider.desc =
                    CStr::from_ptr(provider_desc).to_string_lossy().into_owned();
                (*dict).provider.file =
                    CStr::from_ptr(provider_file).to_string_lossy().into_owned();
            }
        }

        let mut data = Default::default();

        unsafe {
            enchant_dict_describe(dict, describe_fn, &mut data as *mut _ as *mut c_void);
        };

        Self {
            dict: dict,
            broker_holder: broker_holder,
            data: data,
        }
    }

    pub fn check(&self, word: &str) -> Result<bool, String> {
        if word.is_empty() {
            return Err(String::from("empty strings cannot be checked"));
        }

        let word_length = word.len() as isize;
        let word_str = CString::new(word).unwrap();
        unsafe {
            let val = enchant_dict_check(self.dict, word_str.as_ptr(), word_length);

            if val == 0 {
                Ok(true)
            } else if val > 0 {
                Ok(false)
            } else {
                Err(CStr::from_ptr(enchant_dict_get_error(self.dict))
                    .to_string_lossy()
                    .into_owned())
            }
        }
    }

    pub fn suggest(&self, word: &str) -> Vec<String> {
        unsafe {
            let mut n_suggs = 0;

            let word_length = word.len() as isize;
            let word_str = CString::new(word).unwrap();
            let suggs =
                enchant_dict_suggest(self.dict, word_str.as_ptr(), word_length, &mut n_suggs);
            let mut out_suggestions = Vec::with_capacity(n_suggs as usize);

            if !suggs.is_null() && n_suggs != 0 {
                for i in 0..n_suggs {
                    out_suggestions.push(
                        CStr::from_ptr(*suggs.offset(i as isize))
                            .to_string_lossy()
                            .into_owned(),
                    );
                }

                enchant_dict_free_string_list(self.dict, suggs);
            }

            out_suggestions
        }
    }

    pub fn add(&self, word: &str) {
        let word_length = word.len() as isize;
        let word_str = CString::new(word).unwrap();
        unsafe {
            enchant_dict_add(self.dict, word_str.as_ptr(), word_length);
        }
    }

    pub fn add_to_session(&self, word: &str) {
        let word_length = word.len() as isize;
        let word_str = CString::new(word).unwrap();
        unsafe {
            enchant_dict_add_to_session(self.dict, word_str.as_ptr(), word_length);
        }
    }

    pub fn is_added(&self, word: &str) {
        let word_length = word.len() as isize;
        let word_str = CString::new(word).unwrap();
        unsafe {
            enchant_dict_is_added(self.dict, word_str.as_ptr(), word_length);
        }
    }

    pub fn remove(&self, word: &str) {
        let word_length = word.len() as isize;
        let word_str = CString::new(word).unwrap();
        unsafe {
            enchant_dict_remove(self.dict, word_str.as_ptr(), word_length);
        }
    }

    pub fn remove_from_session(&self, word: &str) {
        let word_length = word.len() as isize;
        let word_str = CString::new(word).unwrap();
        unsafe {
            enchant_dict_remove_from_session(self.dict, word_str.as_ptr(), word_length);
        }
    }

    pub fn is_removed(&self, word: &str) {
        let word_length = word.len() as isize;
        let word_str = CString::new(word).unwrap();
        unsafe {
            enchant_dict_is_removed(self.dict, word_str.as_ptr(), word_length);
        }
    }

    pub fn store_replacement(&self, bad: &str, good: &str) {
        let bad_length = bad.len() as isize;
        let bad_str = CString::new(bad).unwrap();
        let good_length = good.len() as isize;
        let good_str = CString::new(good).unwrap();
        unsafe {
            enchant_dict_store_replacement(
                self.dict,
                bad_str.as_ptr(),
                bad_length,
                good_str.as_ptr(),
                good_length,
            );
        }
    }

    pub fn get_lang(&self) -> &str {
        self.data.lang.as_str()
    }

    pub fn get_provider_name(&self) -> &str {
        self.data.provider.name.as_str()
    }

    pub fn get_provider_desc(&self) -> &str {
        self.data.provider.desc.as_str()
    }

    pub fn get_provider_file(&self) -> &str {
        self.data.provider.file.as_str()
    }
}

struct BrokerHolder {
    broker: *mut EnchantBroker,
}

impl Drop for BrokerHolder {
    fn drop(&mut self) {
        unsafe {
            enchant_broker_free(self.broker);
        }
    }
}

pub struct Broker {
    broker_holder: Rc<BrokerHolder>,
}

#[derive(Debug, Default)]
pub struct ProviderData {
    pub name: String,
    pub desc: String,
    pub file: String,
}

impl Broker {
    pub fn new() -> Self {
        unsafe {
            Self {
                broker_holder: Rc::new(BrokerHolder { broker: enchant_broker_init() }),
            }
        }
    }

    pub fn request_dict(&mut self, lang: &str) -> Result<Dict, String> {
        unsafe {
            let lang_str = CString::new(lang).unwrap();
            let dict = enchant_broker_request_dict(self.broker_holder.broker, lang_str.as_ptr());

            if dict.is_null() {
                let err = enchant_broker_get_error(self.broker_holder.broker);
                if err.is_null() {
                    Err(String::from("dictionary not found"))
                } else {
                    Err(CStr::from_ptr(err).to_string_lossy().into_owned())
                }
            } else {
                Ok(Dict::new(dict, self.broker_holder.clone()))
            }
        }
    }

    pub fn request_pwl_dict(&mut self, pwl: &str) -> Result<Dict, String> {
        unsafe {
            let pwl_str = CString::new(pwl).unwrap();
            let dict = enchant_broker_request_pwl_dict(self.broker_holder.broker, pwl_str.as_ptr());

            if dict.is_null() {
                Err(CStr::from_ptr(enchant_broker_get_error(self.broker_holder.broker))
                    .to_string_lossy()
                    .into_owned())
            } else {
                Ok(Dict::new(dict, self.broker_holder.clone()))
            }
        }
    }

    pub fn dict_exists(&mut self, lang: &str) -> bool {
        let lang_str = CString::new(lang).unwrap();
        unsafe { enchant_broker_dict_exists(self.broker_holder.broker, lang_str.as_ptr()) == 1 }
    }

    pub fn set_ordering(&mut self, tag: &str, ordering: &str) {
        let tag_str = CString::new(tag).unwrap();
        let ordering_str = CString::new(ordering).unwrap();
        unsafe {
            enchant_broker_set_ordering(self.broker_holder.broker, tag_str.as_ptr(), ordering_str.as_ptr());
        }
    }

    pub fn list_providers(&mut self) -> Vec<ProviderData> {
        let mut providers = Vec::new();
        extern "C" fn add_provider(
            name: *const c_char,
            desc: *const c_char,
            file: *const c_char,
            user_data: *mut c_void,
        ) {
            unsafe {
                let providers = user_data as *mut Vec<ProviderData>;
                let provider = ProviderData {
                    name: CStr::from_ptr(name).to_string_lossy().into_owned(),
                    desc: CStr::from_ptr(desc).to_string_lossy().into_owned(),
                    file: CStr::from_ptr(file).to_string_lossy().into_owned(),
                };
                (*providers).push(provider);
            }
        }

        unsafe {
            enchant_broker_describe(
                self.broker_holder.broker,
                add_provider,
                &mut providers as *mut _ as *mut c_void,
            );
        }
        providers
    }

    pub fn list_dicts(&mut self) -> Vec<DictData> {
        let mut dicts = Vec::new();
        extern "C" fn add_dict(
            lang: *const c_char,
            provider_name: *const c_char,
            provider_desc: *const c_char,
            provider_file: *const c_char,
            user_data: *mut c_void,
        ) {
            unsafe {
                let dicts = user_data as *mut Vec<DictData>;
                let dict = DictData {
                    lang: CStr::from_ptr(lang).to_string_lossy().into_owned(),
                    provider: ProviderData {
                        name: CStr::from_ptr(provider_name).to_string_lossy().into_owned(),
                        desc: CStr::from_ptr(provider_desc).to_string_lossy().into_owned(),
                        file: CStr::from_ptr(provider_file).to_string_lossy().into_owned(),
                    },
                };
                (*dicts).push(dict);
            }
        }

        unsafe {
            enchant_broker_list_dicts(self.broker_holder.broker, add_dict, &mut dicts as *mut _ as *mut c_void);
        }
        dicts
    }
}

pub fn version() -> String {
    unsafe {
        CStr::from_ptr(enchant_get_version())
            .to_string_lossy()
            .into_owned()
    }
}

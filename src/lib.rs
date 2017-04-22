extern crate libc;

use std::ffi::CStr;
use std::ffi::CString;
use libc::{ssize_t, c_void};
use std::os::raw::c_char;
use bindings::*;

mod bindings;

pub struct Dict {
    dict: *mut EnchantDict,
    broker: *mut EnchantBroker,
    data: DictData,
}

#[derive(Debug)]
pub struct DictData {
    pub lang: String,
    pub provider: ProviderData,
}

impl Drop for Dict {
    fn drop(&mut self) {
        unsafe {
            enchant_broker_free_dict(self.broker, self.dict);
        }
    }
}

impl Dict {
    fn new(dict: *mut EnchantDict, broker: *mut EnchantBroker) -> Self {
        extern "C" fn describe_fn(lang: *const c_char,
                                  provider_name: *const c_char,
                                  provider_desc: *const c_char,
                                  provider_file: *const c_char,
                                  user_data: *mut c_void) {
            unsafe {
                let mut dict = user_data as *mut DictData;
                (*dict).lang = CStr::from_ptr(lang).to_string_lossy().into_owned();
                (*dict).provider.name = CStr::from_ptr(provider_name)
                    .to_string_lossy()
                    .into_owned();
                (*dict).provider.desc = CStr::from_ptr(provider_desc)
                    .to_string_lossy()
                    .into_owned();
                (*dict).provider.file = CStr::from_ptr(provider_file)
                    .to_string_lossy()
                    .into_owned();
            }
        }

        let mut data = DictData {
            lang: String::from(""),
            provider: ProviderData {
                name: String::from(""),
                desc: String::from(""),
                file: String::from(""),
            },
        };

        unsafe {
            enchant_dict_describe(dict, describe_fn, &mut data as *mut _ as *mut c_void);
        };

        Dict {
            dict: dict,
            broker: broker,
            data: data,
        }
    }

    pub fn check(&self, word: &str) -> Result<bool, String> {
        unsafe {
            let word_length = word.len() as ssize_t;
            let val =
                enchant_dict_check(self.dict, CString::new(word).unwrap().as_ptr(), word_length);

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

    pub fn suggest(&self, word: &str) -> Box<Vec<String>> {
        unsafe {
            let mut n_suggs = 0;

            let word_length = word.len() as ssize_t;
            let suggs = enchant_dict_suggest(self.dict,
                                             CString::new(word).unwrap().as_ptr(),
                                             word_length,
                                             &mut n_suggs);
            let mut out_suggestions = Vec::with_capacity(n_suggs as usize);

            if !suggs.is_null() && n_suggs != 0 {
                for i in 0..n_suggs {
                    out_suggestions.push(CStr::from_ptr(*suggs.offset(i as isize))
                                             .to_string_lossy()
                                             .into_owned());
                }

                enchant_dict_free_string_list(self.dict, suggs);
            }

            Box::new(out_suggestions)
        }
    }

    pub fn add(&self, word: &str) {
        unsafe {
            let word_length = word.len() as ssize_t;
            enchant_dict_add(self.dict, CString::new(word).unwrap().as_ptr(), word_length);
        }
    }

    pub fn add_to_session(&self, word: &str) {
        unsafe {
            let word_length = word.len() as ssize_t;
            enchant_dict_add_to_session(self.dict,
                                        CString::new(word).unwrap().as_ptr(),
                                        word_length);
        }
    }

    pub fn is_added(&self, word: &str) {
        unsafe {
            let word_length = word.len() as ssize_t;
            enchant_dict_is_added(self.dict, CString::new(word).unwrap().as_ptr(), word_length);
        }
    }

    pub fn remove(&self, word: &str) {
        unsafe {
            let word_length = word.len() as ssize_t;
            enchant_dict_remove(self.dict, CString::new(word).unwrap().as_ptr(), word_length);
        }
    }

    pub fn remove_from_session(&self, word: &str) {
        unsafe {
            let word_length = word.len() as ssize_t;
            enchant_dict_remove_from_session(self.dict,
                                             CString::new(word).unwrap().as_ptr(),
                                             word_length);
        }
    }

    pub fn is_removed(&self, word: &str) {
        unsafe {
            let word_length = word.len() as ssize_t;
            enchant_dict_is_removed(self.dict, CString::new(word).unwrap().as_ptr(), word_length);
        }
    }

    pub fn store_replacement(&self, bad: &str, good: &str) {
        unsafe {
            let bad_length = bad.len() as ssize_t;
            let good_length = good.len() as ssize_t;
            enchant_dict_store_replacement(self.dict,
                                           CString::new(bad).unwrap().as_ptr(),
                                           bad_length,
                                           CString::new(good).unwrap().as_ptr(),
                                           good_length);
        }
    }

    pub fn get_lang(&self) -> &str {
        return self.data.lang.as_str();
    }

    pub fn get_provider_name(&self) -> &str {
        return self.data.provider.name.as_str();
    }

    pub fn get_provider_desc(&self) -> &str {
        return self.data.provider.desc.as_str();
    }

    pub fn get_provider_file(&self) -> &str {
        return self.data.provider.file.as_str();
    }
}

pub struct Broker {
    broker: *mut EnchantBroker,
}

#[derive(Debug)]
pub struct ProviderData {
    pub name: String,
    pub desc: String,
    pub file: String,
}

impl Broker {
    pub fn new() -> Self {
        unsafe { Broker { broker: enchant_broker_init() } }
    }

    pub fn request_dict(&mut self, lang: &str) -> Result<Dict, String> {
        unsafe {
            let dict = enchant_broker_request_dict(self.broker,
                                                   CString::new(lang).unwrap().as_ptr());

            if dict.is_null() {
                let err = enchant_broker_get_error(self.broker);
                if err.is_null() {
                    Err(String::from("dictionary not found"))
                } else {
                    Err(CStr::from_ptr(err).to_string_lossy().into_owned())
                }
            } else {
                Ok(Dict::new(dict, self.broker))
            }
        }
    }

    pub fn request_pwl_dict(&mut self, pwl: &str) -> Result<Dict, String> {
        unsafe {
            let dict = enchant_broker_request_pwl_dict(self.broker,
                                                       CString::new(pwl).unwrap().as_ptr());

            if dict.is_null() {
                Err(CStr::from_ptr(enchant_broker_get_error(self.broker))
                        .to_string_lossy()
                        .into_owned())
            } else {
                Ok(Dict::new(dict, self.broker))
            }
        }
    }

    pub fn dict_exists(&mut self, lang: &str) -> bool {
        unsafe {
            enchant_broker_dict_exists(self.broker, CString::new(lang).unwrap().as_ptr()) == 1
        }
    }

    pub fn set_ordering(&mut self, tag: &str, ordering: &str) {
        unsafe {
            enchant_broker_set_ordering(self.broker,
                                        CString::new(tag).unwrap().as_ptr(),
                                        CString::new(ordering).unwrap().as_ptr());
        }
    }

    pub fn list_providers(&mut self) -> Vec<ProviderData> {
        let mut providers = Vec::new();
        extern "C" fn add_provider(name: *const c_char,
                                   desc: *const c_char,
                                   file: *const c_char,
                                   user_data: *mut c_void) {
            unsafe {
                let mut providers = user_data as *mut Vec<ProviderData>;
                let provider = ProviderData {
                    name: CStr::from_ptr(name).to_string_lossy().into_owned(),
                    desc: CStr::from_ptr(desc).to_string_lossy().into_owned(),
                    file: CStr::from_ptr(file).to_string_lossy().into_owned(),
                };
                (*providers).push(provider);
            }
        }

        unsafe {
            enchant_broker_describe(self.broker,
                                    add_provider,
                                    &mut providers as *mut _ as *mut c_void);
        }
        providers
    }

    pub fn list_dicts(&mut self) -> Vec<DictData> {
        let mut dicts = Vec::new();
        extern "C" fn add_dict(lang: *const c_char,
                               provider_name: *const c_char,
                               provider_desc: *const c_char,
                               provider_file: *const c_char,
                               user_data: *mut c_void) {
            unsafe {
                let mut dicts = user_data as *mut Vec<DictData>;
                let dict = DictData {
                    lang: CStr::from_ptr(lang).to_string_lossy().into_owned(),
                    provider: ProviderData {
                        name: CStr::from_ptr(provider_name)
                            .to_string_lossy()
                            .into_owned(),
                        desc: CStr::from_ptr(provider_desc)
                            .to_string_lossy()
                            .into_owned(),
                        file: CStr::from_ptr(provider_file)
                            .to_string_lossy()
                            .into_owned(),
                    },
                };
                (*dicts).push(dict);
            }
        }

        unsafe {
            enchant_broker_list_dicts(self.broker, add_dict, &mut dicts as *mut _ as *mut c_void);
        }
        dicts
    }
}

impl Drop for Broker {
    fn drop(&mut self) {
        unsafe {
            enchant_broker_free(self.broker);
        }
    }
}

pub fn version() -> String {
    unsafe {
        CStr::from_ptr(enchant_get_version())
            .to_string_lossy()
            .into_owned()
    }
}

pub fn set_prefix_dir(prefix: &str) {
    let dir = CString::new(prefix).unwrap().as_ptr();
    unsafe {
        enchant_set_prefix_dir(dir);
    }
}

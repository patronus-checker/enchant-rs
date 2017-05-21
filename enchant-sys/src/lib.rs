use std::os::raw::{c_char, c_int, c_void};

pub enum EnchantBroker {}
pub enum EnchantDict {}

// EnchantBrokerDescribeFn
// @provider_name: The provider's identifier, such as "hunspell" or "aspell", in UTF8 encoding
// @provider_desc: A description of the provider, such as "Aspell 0.53" in UTF8 encoding
// @provider_dll_file: The provider's DLL filename in Glib file encoding (UTF8 on Windows)
// @user_data: Supplied user data, or %null if you don't care
//
// Callback used to enumerate and describe Enchant's various providers
pub type EnchantBrokerDescribeFn = extern "C" fn(provider_name: *const c_char,
                                                 provider_desc: *const c_char,
                                                 provider_dll_file: *const c_char,
                                                 user_data: *mut c_void);

// EnchantDictDescribeFn
// @lang_tag: The dictionary's language tag (eg: en_US, de_AT, ...)
// @provider_name: The provider's name (eg: Aspell) in UTF8 encoding
// @provider_desc: The provider's description (eg: Aspell 0.50.3) in UTF8 encoding
// @provider_file: The DLL/SO where this dict's provider was loaded from in Glib file encoding
// (UTF8 on Windows)
// @user_data: Supplied user data, or %null if you don't care
//
// Callback used to describe an individual dictionary
pub type EnchantDictDescribeFn = extern "C" fn(lang_tag: *const c_char,
                                               provider_name: *const c_char,
                                               provider_desc: *const c_char,
                                               provider_file: *const c_char,
                                               user_data: *mut c_void);

#[link(name = "enchant")]
extern "C" {
    pub fn enchant_get_version() -> *const c_char;

    pub fn enchant_broker_init() -> *mut EnchantBroker;
    pub fn enchant_broker_free(broker: *mut EnchantBroker);

    pub fn enchant_broker_request_dict(broker: *mut EnchantBroker,
                                       tag: *const c_char)
                                       -> *mut EnchantDict;
    pub fn enchant_broker_request_pwl_dict(broker: *mut EnchantBroker,
                                           pwl: *const c_char)
                                           -> *mut EnchantDict;
    pub fn enchant_broker_free_dict(broker: *mut EnchantBroker, dict: *mut EnchantDict);
    pub fn enchant_broker_dict_exists(broker: *mut EnchantBroker, tag: *const c_char) -> c_int;
    pub fn enchant_broker_set_ordering(broker: *mut EnchantBroker,
                                       tag: *const c_char,
                                       ordering: *const c_char);
    pub fn enchant_broker_get_error(broker: *mut EnchantBroker) -> *const c_char;


    pub fn enchant_broker_describe(broker: *mut EnchantBroker,
                                   f: EnchantBrokerDescribeFn,
                                   user_data: *mut c_void);

    pub fn enchant_dict_check(dict: *mut EnchantDict, word: *const c_char, len: isize) -> c_int;
    pub fn enchant_dict_suggest(dict: *mut EnchantDict,
                                word: *const c_char,
                                len: isize,
                                out_n_suggs: *mut usize)
                                -> *mut *mut c_char;
    pub fn enchant_dict_add(dict: *mut EnchantDict, word: *const c_char, len: isize);
    pub fn enchant_dict_add_to_session(dict: *mut EnchantDict, word: *const c_char, len: isize);
    pub fn enchant_dict_remove(dict: *mut EnchantDict, word: *const c_char, len: isize);
    pub fn enchant_dict_remove_from_session(dict: *mut EnchantDict,
                                            word: *const c_char,
                                            len: isize);
    pub fn enchant_dict_is_added(dict: *mut EnchantDict, word: *const c_char, len: isize) -> c_int;
    pub fn enchant_dict_is_removed(dict: *mut EnchantDict,
                                   word: *const c_char,
                                   len: isize)
                                   -> c_int;

    pub fn enchant_dict_store_replacement(dict: *mut EnchantDict,
                                          mis: *const c_char,
                                          mis_len: isize,
                                          cor: *const c_char,
                                          cor_len: isize);
    pub fn enchant_dict_free_string_list(dict: *mut EnchantDict, string_list: *mut *mut c_char);

    pub fn enchant_dict_get_error(dict: *mut EnchantDict) -> *const c_char;


    pub fn enchant_dict_describe(dict: *mut EnchantDict,
                                 f: EnchantDictDescribeFn,
                                 user_data: *mut c_void);

    pub fn enchant_broker_list_dicts(broker: *mut EnchantBroker,
                                     f: EnchantDictDescribeFn,
                                     user_data: *mut c_void);
}

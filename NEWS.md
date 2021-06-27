### v0.3.0 (2021-06-27)
* Fixed premature freeing of Broker (https://github.com/patronus-checker/enchant-rs/issues/4).
* Minor **BC break**: Broker no longer implements `Drop` trait, it was moved to internal reference-counted value. (https://github.com/patronus-checker/enchant-rs/pull/5).

### v0.2.0 (2019-05-22)
* Ported to Enchant 2.0.

### v0.1.1 (2019-05-22)
* Passing an empty string to `Dict.check` now returns `Err` instead of asserting in Enchant.

### v0.1.0 (2017-05-27)
* Initial commit

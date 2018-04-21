extern crate diecast;
extern crate chrono;
extern crate typemap;

use diecast::{Item, Handle};

// TODO
// it should be possible to store many/multiple date
// entries
//
// to facilitate this, we should strive to provide
// date extraction tools
//
// as well as, perhaps, a default location

pub struct Date<H, K>
where H: Fn(&Item) -> Option<chrono::NaiveDate>, H: ::std::any::Any,
      K: ::std::any::Any {
    handler: H,
    _key: ::std::marker::PhantomData<K>,
}

impl<H, K> Handle<Item> for Date<H, K>
where H: Fn(&Item) -> Option<chrono::NaiveDate>, H: ::std::any::Any,
      K: ::std::any::Any + typemap::Key<Value=chrono::NaiveDate> {
    fn handle(&self, item: &mut Item) -> diecast::Result<()> {
        let date = (self.handler)(item);

        date.map(|d| item.extensions.insert::<K>(d));

        Ok(())
    }
}

impl<H, K> typemap::Key for Date<H, K>
where H: Fn(&Item) -> Option<chrono::NaiveDate>, H: ::std::any::Any,
      K: ::std::any::Any {
    type Value = chrono::NaiveDate;
}

// TODO
// * make time type generic
// * customizable format
pub fn date<H, K>(handler: H) -> Date<H, K>
where H: Fn(&Item) -> Option<chrono::NaiveDate>, H: ::std::any::Any,
      K: ::std::any::Any {
    Date { handler: handler, _key: ::std::marker::PhantomData }
}


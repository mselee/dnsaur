use std::net::IpAddr;

use domain::{
    base::{Message, RecordSection, Rtype},
    rdata,
};

pub(crate) struct Iter<'a>(RecordSection<'a, Vec<u8>>);

impl<'a> Iterator for Iter<'a> {
    type Item = IpAddr;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(Ok(record)) = self.0.next() {
            match record.rtype() {
                Rtype::A => {
                    let data: rdata::A = record.into_record().unwrap().unwrap().into_data();
                    let addr = IpAddr::V4(data.addr());
                    Some(addr)
                }
                Rtype::AAAA => {
                    let data: rdata::Aaaa = record.into_record().unwrap().unwrap().into_data();
                    let addr = IpAddr::V6(data.addr());
                    Some(addr)
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

pub(crate) struct IpAddresses {
    message: Message<Vec<u8>>,
}

impl IpAddresses {
    pub(crate) fn iter(&self) -> Iter {
        Iter(self.message.answer().unwrap())
    }
}

impl From<Message<Vec<u8>>> for IpAddresses {
    fn from(message: Message<Vec<u8>>) -> Self {
        Self { message }
    }
}

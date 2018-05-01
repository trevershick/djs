// the point of this module is to encapsulate all the
// jenkins information and querying, etc...
//
extern crate serde_xml_rs;

use self::serde_xml_rs::deserialize;
use std::io::Read;
use djs::error::DjsError;

#[derive(Debug, Deserialize)]
struct XmlCollection {
    #[serde(rename = "number", default)] pub number: Vec<XmlElementData>,

    #[serde(rename = "relativePath", default)] pub relative_path: Vec<XmlElementData>,
}

#[derive(Debug, Deserialize)]
struct XmlElementData {
    #[serde(rename = "$value")] pub value: String,
}

impl XmlCollection {
    fn only_value(&self) -> Option<String> {
        let string_from_vec = |coll: &Vec<XmlElementData>| coll.first().map(|x| x.value.clone());
        string_from_vec(&self.number).or(string_from_vec(&self.relative_path))
    }
}

pub fn cdata_i32<'de, R: Read>(r: R) -> Result<i32, DjsError> {
    cdata_string(r).and_then(|it| {
        it.parse::<i32>().map_err(|_| {
            DjsError::XmlContentError("Unable to convert to a number.".to_string(), it)
        })
    })
}

pub fn cdata_string<'de, R: Read>(r: R) -> Result<String, DjsError> {
    let coll: Result<XmlCollection, self::serde_xml_rs::Error> = deserialize(r);
    debug!("XML Collection Data = {:?}", coll);

    let fv = coll.map(|c| c.only_value());

    match fv {
        Ok(x) => x.ok_or(DjsError::EmptyContentError),
        Err(_) => Err(DjsError::XmlContentError(
            "Couldn't extract element from list.".to_string(),
            s!("<empty collection>"),
        )),
    }
}

#[cfg(test)]
mod tests {
    mod cdata_string {
        use super::super::*;

        #[test]
        fn returns_first_string() {
            let str = r##"<x>
                <number>333</number>
                <number>337</number>
            </x>"##;

            let actual = cdata_string(str.as_bytes()).expect("Should return a value");
            assert_eq!(actual, "333");
        }

        #[test]
        fn when_nothing_returned() {
            let str = r##"<x>
            </x>"##;

            match cdata_string(str.as_bytes()) {
                Err(DjsError::EmptyContentError) => (),
                _ => panic!("should throw an EmptyContentError"),
            };
        }
    }

    mod cdata_i32 {
        use super::super::*;

        #[test]
        fn returns_first_value() {
            let str = r##"<x>
                <number>333</number>
                <number>337</number>
            </x>"##;

            let actual = cdata_i32(str.as_bytes()).expect("Should return a value");
            assert_eq!(actual, 333);
        }
        #[test]
        fn returns_xmlcontenterror_when_not_parseable() {
            let str = r##"<x>
                <number>not a number</number>
                <number>337</number>
            </x>"##;

            match cdata_i32(str.as_bytes()) {
                Err(DjsError::XmlContentError(m, v)) => {
                    assert_eq!(m, "Unable to convert to a number.");
                    assert_eq!(v, "not a number");
                }
                _ => panic!("should throw an EmptyContentError"),
            };
        }
        #[test]
        fn when_nothing_returned() {
            let str = r##"<x>
            </x>"##;

            match cdata_i32(str.as_bytes()) {
                Err(DjsError::EmptyContentError) => (),
                _ => panic!("should throw an EmptyContentError"),
            };
        }
    }
}

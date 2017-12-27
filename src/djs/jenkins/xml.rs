// the point of this module is to encapsulate all the
// jenkins information and querying, etc...
//
extern crate serde_xml_rs;

use self::serde_xml_rs::deserialize;
use std::io::Read;
use djs::error::DjsError;

#[derive(Debug, Deserialize)]
struct XmlCollection {
    #[serde(rename = "number", default)]
    pub number: Vec<XmlElementData>,

    #[serde(rename = "relativePath", default)]
    pub relative_path: Vec<XmlElementData>
}

impl XmlCollection {
    fn only_value(&self) -> Option<String> {
        let string_from_vec = |coll: &Vec<XmlElementData>| {
            coll.first().map(|x| x.value.clone())
        };
        string_from_vec(&self.number).or(string_from_vec(&self.relative_path))
    }
}

#[derive(Debug, Deserialize)]
struct XmlElementData {
   #[serde(rename = "$value")]
   pub value : String
}

pub fn cdata_i32<'de, R: Read>(r: R) -> Result<i32, DjsError> {
    cdata_string(r)
        .and_then(|it| it.parse::<i32>()
        .map_err(|_| DjsError::XmlContentError("Unable to convert returned build number to a number value.".to_string())))
}

pub fn cdata_string<'de, R: Read>(r: R) -> Result<String, DjsError> {
    let coll : Result<XmlCollection, self::serde_xml_rs::Error> = deserialize(r);
    debug!("XML Collection Data = {:?}", coll);

    let fv = coll.map(|c| c.only_value());

    match fv {
        Ok(x) => match x {
            Some(v) => Ok(v),
            None => Err(DjsError::XmlContentError("The Xml Collection contained no elements".to_string()))
        }
        Err(_) => Err(DjsError::XmlContentError("Couldn't extract element from list.".to_string()))
    }
}

// the point of this module is to encapsulate all the
// jenkins information and querying, etc...
//
extern crate serde_xml_rs;

use self::serde_xml_rs::deserialize;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct XmlElementData {
   #[serde(rename = "$value")]
   pub value : String
}

pub fn cdata_i32<'de, R: Read>(r: R) -> Result<i32, String> {
    cdata_string(r)
        .and_then(|it| it.parse::<i32>()
        .map_err(|pie| String::from("Unable to parse i32")))
}

pub fn cdata_string<'de, R: Read>(r: R) -> Result<String, String> {
    let x : Result<XmlElementData, self::serde_xml_rs::Error> = deserialize(r);
    debug!("XML Element Data = {:?}", x);

    x.map_err(|e| String::from("Unable to deserialize xml data."))
        .map(|it| it.value)
}

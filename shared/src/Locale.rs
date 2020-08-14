#[allow(non_camel_case_types)]

//
// Since rust doesn't have a reliable library 
// for locale functionality, we will create our own.
// 


// follows https://en.wikipedia.org/wiki/Language_localisation
// TODO: add the others
// TODO: add behavior for converting to strings that match standardized values 
// as reported url above
#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum Locales {
	enUS
}

impl Default for Locales {
	fn default() -> Self {
		Locales::enUS
	}
}

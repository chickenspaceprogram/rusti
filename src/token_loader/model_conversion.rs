use std::{error::Error, fmt};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Model {
    Empty,
    TI82,
    TI83,
    TI83P,
    TI84P,
    TI82A, // didn't know the TI-82 advanced was a thing!
    TI84PCSE,
    TI84PCE,
    Latest,
}

#[derive(Debug)]
pub struct ModelConversionError;

impl Error for ModelConversionError {}

impl fmt::Display for ModelConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "String did not convert into a valid model.")
    }
}

impl Model {
    pub fn get_compatible_models(model: Model) -> Vec<String> {
        // this is cursed and absolutely terrible, there has got to be a better way to do this
        // if only rust had fallthrough like C ;-;
        match model {
            Model::Empty | Model::TI82 => vec!["TI-82".to_string(), "TI-83".to_string(), "TI-83+".to_string(), "TI-84+".to_string(), "TI-82A".to_string(), "TI-84+CSE".to_string(), "TI-84+CE".to_string()],
            Model::TI83 => vec!["TI-83".to_string(), "TI-83+".to_string(), "TI-84+".to_string(), "TI-82A".to_string(), "TI-84+CSE".to_string(), "TI-84+CE".to_string()],
            Model::TI83P => vec!["TI-83+".to_string(), "TI-84+".to_string(), "TI-82A".to_string(), "TI-84+CSE".to_string(), "TI-84+CE".to_string()],
            Model::TI84P => vec!["TI-84+".to_string(), "TI-82A".to_string(), "TI-84+CSE".to_string(), "TI-84+CE".to_string()],
            Model::TI82A => vec!["TI-82A".to_string(), "TI-84+CSE".to_string(), "TI-84+CE".to_string()],
            Model::TI84PCSE => vec!["TI-84+CSE".to_string(), "TI-84+CE".to_string()],
            Model::TI84PCE | Model::Latest => vec!["TI-84+CE".to_string()],
        }
    }
    pub fn new(model: String) -> Result<Model, ModelConversionError> {
        // this sucks but it does work
        match model.as_str() {
            "" => Ok(Model::Empty),
            "TI-82" => Ok(Model::TI82),
            "TI-83" | "TI-82ST" | "TI-82ST.fr" | "TI-76.fr" => Ok(Model::TI83),
            "TI-83+" | "TI-83+SE" | "TI-83+.fr" | "TI-82+" => Ok(Model::TI83P),
            "TI-84+" | "TI-84+SE" | "TI-83+.fr:USB" | "TI-84P.fr" | "TI-84+PSE" => Ok(Model::TI84P),
            "TI-82A" | "TI-84+T" => Ok(Model::TI84P),
            "TI-84+CSE" => Ok(Model::TI84PCSE),
            "TI-84+CE" | "TI-84+CET" | "TI-83PCE" | "TI-83PCEEP" | "TI-84+CEPY" | "TI-84+CETPE" | "TI-82AEP" => Ok(Model::TI84PCE),
            "latest" => Ok(Model::Latest),
            _ => Err(ModelConversionError),
        }
    }
}


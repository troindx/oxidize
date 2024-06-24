use fluent::FluentResource;
use fluent_bundle::bundle::FluentBundle as FluentBundleConcurrent;
use std::fs;
use std::sync::Arc;
use unic_langid::LanguageIdentifier;
use super::config::OxidizeConfig;
use std::path::PathBuf;
pub struct OxidizeTranslator{
    pub config : Arc<OxidizeConfig>,
    bundle: Arc<FluentBundleConcurrent<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>>
}

impl OxidizeTranslator {
    fn load_resource(path: &str) -> FluentResource {
        let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        base.push(path);
        let source = fs::read_to_string(base).expect("Failed to read localization file");
        FluentResource::try_new(source).expect("Failed to parse localization file")
    }

    pub fn new(config: Arc<OxidizeConfig>)-> Self {
         // Load localization files
        let en_us = Self::load_resource("./i8n/en-US.ftl");
        let _es_es = Self::load_resource("./i8n/es-ES.ftl");

        // Create FluentBundle with IntlLangMemoizer
        let langid_en_us: LanguageIdentifier = "en-US".parse().expect("Parsing langid failed");
        let langid_es_es: LanguageIdentifier = "es-ES".parse().expect("Parsing langid failed");
        let mut bundle = FluentBundleConcurrent::new_concurrent(vec![langid_en_us, langid_es_es]);
        bundle.add_resource(en_us).expect("Failed to add en-US resource");
        //bundle.add_resource(es_es).expect("Failed to add es-ES resource");
        Self { config: config.clone(), bundle:Arc::new(bundle)}
    } 

    pub fn get(&self, str: &str) -> String{
        let msg = self.bundle.get_message(str)
            .expect("Message doesn't exist.");
        let mut errors = vec![];
        let pattern = msg.value()
            .expect("Message has no value.");
        let value = self.bundle.format_pattern(&pattern, None, &mut errors);
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::framework::config::OxidizeConfig;

    use super::OxidizeTranslator;

    #[test]
    fn test_oxidize_translator() {
        let config = Arc::new(OxidizeConfig::new().expect("Error creating config"));
        let translator = OxidizeTranslator::new(config);
        let value = translator.get("test");
        assert_eq!(&value, "This is a test");

    }
}
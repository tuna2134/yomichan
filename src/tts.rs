use std::env;
use jpreprocess::*;

pub fn synthesis(text: String) -> anyhow::Result<()> {
    let jtalk = JPreprocess::from_config(
        JPreprocessConfig {
            dictionary: SystemDictionaryConfig::File(env::var("DICT_PATH")?.into()),
            user_dictionary: None,
        }
    )?;
    let label = jtalk.extract_fullcontext(&text)?;
    Ok(())
}
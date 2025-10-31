use crate::settings::error::SettingsError as SE;
use crate::{settings::error::SettingsResult, settings_enum};
use std::fs;

settings_enum! {
    pub enum Mode { Easy, #[default] Normal, Hard }
}

settings_enum! {
    pub enum Theme { #[default] None, Dracula, BlackAndWhite }
}

settings_enum! {
    pub enum SoundOnClick { #[default] None, Click }
}

settings_enum! {
    pub enum TitleFont {
        #[default]
        Standard,
        AnsiRegular,
        AnsiShadow,
        Bloody,
        Graffiti,
        Puffy,
        RedPhoenix,
        Slant,
        TheEdge,
        TubesRegular,
    }
}

impl TitleFont {
    pub fn file(&self) -> String {
        if let TitleFont::Standard = self {
            panic!("`standard` font (TitleFont::Standard) is built-in and does not have a file.");
        }
        format!("./assets/fonts/{}.flf", self.as_ref())
    }

    pub fn ensure_file_exists(&self) -> SettingsResult<()> {
        if matches!(self, TitleFont::Standard) {
            return Ok(());
        }
        let file = self.file();
        if fs::metadata(&file).is_err() { Err(SE::MissingTitleFontFile(file)) } else { Ok(()) }
    }
}

settings_enum! {
    pub enum Language {
        CodeBash,
        CodeC,
        CodeCpp,
        CodeCss,
        CodeGo,
        CodeHaskell,
        CodeHtml,
        CodeJava,
        CodeJavascript,
        CodeJavascript1k,
        CodeJavascriptReact,
        CodeLua,
        CodePython,
        CodePython1k,
        CodePython2k,
        CodePython5k,
        CodeRuby,
        CodeRust,
        CodeScala,
        CodeSql,
        CodeSwift,
        CodeTypescript,
        CodeZig,
        #[default]
        English,
        English10k,
        English1k,
        English5k,
        English25k,
        English450k,
        EnglishCommonlyMisspelled,
        EnglishContractions,
        EnglishDoubleletter,
        EnglishMedical,
        EnglishOld,
        EnglishShakespearean,
    }
}

impl Language {
    pub fn file(&self) -> String {
        format!("./assets/languages/{}.json", self.as_ref())
    }

    pub fn ensure_file_exists(&self) -> SettingsResult<()> {
        let file = self.file();
        if fs::metadata(&file).is_err() { Err(SE::MissingLanguageFile(file)) } else { Ok(()) }
    }
}

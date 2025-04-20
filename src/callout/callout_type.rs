use std::fmt::Display;

use strum::{Display, EnumProperty, EnumString};

#[derive(Debug, Display, EnumString, EnumProperty, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum CalloutType {
    // Builtin callouts
    #[strum(to_string = "abstract", serialize = "개요", props(ko = "개요"))]
    Abstract,
    #[strum(to_string = "attention", serialize = "알림", props(ko = "알림"))]
    Attention,
    #[strum(to_string = "bug", serialize = "버그", props(ko = "버그"))]
    Bug,
    #[strum(to_string = "caution", serialize = "주의", props(ko = "주의"))]
    Caution,
    #[strum(to_string = "check", serialize = "확인됨", props(ko = "확인됨"))]
    Check,
    #[strum(to_string = "cite", serialize = "인용", props(ko = "인용"))]
    Cite,
    #[strum(to_string = "danger", serialize = "위험", props(ko = "위험"))]
    Danger,
    #[strum(to_string = "done", serialize = "완료", props(ko = "완료"))]
    Done,
    #[strum(to_string = "error", serialize = "오류", props(ko = "오류"))]
    Error,
    #[strum(to_string = "example", serialize = "예", props(ko = "예"))]
    Example,
    #[strum(to_string = "fail", serialize = "실패", props(ko = "실패"))]
    Fail,
    // TODO: find a better way to distinguish Fail and Failure in korean
    #[strum(to_string = "failure", serialize = "실패", props(ko = "실패"))]
    Failure,
    #[strum(
        to_string = "faq",
        serialize = "자주-묻는-질문",
        props(ko = "자주-묻는-질문")
    )]
    Faq,
    #[strum(to_string = "help", serialize = "도움말", props(ko = "도움말"))]
    Help,
    #[strum(to_string = "hint", serialize = "힌트", props(ko = "힌트"))]
    Hint,
    #[strum(to_string = "important", serialize = "중요", props(ko = "중요"))]
    Important,
    #[strum(to_string = "info", serialize = "정보", props(ko = "정보"))]
    Info,
    #[strum(to_string = "missing", serialize = "누락", props(ko = "누락"))]
    Missing,
    #[strum(to_string = "note", serialize = "노트", props(ko = "노트"))]
    Note,
    #[strum(to_string = "question", serialize = "질문", props(ko = "질문"))]
    Question,
    #[strum(to_string = "quote", serialize = "인용", props(ko = "인용"))]
    Quote,
    #[strum(to_string = "success", serialize = "성공", props(ko = "성공"))]
    Success,
    #[strum(to_string = "summary", serialize = "요약", props(ko = "요약"))]
    Summary,
    #[strum(to_string = "tip", serialize = "팁", props(ko = "팁"))]
    Tip,
    #[strum(to_string = "tldr", serialize = "요약", props(ko = "요약"))]
    Tldr,
    #[strum(to_string = "todo", serialize = "작업", props(ko = "작업"))]
    Todo,
    #[strum(to_string = "warning", serialize = "경고", props(ko = "경고"))]
    Warning,

    // Custom cto_string = "",allouts
    #[strum(to_string = "links", serialize = "링크", props(ko = "링크"))]
    Links,
    #[strum(
        to_string = "example-sentence",
        serialize = "예문-문장",
        props(ko = "예")
    )]
    ExampleSentence,
    #[strum(to_string = "exception", serialize = "예외", props(ko = "예외"))]
    Exception,
    #[strum(to_string = "reference", serialize = "참고", props(ko = "참고"))]
    Reference,
    #[strum(to_string = "rule", serialize = "규칙", props(ko = "규칙"))]
    Rule,
    #[strum(to_string = "word", serialize = "단어", props(ko = "단어"))]
    Word,
    #[strum(to_string = "conjugation", serialize = "활용", props(ko = "활용"))]
    Conjugation,
}

impl CalloutType {
    pub fn get_name(&self, lang_iso: Option<&str>) -> String {
        let default = &self.to_string();
        let name = if let Some(lang) = lang_iso {
            self.get_str(lang).unwrap_or(default)
        } else {
            default
        };
        name.to_string()
    }
}

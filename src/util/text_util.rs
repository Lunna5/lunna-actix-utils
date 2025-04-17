pub struct TextUtil;

impl TextUtil {
    pub fn i18n_key<T>(enum_key: T) -> String
    where
        T: AsRef<str>,
    {
        let text = enum_key.as_ref();
        let mut builder = String::new();

        text.chars().for_each(|c| {
            if !c.is_uppercase() {
                builder.push(c);
                return;
            }

            if !builder.is_empty() {
                builder.push('_')
            }

            builder.push(c.to_ascii_lowercase());
        });

        builder
    }

    pub fn i18n_key_with_prefix<T>(prefix: &'static str, enum_key: T) -> String
    where
        T: AsRef<str>,
    {
        format!("{}.{}", prefix, Self::i18n_key(enum_key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::text_util::tests::TestEnum::MyError;
    use strum::{AsRefStr, EnumString};

    #[derive(Debug, AsRefStr, EnumString)]
    enum TestEnum {
        MyError,
        CheckYourPassword,
    }

    #[test]
    fn test_i18n_key() {
        assert_eq!("my_error", TextUtil::i18n_key(MyError));
    }

    #[test]
    fn test_i18n_key_with_prefix() {
        assert_eq!(
            "test.check_your_password",
            TextUtil::i18n_key_with_prefix("test", TestEnum::CheckYourPassword)
        )
    }
}
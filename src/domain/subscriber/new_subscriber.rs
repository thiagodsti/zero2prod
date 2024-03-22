use serde::Deserialize;
use validator::Validate;

use crate::utils::validation::validate_name;

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct NewSubscriber {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1), custom(function = "validate_name"))]
    pub name: String,
}

impl NewSubscriber {
    pub fn new(email: String, name: String) -> Result<Self, String> {
        let new_subscriber = NewSubscriber { email, name };
        match new_subscriber.validate() {
            Ok(_) => Ok(new_subscriber),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    use quickcheck::Gen;
    use rand::prelude::StdRng;
    use rand::SeedableRng;
    use validator::Validate;

    use crate::domain::NewSubscriber;

    static VALID_EMAIL: &str = "thiago@gmail.com";
    static VALID_USERNAME: &str = "thiago_username";

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(NewSubscriber {
            name,
            email: VALID_EMAIL.parse().unwrap()
        }
        .validate());
    }
    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(NewSubscriber {
            name,
            email: VALID_EMAIL.parse().unwrap()
        }
        .validate());
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(NewSubscriber {
            name,
            email: VALID_EMAIL.parse().unwrap()
        }
        .validate());
    }

    #[test]
    fn empty_name_string_is_rejected() {
        let name = "".to_string();
        assert_err!(NewSubscriber {
            name,
            email: VALID_EMAIL.parse().unwrap()
        }
        .validate());
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(NewSubscriber {
                name,
                email: VALID_EMAIL.parse().unwrap()
            }
            .validate());
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(NewSubscriber {
            name,
            email: VALID_EMAIL.parse().unwrap()
        }
        .validate());
    }

    #[test]
    fn empty_email_string_is_rejected() {
        let email = "".to_string();
        assert_err!(NewSubscriber {
            name: VALID_USERNAME.parse().unwrap(),
            email
        }
        .validate());
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(NewSubscriber {
            name: VALID_USERNAME.parse().unwrap(),
            email
        }
        .validate());
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(NewSubscriber {
            name: VALID_USERNAME.parse().unwrap(),
            email
        }
        .validate());
    }

    #[derive(Debug, Clone)]
    struct ValidNewSubscriber(pub String, pub String);

    impl quickcheck::Arbitrary for ValidNewSubscriber {
        fn arbitrary(g: &mut Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);
            let username = fake::faker::internet::en::Username().fake_with_rng(&mut rng);
            Self(email, username)
        }
    }
    #[quickcheck_macros::quickcheck]
    fn valid_subscriptions_are_parsed_successfully(valid_subscriber: ValidNewSubscriber) -> bool {
        NewSubscriber::new(valid_subscriber.0, valid_subscriber.1).is_ok()
    }
}

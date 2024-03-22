use derive_builder::Builder;
use validator::{ValidateEmail, ValidateLength};

use crate::utils::validation::validate_name;

#[derive(Builder, Debug)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    #[builder(default = "vec![]")]
    pub roles: Vec<String>,
}

impl NewUserBuilder {
    fn validate(&self) -> Result<(), String> {
        let mut errors = vec![];
        if !self.name.validate_length(Option::from(1), None, None) {
            errors.push("invalid_username")
        }
        let _name: Option<&String> = Option::from(&self.name);
        if validate_name(_name.expect("invalid_username").as_str()).is_err() {
            errors.push("invalid_username")
        }
        if !self.email.validate_email() {
            errors.push("invalid_email")
        }
        if !self.password.validate_length(Option::from(8), None, None) {
            errors.push("invalid_password")
        }

        if errors.is_empty() {
            return Ok(());
        }
        Err(errors.join(","))
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

    use crate::domain::users::entities::new_user::{NewUser, NewUserBuilder, NewUserBuilderError};

    static VALID_EMAIL: &str = "thiago@gmail.com";
    static VALID_USERNAME: &str = "thiago_username";
    static VALID_PASSWORD: &str = "1234512345";

    fn get_valid_user_builder() -> NewUserBuilder {
        NewUserBuilder::default()
            .name(VALID_USERNAME.to_string())
            .email(VALID_EMAIL.to_string())
            .password(VALID_PASSWORD.to_string())
            .roles(vec![])
            .to_owned()
    }

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        let user = get_valid_user_builder().name(name).build();
        assert_ok!(user);
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(get_valid_user_builder().name(name).build());
    }

    #[test]
    fn empty_email_string_is_rejected() {
        let email = "".to_string();
        assert_err!(get_valid_user_builder().email(email).build());
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(get_valid_user_builder().name(name).build());
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(get_valid_user_builder().name(name).build());
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(get_valid_user_builder().name(name).build());
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(get_valid_user_builder().email(email).build());
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(get_valid_user_builder().email(email).build());
    }

    #[derive(Debug, Clone)]
    struct ValidNewUser(pub String, pub String, pub String);

    impl quickcheck::Arbitrary for ValidNewUser {
        fn arbitrary(g: &mut Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);
            let username = fake::faker::internet::en::Username().fake_with_rng(&mut rng);
            let password = fake::faker::internet::en::Password(8..10).fake_with_rng(&mut rng);
            Self(email, username, password)
        }
    }

    impl TryFrom<ValidNewUser> for NewUser {
        type Error = NewUserBuilderError;

        fn try_from(value: ValidNewUser) -> Result<Self, Self::Error> {
            NewUserBuilder::default()
                .email(value.0)
                .name(value.1)
                .password(value.2)
                .build()
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_users_are_parsed_successfully(valid_new_user: ValidNewUser) -> bool {
        NewUser::try_from(valid_new_user).is_ok()
    }
}

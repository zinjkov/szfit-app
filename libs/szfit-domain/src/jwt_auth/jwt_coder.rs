use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::jwt_auth::error::JwtAuthResult;

pub(crate) struct JwtCoder<Encoding, Decoding, Header, Validation> {
    encoding_key: Encoding,
    decoding_key: Decoding,
    header: Header,
    validation: Validation,
}

impl<Encoding, Header> JwtCoder<Encoding, (), Header, ()> {
    pub(crate) fn new_encoder(encoding_key: Encoding,
                   header: Header) -> Self {
        Self {
            encoding_key,
            decoding_key: (),
            header,
            validation: (),
        }
    }
}

impl<Decoding, Validation> JwtCoder<(), Decoding, (), Validation> {
    pub(crate) fn new_decoder(decoding_key: Decoding,
                   validation: Validation) -> Self {
        Self {
            encoding_key: (),
            decoding_key,
            header: (),
            validation,
        }
    }
}

impl<Decoding, Validation> JwtCoder<EncodingKey, Decoding, Header, Validation> {
    pub(crate) fn encode<Claims: Serialize>(&self, claims: &Claims) -> JwtAuthResult<String> {
        let token = encode(&self.header, claims, &self.encoding_key)?;

        return Ok(token);
    }
}

impl<Encoding, Header> JwtCoder<Encoding, DecodingKey, Header, Validation> {
    pub(crate) fn decode<Claims: DeserializeOwned>(&self, token: &str) -> JwtAuthResult<Claims> {
        let token_data =
            decode::<Claims>(token, &self.decoding_key, &self.validation)?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod test {
    use chrono::{Duration, Utc};
    use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
    use serde::{Deserialize, Serialize};

    use crate::entity::Id;
    use crate::jwt_auth::jwt_coder::JwtCoder;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Claims<UserPartClaims>
        where UserPartClaims: Serialize {
        pub user_claims: UserPartClaims,
        pub exp: i64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AuthClaims {
        user_id: Id,
    }

    #[test]
    fn test() {
//         let coder = JwtCoder::new(
//             EncodingKey::from_secret("secret_key".as_bytes()),
//             DecodingKey::from_secret("secret_key".as_bytes()),
//             Header::default(),
//             Validation::default());
// //
//         let default_claims = Claims { user_claims: AuthClaims { user_id: Id(1) }, exp: (Utc::now() + Duration::minutes(1)).timestamp() };
//         println!("default_claims {default_claims:?}");
//         let token = coder.encode(&default_claims).expect("expect token!");
//         println!("token {token:?}");
//         let result_claims: Claims<AuthClaims> = coder.decode(token.as_str()).unwrap();
//         println!("result_claims {result_claims:?}");
    }

    #[test]
    fn test_sep() {
        let decoder = JwtCoder::new_decoder(
            DecodingKey::from_secret("secret_key".as_bytes()),
            Validation::default());

        let encoder = JwtCoder::new_encoder(
            EncodingKey::from_secret("secret_key".as_bytes()),
            Header::default());

        let default_claims = Claims { user_claims: AuthClaims { user_id: Id(1) }, exp: (Utc::now() + Duration::minutes(1)).timestamp() };
        println!("default_claims {default_claims:?}");
        let token = encoder.encode(&default_claims).expect("expect token!");
        println!("token {token:?}");
        let result_claims: Claims<AuthClaims> = decoder.decode(token.as_str()).unwrap();
        println!("result_claims {result_claims:?}");
    }
}
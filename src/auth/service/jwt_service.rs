use crate::auth::error::AuthError;
use crate::auth::error::AuthError::NoPrivateKey;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub struct JwtService {
    private_key: Option<String>,
    public_key: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(bound(deserialize = "T: DeserializeOwned", serialize = "T: Serialize"))] // Add serde bounds
pub struct JwtDataContainer<T>
where
    T: Serialize + DeserializeOwned,
{
    pub data: T,
    exp: u64,
}

impl JwtService {
    /// Creates a new JwtService instance with private key (ability to sign)
    pub fn new(private_key: String, public_key: String) -> JwtService {
        JwtService {
            private_key: Some(private_key),
            public_key,
        }
    }

    pub fn new_without_private(public_key: String) -> JwtService {
        JwtService {
            public_key,
            private_key: None,
        }
    }

    pub fn generate_token<T>(&self, data: T, until: u64) -> Result<String, AuthError>
    where
        T: Serialize + DeserializeOwned,
    {
        self.generate_token_with_data_container(JwtDataContainer { data, exp: until })
    }

    pub fn generate_token_with_data_container<T>(
        &self,
        data: JwtDataContainer<T>,
    ) -> Result<String, AuthError>
    where
        T: Serialize + DeserializeOwned,
    {
        if self.private_key.is_none() {
            return Err(NoPrivateKey);
        }

        let private_key = self.private_key.as_ref().unwrap();

        let token = encode(
            &Header::new(Algorithm::RS256),
            &data,
            &EncodingKey::from_rsa_pem(private_key.as_bytes())
                .expect("An error occurred while building RSA key"),
        )
        .expect("Error occurred while generating token");

        Ok(token)
    }

    pub fn verify_token<T>(&self, token: &str) -> Result<JwtDataContainer<T>, AuthError>
    where
        T: Serialize + DeserializeOwned + Send + Sync,
    {
        let validation = Validation::new(Algorithm::RS256);
        let token_data = decode::<JwtDataContainer<T>>(
            token,
            &DecodingKey::from_rsa_pem(self.public_key.as_bytes())
                .expect("An error occurred while building RSA key"),
            &validation,
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => AuthError::InvalidToken,
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::InvalidToken,
        })?;

        if token_data.claims.exp < get_current_time() {
            return Err(AuthError::TokenExpired);
        }

        Ok(token_data.claims)
    }
}

pub fn get_current_time() -> u64 {
    Utc::now().timestamp().max(0) as u64
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    const RSA_PUBLIC_TEST_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAu1SU1LfVLPHCozMxH2Mo
4lgOEePzNm0tRgeLezV6ffAt0gunVTLw7onLRnrq0/IzW7yWR7QkrmBL7jTKEn5u
+qKhbwKfBstIs+bMY2Zkp18gnTxKLxoS2tFczGkPLPgizskuemMghRniWaoLcyeh
kd3qqGElvW/VDL5AaWTg0nLVkjRo9z+40RQzuVaE8AkAFmxZzow3x+VJYKdjykkJ
0iT9wCS0DRTXu269V264Vf/3jvredZiKRkgwlL9xNAwxXFg0x/XFw005UWVRIkdg
cKWTjpBP2dPwVZ4WWC+9aGVd+Gyn1o0CLelf4rEjGoXbAAEgAqeGUxrcIlbjXfbc
mwIDAQAB
-----END PUBLIC KEY-----"#;

    const RSA_PRIVATE_TEST_KEY: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvwIBADANBgkqhkiG9w0BAQEFAASCBKkwggSlAgEAAoIBAQC7VJTUt9Us8cKj
MzEfYyjiWA4R4/M2bS1GB4t7NXp98C3SC6dVMvDuictGeurT8jNbvJZHtCSuYEvu
NMoSfm76oqFvAp8Gy0iz5sxjZmSnXyCdPEovGhLa0VzMaQ8s+CLOyS56YyCFGeJZ
qgtzJ6GR3eqoYSW9b9UMvkBpZODSctWSNGj3P7jRFDO5VoTwCQAWbFnOjDfH5Ulg
p2PKSQnSJP3AJLQNFNe7br1XbrhV//eO+t51mIpGSDCUv3E0DDFcWDTH9cXDTTlR
ZVEiR2BwpZOOkE/Z0/BVnhZYL71oZV34bKfWjQIt6V/isSMahdsAASACp4ZTGtwi
VuNd9tybAgMBAAECggEBAKTmjaS6tkK8BlPXClTQ2vpz/N6uxDeS35mXpqasqskV
laAidgg/sWqpjXDbXr93otIMLlWsM+X0CqMDgSXKejLS2jx4GDjI1ZTXg++0AMJ8
sJ74pWzVDOfmCEQ/7wXs3+cbnXhKriO8Z036q92Qc1+N87SI38nkGa0ABH9CN83H
mQqt4fB7UdHzuIRe/me2PGhIq5ZBzj6h3BpoPGzEP+x3l9YmK8t/1cN0pqI+dQwY
dgfGjackLu/2qH80MCF7IyQaseZUOJyKrCLtSD/Iixv/hzDEUPfOCjFDgTpzf3cw
ta8+oE4wHCo1iI1/4TlPkwmXx4qSXtmw4aQPz7IDQvECgYEA8KNThCO2gsC2I9PQ
DM/8Cw0O983WCDY+oi+7JPiNAJwv5DYBqEZB1QYdj06YD16XlC/HAZMsMku1na2T
N0driwenQQWzoev3g2S7gRDoS/FCJSI3jJ+kjgtaA7Qmzlgk1TxODN+G1H91HW7t
0l7VnL27IWyYo2qRRK3jzxqUiPUCgYEAx0oQs2reBQGMVZnApD1jeq7n4MvNLcPv
t8b/eU9iUv6Y4Mj0Suo/AU8lYZXm8ubbqAlwz2VSVunD2tOplHyMUrtCtObAfVDU
AhCndKaA9gApgfb3xw1IKbuQ1u4IF1FJl3VtumfQn//LiH1B3rXhcdyo3/vIttEk
48RakUKClU8CgYEAzV7W3COOlDDcQd935DdtKBFRAPRPAlspQUnzMi5eSHMD/ISL
DY5IiQHbIH83D4bvXq0X7qQoSBSNP7Dvv3HYuqMhf0DaegrlBuJllFVVq9qPVRnK
xt1Il2HgxOBvbhOT+9in1BzA+YJ99UzC85O0Qz06A+CmtHEy4aZ2kj5hHjECgYEA
mNS4+A8Fkss8Js1RieK2LniBxMgmYml3pfVLKGnzmng7H2+cwPLhPIzIuwytXywh
2bzbsYEfYx3EoEVgMEpPhoarQnYPukrJO4gwE2o5Te6T5mJSZGlQJQj9q4ZB2Dfz
et6INsK0oG8XVGXSpQvQh3RUYekCZQkBBFcpqWpbIEsCgYAnM3DQf3FJoSnXaMhr
VBIovic5l0xFkEHskAjFTevO86Fsz1C2aSeRKSqGFoOQ0tmJzBEs1R6KqnHInicD
TQrKhArgLXX4v3CddjfTRJkFWDbE/CkvKZNOrcf1nhaGCPspRJj2KUkj1Fhl9Cnc
dn/RsYEONbwQSjIfMPkvxF+8HQ==
-----END PRIVATE KEY-----"#;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    struct SimpleUser {
        username: String,
        email: String,
        age: i8,
    }

    #[test]
    fn test_token_generation() {
        let example_user: SimpleUser = SimpleUser {
            username: "Lunna".to_string(),
            email: "hi@lunna.dev".to_string(),
            age: 17,
        };

        let service = JwtService::new(
            String::from(RSA_PRIVATE_TEST_KEY),
            String::from(RSA_PUBLIC_TEST_KEY),
        );

        match service.generate_token(
            example_user,
            get_current_time() + Duration::from_secs(10).as_secs(),
        ) {
            Ok(token) => {
                println!("Token generated: {:?}", token);
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    #[should_panic]
    fn token_expired() {
        let service = JwtService::new(
            String::from(RSA_PRIVATE_TEST_KEY),
            String::from(RSA_PUBLIC_TEST_KEY),
        );

        // We are expecting these to fail
        service.verify_token::<SimpleUser>("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJkYXRhIjp7InVzZXJuYW1lIjoiQW5nZWxpbGxvMTUiLCJlbWFpbCI6ImNvbnRhY3RAYW5nZWxpbGxvMTUuZXMiLCJhZ2UiOjE3fSwiZXhwaXJhdGlvbl90aW1lIjoxNzM5MDMzNjA1fQ.IrZL5j08FEC3g85YYTCbSJ0MbnF1FILPgJC2fTW6ERH1RM-NwEZig8FZt17GOWbpDys3E5gXsDYPjOWy7Ibt503VTVmkFSYnPwESgy4-VHBoDLqQ19PrAQfU40KUpBLF1NuXhlYo-UyR65pZq50UIfTnD90UeKovnRb48jb65g_7_lWBRRZQ-HysychTQWzxMEo24_8gk3PIRE5Nzdv0bbPCrUurqcwYbs9ZpdRkN2BX0wo9g4sY4eMcaaEGMU46SmD3zVS2eiE7vu6ZgIY8WUK2bcC0MGGOQqkbexX1VxZ4AARhyzFTIleYNs5c5C1GGN6uha0acTv8dZCnJabS5g").expect("The token must be expired");
    }

    #[test]
    fn valid_token_verification() {
        let example_user: SimpleUser = SimpleUser {
            username: "Lunna".to_string(),
            email: "hi@lunna.dev".to_string(),
            age: 17,
        };

        let service = JwtService::new(
            String::from(RSA_PRIVATE_TEST_KEY),
            String::from(RSA_PUBLIC_TEST_KEY),
        );

        let token = service
            .generate_token(
                example_user,
                get_current_time() + Duration::from_secs(5).as_secs(),
            )
            .expect("Token generated");

        let result = service.verify_token::<SimpleUser>(&token);

        match result {
            Ok(_) => {}
            Err(e) => panic!("{:?}", e),
        }
    }
}

use biscuit::jwa::{
    ContentEncryptionAlgorithm, EncryptionOptions, KeyManagementAlgorithm, SignatureAlgorithm,
};
use biscuit::jws::Secret;
use biscuit::{jwa::SecureRandom, jwk::JWK, Empty};
use biscuit::{jwe, jws, ClaimsSet, RegisteredClaims, SingleOrMultiple, Timestamp, JWE, JWT};
use chrono::{Duration, Utc};
use ring::{error::Unspecified, rand::SystemRandom};
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{atomic::AtomicU64, Arc};
use thiserror::Error;
use webauthn_rs::prelude::{PasskeyRegistration, Uuid};

#[derive(Clone)]
pub struct JweService {
    jwe_key: JWK<Empty>,
    jwt_key: Secret,
    app_domain: String,
    nonce: Arc<AtomicU64>,
}

impl JweService {
    pub fn create(rand_gen: SystemRandom, app_domain: String) -> Result<Self, JweServiceError> {
        let jwe_rand: [u8; 256 / 8];
        rand_gen.fill(&mut jwe_rand)?;

        let jwt_rand: [u8; 256 / 8];
        rand_gen.fill(&mut jwt_rand)?;

        let jwe_key = JWK::new_octet_key(&jwe_rand, Default::default());

        Ok(Self {
            jwe_key,
            jwt_key: Secret::Bytes(jwt_rand.to_vec()),
            app_domain,
            nonce: Arc::new(AtomicU64::new(0)),
        })
    }

    pub fn encrypt_registration_token(
        &self,
        username: String,
        unique_id: Uuid,
        passkey: PasskeyRegistration,
    ) -> Result<String, JweServiceError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(5))
            .ok_or(JweServiceError::Overflow)?;

        let expected_claims = ClaimsSet::<RegistrationClaims> {
            registered: RegisteredClaims {
                audience: Some(SingleOrMultiple::Single(self.app_domain)),
                not_before: Some(Timestamp::from(Utc::now())),
                expiry: Some(Timestamp::from(expiration)),
                ..Default::default()
            },
            private: RegistrationClaims {
                username,
                unique_id,
                passkey,
            },
        };

        let expected_jwt = JWT::new_decoded(
            From::from(jws::RegisteredHeader {
                algorithm: SignatureAlgorithm::HS256,
                ..Default::default()
            }),
            expected_claims,
        );

        let jws = expected_jwt.into_encoded(&self.jwt_key)?;

        let mut nonce_bytes: [u8; 8] = self.nonce.fetch_add(1, Relaxed).to_be_bytes();
        let mut nonce = Vec::from(nonce_bytes);
        nonce.resize(96 / 8, 0);
        let options = EncryptionOptions::AES_GCM { nonce };

        let jwe = JWE::new_decrypted(
            From::from(jwe::RegisteredHeader {
                cek_algorithm: KeyManagementAlgorithm::A256GCMKW,
                enc_algorithm: ContentEncryptionAlgorithm::A256GCM,
                media_type: Some("JOSE".to_string()),
                content_type: Some("JOSE".to_string()),
                ..Default::default()
            }),
            jws,
        );

        let encrypted_jwe = jwe.encrypt(&self.jwe_key, &options)?;

        Ok(encrypted_jwe.unwrap_encrypted().to_string())
    }

    pub fn decrypt_registration_token(
        &self,
        encrypted_token: String,
    ) -> Result<RegistrationClaims, JweServiceError> {
        let token: JWE<RegistrationClaims, Empty, Empty> = JWE::new_encrypted(&encrypted_token);

        // Decrypt
        let decrypted_jwe = token.into_decrypted(
            &self.jwe_key,
            KeyManagementAlgorithm::A256GCMKW,
            ContentEncryptionAlgorithm::A256GCM,
        )?;

        let decrypted_jws = decrypted_jwe.payload()?;

        let jwt = decrypted_jws.into_decoded(&self.jwt_key, SignatureAlgorithm::HS256)?;

        Ok(jwt.payload()?.private)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RegistrationClaims {
    username: String,
    unique_id: Uuid,
    passkey: PasskeyRegistration,
}

#[derive(Debug, Error)]
pub enum JweServiceError {
    #[error(transparent)]
    Biscuit(#[from] biscuit::errors::Error),
    #[error("Datatime Overflow")]
    Overflow,
    #[error(transparent)]
    Rng(#[from] Unspecified),
}

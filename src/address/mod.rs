use actix_web::{error, http, HttpRequest};
use ethers::types::{Address, Signature};
use ethers::utils::keccak256;
use hex::decode;

pub fn check_address(req: &HttpRequest, address: String) -> Result<bool, error::Error> {
    let authorization_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .ok_or(error::ErrorInternalServerError("No authorization provided"))?;

    let auth = authorization_header
        .to_str()
        .map_err(|_| error::ErrorInternalServerError("Invalid authorization header format"))?;

    let recovered_address = recover_eth_address("Controller message", &auth)?;

    Ok(recovered_address.to_string() != address.to_string())
}

pub fn recover_eth_address(
    message: &str,
    signature: &str,
) -> Result<Address, Box<dyn std::error::Error>> {
    let prefix = format!("\x19Ethereum Signed Message:\n{}", message.len());
    let prefixed_message = [prefix.as_bytes(), message.as_bytes()].concat();

    // Hash the prefixed message
    let message_hash = keccak256(prefixed_message);

    // Decode the signature from hex
    let signature_bytes = decode(signature.trim_start_matches("0x"))?;
    let signature = Signature::try_from(signature_bytes.as_ref())?;

    // Recover the signer's address
    let address = signature.recover(message_hash)?;

    Ok(address)
}

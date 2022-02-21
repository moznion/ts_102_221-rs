use crate::class::Class;
use anyhow::Result;
use thiserror::Error;

/// ClassByte: ref 10.1.2 / ETSI TS 102 221 V15.0.0
pub trait Instruction {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError>;
}

#[derive(Debug, Error, PartialEq)]
pub enum InstructionError {
    #[error("invalid class byte for instruction '{0}'; the class byte according to the following pattern: {1}; but that is '{2}'")]
    InvalidClassByte(u8, String, u8),
}

pub struct SelectFile {}

impl Instruction for SelectFile {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0xa4;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct Status {}

impl Instruction for Status {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0xf2;
        match validate(code, class, &[0x80, 0xc0, 0xe0], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct ReadBinary {}

impl Instruction for ReadBinary {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0xb0;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct UpdateBinary {}

impl Instruction for UpdateBinary {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0xd6;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct ReadRecord {}

impl Instruction for ReadRecord {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0xb2;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct UpdateRecord {}

impl Instruction for UpdateRecord {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0xdc;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct SearchRecord {}

impl Instruction for SearchRecord {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0xa2;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct Increase {}

impl Instruction for Increase {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x32;
        match validate(code, class, &[0x80, 0xc0, 0xe0], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct RetrieveData {}

impl Instruction for RetrieveData {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0xcb;
        match validate(code, class, &[0x80, 0xc0, 0xe0], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct SetData {}

impl Instruction for SetData {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0xdb;
        match validate(code, class, &[0x80, 0xc0, 0xe0], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct VerifyPin {}

impl Instruction for VerifyPin {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x20;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct ChangePin {}

impl Instruction for ChangePin {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x24;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct DisablePin {}

impl Instruction for DisablePin {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x26;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct EnablePin {}

impl Instruction for EnablePin {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x28;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct UnblockPin {}

impl Instruction for UnblockPin {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x2c;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct DeactivateFile {}

impl Instruction for DeactivateFile {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x04;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct ActivateFile {}

impl Instruction for ActivateFile {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x44;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct Authenticate {}

impl Instruction for Authenticate {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x88;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct GetChallenge {}

impl Instruction for GetChallenge {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x84;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct TerminalCapability {}

impl Instruction for TerminalCapability {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0xaa;
        match validate(code, class, &[0x80, 0xc0, 0xe0], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct TerminalProfile {}

impl Instruction for TerminalProfile {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x10;
        match validate(code, class, &[0x80], true) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct Envelope {}

impl Instruction for Envelope {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0xc2;
        match validate(code, class, &[0x80], true) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct Fetch {}

impl Instruction for Fetch {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x12;
        match validate(code, class, &[0x80], true) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct TerminalResponse {}

impl Instruction for TerminalResponse {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x14;
        match validate(code, class, &[0x80], true) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct ManageChannel {}

impl Instruction for ManageChannel {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x70;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct ManageSecureChannel {}

impl Instruction for ManageSecureChannel {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x73;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct TransactData {}

impl Instruction for TransactData {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x75;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct SuspendUICC {}

impl Instruction for SuspendUICC {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x76;
        match validate(code, class, &[0x80], true) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct GetIdentity {}

impl Instruction for GetIdentity {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0x78;
        match validate(code, class, &[0x80, 0xc0, 0xe0], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

pub struct GetResponse {}

impl Instruction for GetResponse {
    fn get_byte(&self, class: &Class) -> Result<u8, InstructionError> {
        let code = 0xc0;
        match validate(code, class, &[0x00, 0x40, 0x60], false) {
            None => Ok(code),
            Some(e) => Err(e),
        }
    }
}

fn validate(instruction: u8, class: &Class, patterns: &[u8], exact: bool) -> Option<InstructionError> {
    let b = class.get_byte();

    let mut patterns_for_err = Vec::new();
    for pattern in patterns {
        if exact {
            if b == *pattern {
                return Option::None;
            }
        } else if b & 0xf0 == *pattern {
            return Option::None;
        }

        patterns_for_err.push(format!("'{:#04x}'", *pattern));
    }

    Option::Some(InstructionError::InvalidClassByte(instruction, patterns_for_err.join(" or "), b))
}

#[cfg(test)]
mod test {
    use crate::class::{ClassTypeForExtendedLogicalChannels, ClassTypeForStandardLogicalChannels, new_extended_class, new_standard_class, SecureMessagingIndicationForExtendedLogicalChannels, SecureMessagingIndicationForStandardLogicalChannels};
    use crate::instruction::{Fetch, Instruction, InstructionError, SelectFile};

    #[test]
    fn should_get_byte_successfully() {
        let sf = SelectFile {};

        let class = new_standard_class(ClassTypeForStandardLogicalChannels::ISOIEC7816_4, SecureMessagingIndicationForStandardLogicalChannels::CommandHeaderAuthenticated, 3).unwrap(); // 0b00001111
        assert_eq!(sf.get_byte(&class).unwrap(), 0xa4);

        let class = new_extended_class(ClassTypeForExtendedLogicalChannels::ISOIEC7816_4, SecureMessagingIndicationForExtendedLogicalChannels::NoSM, 15).unwrap(); // 0b01001111
        assert_eq!(sf.get_byte(&class).unwrap(), 0xa4);

        let class = new_extended_class(ClassTypeForExtendedLogicalChannels::ISOIEC7816_4, SecureMessagingIndicationForExtendedLogicalChannels::CommandHeaderNotAuthenticated, 15).unwrap(); // 0b01101111
        assert_eq!(sf.get_byte(&class).unwrap(), 0xa4);
    }

    #[test]
    fn should_fail_get_byte_when_class_is_unsuitable() {
        let sf = SelectFile {};

        let class = new_standard_class(ClassTypeForStandardLogicalChannels::TS102_221, SecureMessagingIndicationForStandardLogicalChannels::CommandHeaderAuthenticated, 3).unwrap(); // 0b10001111
        assert_eq!(sf.get_byte(&class).unwrap_err(), InstructionError::InvalidClassByte(0xa4, "'0x00' or '0x40' or '0x60'".into(), 0b10001111));

        let class = new_standard_class(ClassTypeForStandardLogicalChannels::OTHER, SecureMessagingIndicationForStandardLogicalChannels::CommandHeaderAuthenticated, 3).unwrap(); // 0b10101111
        assert_eq!(sf.get_byte(&class).unwrap_err(), InstructionError::InvalidClassByte(0xa4, "'0x00' or '0x40' or '0x60'".into(), 0b10101111));

        let class = new_extended_class(ClassTypeForExtendedLogicalChannels::TS102_221, SecureMessagingIndicationForExtendedLogicalChannels::NoSM, 15).unwrap(); // 0b11001111
        assert_eq!(sf.get_byte(&class).unwrap_err(), InstructionError::InvalidClassByte(0xa4, "'0x00' or '0x40' or '0x60'".into(), 0b11001111));
    }

    #[test]
    fn should_get_byte_with_exact_successfully() {
        let fetch = Fetch {};

        let class = new_standard_class(ClassTypeForStandardLogicalChannels::TS102_221, SecureMessagingIndicationForStandardLogicalChannels::NoSM, 0).unwrap(); // 0b10000000
        assert_eq!(fetch.get_byte(&class).unwrap(), 0x12);
    }

    #[test]
    fn should_fail_get_byte_with_exact() {
        let fetch = Fetch {};

        let class = new_standard_class(ClassTypeForStandardLogicalChannels::TS102_221, SecureMessagingIndicationForStandardLogicalChannels::NoSM, 1).unwrap(); // 0b10000001
        assert_eq!(fetch.get_byte(&class).unwrap_err(), InstructionError::InvalidClassByte(0x12, "'0x80'".into(), 0b10000001));
    }
}

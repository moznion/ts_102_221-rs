use anyhow::Result;
use thiserror::Error;

/// Class: ref 10.1.1 / ETSI TS 102 221 V15.0.0
pub struct Class {
    byte: u8,
}

#[repr(u8)]
pub enum ClassTypeForStandardLogicalChannels {
    ISOIEC7816_4 = 0b00000000,
    TS102_221 = 0b10000000,
    OTHER = 0b10100000,
}

#[repr(u8)]
pub enum SecureMessagingIndicationForStandardLogicalChannels {
    NoSM = 0b00000000,
    ProprietarySM = 0b00000100,
    CommandHeaderNotAuthenticated = 0b00001000,
    CommandHeaderAuthenticated = 0b00001100,
}

#[repr(u8)]
pub enum ClassTypeForExtendedLogicalChannels {
    ISOIEC7816_4 = 0b01000000,
    TS102_221 = 0b11000000,
}

#[repr(u8)]
pub enum SecureMessagingIndicationForExtendedLogicalChannels {
    NoSM = 0b00000000,
    CommandHeaderNotAuthenticated = 0b00100000,
}

#[derive(Debug, Error, PartialEq)]
pub enum ClassError {
    #[error("invalid number of standard logical channel; this must be within [0, 3] but the given value is {0}")]
    InvalidNumberOfStandardLogicalChannel(u8),
    #[error("invalid number of extended logical channel; this must be within [0, 15] (internally, it adds 4 to that number of channel) but the given value is {0}")]
    InvalidNumberOfExtendedLogicalChannel(u8),
}

pub fn new_standard_class(
    typ: ClassTypeForStandardLogicalChannels,
    secure_messaging_indication: SecureMessagingIndicationForStandardLogicalChannels,
    logical_channel_number: u8,
) -> Result<Class, ClassError> {
    if logical_channel_number >= 4 {
        return Err(ClassError::InvalidNumberOfStandardLogicalChannel(
            logical_channel_number,
        ));
    }

    Ok(Class {
        byte: typ as u8 | secure_messaging_indication as u8 | logical_channel_number,
    })
}

pub fn new_extended_class(
    typ: ClassTypeForExtendedLogicalChannels,
    secure_messaging_indication: SecureMessagingIndicationForExtendedLogicalChannels,
    logical_channel_number: u8,
) -> Result<Class, ClassError> {
    if logical_channel_number >= 16 {
        return Err(ClassError::InvalidNumberOfExtendedLogicalChannel(
            logical_channel_number,
        ));
    }

    Ok(Class {
        byte: typ as u8 | secure_messaging_indication as u8 | logical_channel_number,
    })
}

impl Class {
    pub fn get_byte(&self) -> u8 {
        self.byte
    }
}

#[cfg(test)]
mod test {
    use crate::class::{
        new_extended_class, new_standard_class, ClassError, ClassTypeForExtendedLogicalChannels,
        ClassTypeForStandardLogicalChannels, SecureMessagingIndicationForExtendedLogicalChannels,
        SecureMessagingIndicationForStandardLogicalChannels,
    };

    #[test]
    fn should_new_standard_class_successfully() {
        let result = new_standard_class(
            ClassTypeForStandardLogicalChannels::ISOIEC7816_4,
            SecureMessagingIndicationForStandardLogicalChannels::NoSM,
            0,
        );
        assert_eq!(result.unwrap().get_byte(), 0b00000000);
    }

    #[test]
    fn should_new_extended_class_successfully() {
        let result = new_extended_class(
            ClassTypeForExtendedLogicalChannels::ISOIEC7816_4,
            SecureMessagingIndicationForExtendedLogicalChannels::NoSM,
            0,
        );
        assert_eq!(result.unwrap().get_byte(), 0b01000000);
    }

    #[test]
    fn should_fail_new_standard_class_with_exceeded_channel_num() {
        let result = new_standard_class(
            ClassTypeForStandardLogicalChannels::ISOIEC7816_4,
            SecureMessagingIndicationForStandardLogicalChannels::NoSM,
            4,
        );
        assert_eq!(
            result.err().unwrap(),
            ClassError::InvalidNumberOfStandardLogicalChannel(4)
        );
    }

    #[test]
    fn should_fail_new_extended_class_with_exceeded_channel_num() {
        let result = new_extended_class(
            ClassTypeForExtendedLogicalChannels::ISOIEC7816_4,
            SecureMessagingIndicationForExtendedLogicalChannels::NoSM,
            16,
        );
        assert_eq!(
            result.err().unwrap(),
            ClassError::InvalidNumberOfExtendedLogicalChannel(16)
        );
    }
}

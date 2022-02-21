use anyhow::Result;
use thiserror::Error;

use crate::class::Class;
use crate::instruction::Instruction;

/// CommandAPDU: ref 10.1.0 / ETSI TS 102 221 V15.0.0
pub struct CommandAPDU<'a> {
    class: &'a Class,
    instruction: &'a dyn Instruction,
    p1: u8,
    p2: u8,
    max_response_byte_size: Option<u8>,
    command_data: Option<&'a [u8]>,
}

pub fn new_command_apdu<'a>(
    class: &'a Class,
    instruction: &'a dyn Instruction,
    p1: u8,
    p2: u8,
    le: Option<u8>,
    command_data: Option<&'a [u8]>,
) -> CommandAPDU<'a> {
    CommandAPDU {
        class,
        instruction,
        p1,
        p2,
        max_response_byte_size: le,
        command_data,
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum CommandAPDUError {
    #[error("failed to construct command APDU bytes: {0}")]
    FailedBytesConstruction(String),
    #[error("illegal length of the command data; this must be within [0, 255], but {0}")]
    IllegalCommandDataLength(usize),
}

impl<'a> CommandAPDU<'a> {
    pub fn to_bytes(&self) -> Result<Vec<u8>, CommandAPDUError> {
        let instruction_byte = match self.instruction.get_byte(self.class) {
            Ok(b) => b,
            Err(e) => return Err(CommandAPDUError::FailedBytesConstruction(e.to_string())),
        };

        let mut bytes = Vec::from([self.class.get_byte(), instruction_byte, self.p1, self.p2]);

        if self.command_data.is_some() {
            let command_data_bytes = self.command_data.unwrap();
            let command_data_bytes_len = command_data_bytes.len();
            if command_data_bytes_len > 255 {
                return Err(CommandAPDUError::IllegalCommandDataLength(
                    command_data_bytes_len,
                ));
            }
            bytes.push(command_data_bytes_len as u8);
            bytes.extend_from_slice(command_data_bytes);
        }

        if self.max_response_byte_size.is_some() {
            bytes.push(self.max_response_byte_size.unwrap())
        }

        Ok(bytes)
    }
}

#[cfg(test)]
mod test {
    use crate::class::{
        new_standard_class, ClassTypeForStandardLogicalChannels,
        SecureMessagingIndicationForStandardLogicalChannels,
    };
    use crate::command_apdu::new_command_apdu;
    use crate::instruction::SelectFile;

    #[test]
    fn should_construct_command_with_data() {
        let class = new_standard_class(
            ClassTypeForStandardLogicalChannels::ISOIEC7816_4,
            SecureMessagingIndicationForStandardLogicalChannels::NoSM,
            0,
        )
        .unwrap();
        let instruction = SelectFile {};
        let command_data = [0x6f, 0x61];
        let apdu = new_command_apdu(
            &class,
            &instruction,
            0x00,
            0x04,
            Option::None,
            Option::Some(&command_data),
        );
        let result = apdu.to_bytes();
        assert_eq!(
            result.unwrap(),
            Vec::from([0x00, 0xa4, 0x00, 0x04, 0x02, 0x6f, 0x61])
        );
    }
}

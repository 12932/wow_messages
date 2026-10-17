use std::io::{Read, Write};

use crate::vanilla::GmSurveyQuestion;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm:17`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm#L17):
/// ```text
/// cmsg CMSG_GMSURVEY_SUBMIT = 0x032A {
///     u32 survey_id;
///     GmSurveyQuestionArray questions;
///     CString answer_comment;
/// }
/// ```
pub struct CMSG_GMSURVEY_SUBMIT {
    /// cmangos: Survey ID: found in GMSurveySurveys.dbc
    pub survey_id: u32,
    /// mangoszero/cmangos: Up to 10, a question id of 0 ending them when there are fewer
    pub questions: Vec<GmSurveyQuestion>,
    /// cmangos: Answer comment: Unused in stock UI, can be only set by calling Lua function
    /// cmangos: Answer comment max sizes in bytes: Vanilla - 8106:8110, TBC - 11459:11463, Wrath - 582:586
    pub answer_comment: String,
}

impl crate::private::Sealed for CMSG_GMSURVEY_SUBMIT {}
impl CMSG_GMSURVEY_SUBMIT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(9..=2870).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // survey_id: u32
        let survey_id = crate::util::read_u32_le(&mut r)?;

        // questions: GmSurveyQuestionArray
        let questions = crate::util::read_gm_survey_questions(&mut r)?;

        // answer_comment: CString
        let answer_comment = {
            let answer_comment = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(answer_comment)?
        };

        Ok(Self {
            survey_id,
            questions,
            answer_comment,
        })
    }

}

impl crate::Message for CMSG_GMSURVEY_SUBMIT {
    const OPCODE: u32 = 0x032a;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_GMSURVEY_SUBMIT"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // survey_id: u32
        w.write_all(&self.survey_id.to_le_bytes())?;

        // questions: GmSurveyQuestionArray
        crate::util::write_gm_survey_questions(self.questions.as_slice(), &mut w)?;

        // answer_comment: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.answer_comment.as_bytes().iter().next_back(), Some(&0_u8), "String `answer_comment` must not be null-terminated.");
        w.write_all(self.answer_comment.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(810, "CMSG_GMSURVEY_SUBMIT", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GMSURVEY_SUBMIT {}

impl CMSG_GMSURVEY_SUBMIT {
    pub(crate) fn size(&self) -> usize {
        4 // survey_id: u32
        + crate::util::gm_survey_questions_size(self.questions.as_slice()) // questions: GmSurveyQuestionArray
        + self.answer_comment.len() + 1 // answer_comment: CString
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_GMSURVEY_SUBMIT;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_GMSURVEY_SUBMIT, expected: &CMSG_GMSURVEY_SUBMIT) {
        assert_eq!(t.survey_id, expected.survey_id);
        assert_eq!(t.questions, expected.questions);
        assert_eq!(t.answer_comment, expected.answer_comment);
    }

    const RAW0: [u8; 35] = [ 0x00, 0x21, 0x2A, 0x03, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, 0x07, 0x00, 0x00, 0x00, 0x05, 0x00, 0x08, 0x00, 0x00, 0x00, 0x03,
         0x6F, 0x6B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x68, 0x61, 0x6E, 0x6B,
         0x73, 0x00, ];

    pub(crate) fn expected0() -> CMSG_GMSURVEY_SUBMIT {
        CMSG_GMSURVEY_SUBMIT {
            survey_id: 0x1,
            questions: vec![
                GmSurveyQuestion {
                    question_id: 0x7,
                    answer: 0x5,
                    comment: String::from(""),
                },
                GmSurveyQuestion {
                    question_id: 0x8,
                    answer: 0x3,
                    comment: String::from("ok"),
                },
            ],
            answer_comment: String::from("Thanks"),
        }

    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm` line 33.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_gmsurvey_submit0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_GMSURVEY_SUBMIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_GMSURVEY_SUBMIT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm` line 33.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_gmsurvey_submit0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_GMSURVEY_SUBMIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_GMSURVEY_SUBMIT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm` line 33.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_gmsurvey_submit0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_GMSURVEY_SUBMIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_GMSURVEY_SUBMIT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 71] = [ 0x00, 0x45, 0x2A, 0x03, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01,
         0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01,
         0x00, 0x05, 0x00, 0x00, 0x00, 0x01, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01,
         0x00, 0x07, 0x00, 0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01,
         0x00, 0x09, 0x00, 0x00, 0x00, 0x01, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x01,
         0x00, 0x00, ];

    pub(crate) fn expected1() -> CMSG_GMSURVEY_SUBMIT {
        CMSG_GMSURVEY_SUBMIT {
            survey_id: 0x1,
            questions: vec![
                GmSurveyQuestion {
                    question_id: 0x1,
                    answer: 0x1,
                    comment: String::from(""),
                },
                GmSurveyQuestion {
                    question_id: 0x2,
                    answer: 0x1,
                    comment: String::from(""),
                },
                GmSurveyQuestion {
                    question_id: 0x3,
                    answer: 0x1,
                    comment: String::from(""),
                },
                GmSurveyQuestion {
                    question_id: 0x4,
                    answer: 0x1,
                    comment: String::from(""),
                },
                GmSurveyQuestion {
                    question_id: 0x5,
                    answer: 0x1,
                    comment: String::from(""),
                },
                GmSurveyQuestion {
                    question_id: 0x6,
                    answer: 0x1,
                    comment: String::from(""),
                },
                GmSurveyQuestion {
                    question_id: 0x7,
                    answer: 0x1,
                    comment: String::from(""),
                },
                GmSurveyQuestion {
                    question_id: 0x8,
                    answer: 0x1,
                    comment: String::from(""),
                },
                GmSurveyQuestion {
                    question_id: 0x9,
                    answer: 0x1,
                    comment: String::from(""),
                },
                GmSurveyQuestion {
                    question_id: 0xA,
                    answer: 0x1,
                    comment: String::from(""),
                },
            ],
            answer_comment: String::from(""),
        }

    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm` line 64.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_gmsurvey_submit1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_GMSURVEY_SUBMIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_GMSURVEY_SUBMIT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm` line 64.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_gmsurvey_submit1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_GMSURVEY_SUBMIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_GMSURVEY_SUBMIT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm` line 64.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_gmsurvey_submit1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_GMSURVEY_SUBMIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_GMSURVEY_SUBMIT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}


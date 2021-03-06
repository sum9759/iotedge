use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{MessageTestResult, TestType};

#[derive(Debug, Serialize, PartialEq)]
pub(crate) struct TestOperationResultDto {
    #[serde(rename = "source")]
    source: String,
    #[serde(rename = "result")]
    result: MessageTestResult,
    #[serde(rename = "type")]
    test_type: u8,
    #[serde(rename = "createdAt")]
    created_at: String,
}

impl TestOperationResultDto {
    pub fn new(
        source: String,
        result: MessageTestResult,
        test_type: TestType,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            source,
            result,
            test_type: test_type as u8,
            created_at: created_at.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::{TestOperationResultDto, TestType};
    use crate::MessageTestResult;

    #[test]
    fn serialize() {
        let tracking_id = "tracking".to_string();
        let batch_id = "batch".to_string();
        let seq_num = 2;
        let test_result = MessageTestResult::new(tracking_id, batch_id, seq_num);

        let source = "source".to_string();
        let _type = TestType::Messages;
        let created_at = Utc::now();
        let test_result_dto =
            TestOperationResultDto::new(source, test_result, _type.clone(), created_at.clone());

        let expected = format!("{{\"source\":\"source\",\"result\":\"tracking;batch;2\",\"type\":2,\"createdAt\":\"{}\"}}", created_at);
        let serialized = serde_json::to_string(&test_result_dto).unwrap();

        assert_eq!(expected, serialized);
    }
}

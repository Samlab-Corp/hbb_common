use hbb_common::{message_proto::*,protobuf::{self,Message as _}};
fn session()->NativeManagementSession{NativeManagementSession{management_version:1,operation_id:"aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa".into(),command_hash:"a".repeat(64),timeout_seconds:60,output_limit_bytes:1024,..Default::default()}}
#[test]fn management_marker_preserves_existing_terminal_login_oneof(){
    let mut login=LoginRequest::new();login.set_terminal(Terminal{native_management:Some(session()).into(),..Default::default()});
    let wire=login.write_to_bytes().unwrap();assert_eq!(&wire[..2],&[0x82,0x01]);
    let parsed=LoginRequest::parse_from_bytes(&wire).unwrap();
    match parsed.union.unwrap(){login_request::Union::Terminal(t)=>{assert!(t.service_id.is_empty());assert_eq!(t.native_management.as_ref().unwrap().management_version,1)},_=>panic!("terminal scope changed")}
}
#[test]fn old_terminal_schema_ignores_marker_but_never_receives_a_command(){
    let wire=Terminal{native_management:Some(session()).into(),..Default::default()}.write_to_bytes().unwrap();
    let mut legacy=protobuf::CodedInputStream::from_bytes(&wire);let mut service=String::new();
    while let Some(tag)=legacy.read_raw_tag_or_eof().unwrap(){match tag{10=>service=legacy.read_string().unwrap(),18=>legacy.skip_field(protobuf::rt::WireType::LengthDelimited).unwrap(),_=>panic!("unexpected legacy terminal field")}}
    assert!(service.is_empty()); // No OpenTerminal, input or command is embedded in login.
}
#[test]fn execute_challenge_and_result_have_distinct_unused_top_level_tags(){
    let mut message=Message::new();message.set_native_management_execute(NativeManagementExecute{management_version:1,..Default::default()});
    assert_eq!(&message.write_to_bytes().unwrap()[..2],&[0x8a,0x02]);
    message.set_native_management_challenge(NativeManagementChallenge{management_version:1,..Default::default()});
    assert_eq!(&message.write_to_bytes().unwrap()[..2],&[0x92,0x02]);
    message.set_native_management_result(NativeManagementResult{management_version:1,..Default::default()});
    assert_eq!(&message.write_to_bytes().unwrap()[..2],&[0x9a,0x02]);
}
#[test]fn result_preserves_binary_output_and_unconfirmed_exit(){
    let value=NativeManagementResult{management_version:1,operation_id:session().operation_id,status:"unconfirmed".into(),reason:"management_timeout_unconfirmed".into(),stdout:vec![0,255,1].into(),executor_is_system:true,..Default::default()};
    let parsed=NativeManagementResult::parse_from_bytes(&value.write_to_bytes().unwrap()).unwrap();assert_eq!(parsed.stdout.as_ref(),&[0,255,1]);assert!(!parsed.exit_code_confirmed);
}

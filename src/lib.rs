use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Default, Serialize, Deserialize)]
pub struct Registry { pub artifacts: BTreeMap<String, Artifact> }
#[derive(Clone, Serialize, Deserialize)]
pub struct Artifact { pub id: String, pub name: String, pub folder: String, pub project: Option<String>, pub conversation: String, pub revision: u64, pub location: String }
impl Registry {
    pub fn publish(&mut self, next: Artifact) -> Result<Option<String>, &'static str> {
        if next.id.is_empty() || next.conversation.is_empty() || next.location.is_empty() { return Err("missing identity or location"); }
        if let Some(old) = self.artifacts.get(&next.id) {
            if next.revision <= old.revision { return Err("stale revision"); }
            if old.conversation != next.conversation { return Err("ownership mismatch"); }
        }
        Ok(self.artifacts.insert(next.id.clone(), next).map(|a| a.location))
    }
}
#[derive(Default)]
pub struct Stream { pub text: String, pub next_sequence: u64, pub finished: bool }
impl Stream {
    pub fn append(&mut self, sequence: u64, delta: &str) -> Result<(), &'static str> {
        if self.finished || sequence != self.next_sequence { return Err("invalid stream event"); }
        self.text.push_str(delta); self.next_sequence += 1; Ok(())
    }
}
#[cfg(target_os="android")]
#[no_mangle]
pub extern "system" fn Java_com_visualsec_synapt_MainActivity_nativeVersion(env: jni::JNIEnv, _: jni::objects::JObject) -> jni::sys::jstring {
    env.new_string(env!("CARGO_PKG_VERSION")).map(|s| s.into_raw()).unwrap_or(std::ptr::null_mut())
}
#[cfg(test)]
mod tests {
 use super::*;
 fn artifact(revision:u64)->Artifact { Artifact{id:"a".into(),name:"project.zip".into(),folder:"Development".into(),project:Some("SynapT".into()),conversation:"c".into(),revision,location:format!("file-{revision}")} }
 #[test] fn latest_only(){let mut r=Registry::default();assert_eq!(r.publish(artifact(1)),Ok(None));assert_eq!(r.publish(artifact(2)),Ok(Some("file-1".into())));assert!(r.publish(artifact(1)).is_err());assert_eq!(r.artifacts.len(),1);}
 #[test] fn stream_order(){let mut s=Stream::default();assert!(s.append(1,"bad").is_err());s.append(0,"Hello ").unwrap();s.append(1,"world").unwrap();assert_eq!(s.text,"Hello world");s.finished=true;assert!(s.append(2,"!").is_err());}
}

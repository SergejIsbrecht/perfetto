// Copyright (C) 2025 Rivos Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Manually generated with bindings for an extra set of TracePacket fields.

use crate::pb_msg;
use crate::pb_msg_ext;
use crate::protos::trace::android::android_log::AndroidLogPacket;

use perfetto_sdk::protos::trace::trace_packet::TracePacket;

pb_msg_ext!(TracePacket {
    // https://cs.android.com/android/platform/superproject/+/android-latest-release:device/google/trout/tools/tracing/proto/perfetto_trace.proto;l=8934?q=AndroidLogPacket
    android_log_packet: AndroidLogPacket, msg, 39,
});

/// Import this to use the extra `TracePacket` fields.
pub mod prelude {
    pub use super::TracePacketExt;
}

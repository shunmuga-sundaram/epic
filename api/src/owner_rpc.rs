// Copyright 2020 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! JSON-RPC Stub generation for the Owner API

use crate::owner::Owner;
use crate::p2p::types::PeerInfoDisplay;
use crate::p2p::PeerData;
use crate::rest::ErrorKind;
use crate::types::Status;
use std::net::SocketAddr;

/// Public definition used to generate Node jsonrpc api.
/// * When running `epic` with defaults, the V2 api is available at
/// `localhost:3413/v2/owner`
/// * The endpoint only supports POST operations, with the json-rpc request as the body
#[easy_jsonrpc_mw::rpc]
pub trait OwnerRpc: Sync + Send {
	/**
	Networked version of [Owner::get_status](struct.Node.html#method.get_status).

	# Json rpc example

	```
	# epic_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "get_status",
		"params": [],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": {
			"protocol_version": "2",
			"user_agent": "MW/Epic 2.x.x",
			"connections": "8",
			"tip": {
				"height": 371553,
				"last_block_pushed": "00001d1623db988d7ed10c5b6319360a52f20c89b4710474145806ba0e8455ec",
				"prev_block_to_last": "0000029f51bacee81c49a27b4bc9c6c446e03183867c922890f90bb17108d89f",
				"total_difficulty": 1127628411943045
			},
			"sync_status": "header_sync",
			"sync_info": {
				"current_height": 371553,
				"highest_height": 0
			}
			}
		}
	}
	# "#
	# );
	```
	 */
	fn get_status(&self) -> Result<Status, ErrorKind>;

	/**
	Networked version of [Owner::validate_chain](struct.Node.html#method.validate_chain).

	# Json rpc example

	```
	# epic_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "validate_chain",
		"params": [],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": null
		}
	}
	# "#
	# );
	```
	 */
	fn validate_chain(&self) -> Result<(), ErrorKind>;

	/**
	Networked version of [Owner::compact_chain](struct.Node.html#method.compact_chain).

	# Json rpc example

	```
	# epic_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "compact_chain",
		"params": [],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": null
		}
	}
	# "#
	# );
	```
	 */
	fn compact_chain(&self) -> Result<(), ErrorKind>;

	/**
	Networked version of [Owner::get_peers](struct.Node.html#method.get_peers).

	# Json rpc example

	```
	# epic_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "get_peers",
		"params": ["70.50.33.130:3414"],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": [
			{
				"addr": "70.50.33.130:3414",
				"ban_reason": "None",
				"capabilities": {
				"bits": 15
				},
				"flags": "Defunct",
				"last_banned": 0,
				"last_connected": 1570129317,
				"user_agent": "MW/Epic 2.0.0"
			}
			]
		}
	}
	# "#
	# );
	```
	 */
	fn get_peers(&self, peer_addr: Option<SocketAddr>) -> Result<Vec<PeerData>, ErrorKind>;

	/**
	Networked version of [Owner::get_connected_peers](struct.Node.html#method.get_connected_peers).

	# Json rpc example

	```
	# epic_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "get_connected_peers",
		"params": [],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": [
			{
				"addr": "35.176.195.242:3414",
				"capabilities": {
				"bits": 15
				},
				"direction": "Outbound",
				"height": 374510,
				"total_difficulty": 1133954621205750,
				"user_agent": "MW/Epic 2.0.0",
				"version": 1
			},
			{
				"addr": "47.97.198.21:3414",
				"capabilities": {
				"bits": 15
				},
				"direction": "Outbound",
				"height": 374510,
				"total_difficulty": 1133954621205750,
				"user_agent": "MW/Epic 2.0.0",
				"version": 1
			},
			{
				"addr": "148.251.16.13:3414",
				"capabilities": {
				"bits": 15
				},
				"direction": "Outbound",
				"height": 374510,
				"total_difficulty": 1133954621205750,
				"user_agent": "MW/Epic 2.0.0",
				"version": 1
			},
			{
				"addr": "68.195.18.155:3414",
				"capabilities": {
				"bits": 15
				},
				"direction": "Outbound",
				"height": 374510,
				"total_difficulty": 1133954621205750,
				"user_agent": "MW/Epic 2.0.0",
				"version": 1
			},
			{
				"addr": "52.53.221.15:3414",
				"capabilities": {
				"bits": 15
				},
				"direction": "Outbound",
				"height": 0,
				"total_difficulty": 1133954621205750,
				"user_agent": "MW/Epic 2.0.0",
				"version": 1
			},
			{
				"addr": "109.74.202.16:3414",
				"capabilities": {
				"bits": 15
				},
				"direction": "Outbound",
				"height": 374510,
				"total_difficulty": 1133954621205750,
				"user_agent": "MW/Epic 2.0.0",
				"version": 1
			},
			{
				"addr": "121.43.183.180:3414",
				"capabilities": {
				"bits": 15
				},
				"direction": "Outbound",
				"height": 374510,
				"total_difficulty": 1133954621205750,
				"user_agent": "MW/Epic 2.0.0",
				"version": 1
			},
			{
				"addr": "35.157.247.209:23414",
				"capabilities": {
				"bits": 15
				},
				"direction": "Outbound",
				"height": 374510,
				"total_difficulty": 1133954621205750,
				"user_agent": "MW/Epic 2.0.0",
				"version": 1
			}
			]
		}
	}
	# "#
	# );
	```
	 */
	fn get_connected_peers(&self) -> Result<Vec<PeerInfoDisplay>, ErrorKind>;

	/**
	Networked version of [Owner::ban_peer](struct.Node.html#method.ban_peer).

	# Json rpc example

	```
	# epic_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "ban_peer",
		"params": ["70.50.33.130:3414"],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": null
		}
	}
	# "#
	# );
	```
	 */
	fn ban_peer(&self, peer_addr: SocketAddr) -> Result<(), ErrorKind>;

	/**
	Networked version of [Owner::unban_peer](struct.Node.html#method.unban_peer).

	# Json rpc example

	```
	# epic_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "unban_peer",
		"params": ["70.50.33.130:3414"],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": null
		}
	}
	# "#
	# );
	```
	 */
	fn unban_peer(&self, peer_addr: SocketAddr) -> Result<(), ErrorKind>;
}

impl OwnerRpc for Owner {
	fn get_status(&self) -> Result<Status, ErrorKind> {
		Owner::get_status(self).map_err(|e| e.kind().clone())
	}

	fn validate_chain(&self) -> Result<(), ErrorKind> {
		Owner::validate_chain(self).map_err(|e| e.kind().clone())
	}

	fn compact_chain(&self) -> Result<(), ErrorKind> {
		Owner::compact_chain(self).map_err(|e| e.kind().clone())
	}

	fn get_peers(&self, addr: Option<SocketAddr>) -> Result<Vec<PeerData>, ErrorKind> {
		Owner::get_peers(self, addr).map_err(|e| e.kind().clone())
	}

	fn get_connected_peers(&self) -> Result<Vec<PeerInfoDisplay>, ErrorKind> {
		Owner::get_connected_peers(self).map_err(|e| e.kind().clone())
	}

	fn ban_peer(&self, addr: SocketAddr) -> Result<(), ErrorKind> {
		Owner::ban_peer(self, addr).map_err(|e| e.kind().clone())
	}

	fn unban_peer(&self, addr: SocketAddr) -> Result<(), ErrorKind> {
		Owner::unban_peer(self, addr).map_err(|e| e.kind().clone())
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! doctest_helper_json_rpc_owner_assert_response {
	($request:expr, $expected_response:expr) => {
		// create temporary epic server, run jsonrpc request on node api, delete server, return
		// json response.

		{
			/*use epic_servers::test_framework::framework::run_doctest;
			use epic_util as util;
			use serde_json;
			use serde_json::Value;
			use tempfile::tempdir;

			let dir = tempdir().map_err(|e| format!("{:#?}", e)).unwrap();
			let dir = dir
				.path()
				.to_str()
				.ok_or("Failed to convert tmpdir path to string.".to_owned())
				.unwrap();

			let request_val: Value = serde_json::from_str($request).unwrap();
			let expected_response: Value = serde_json::from_str($expected_response).unwrap();
			let response = run_doctest(
				request_val,
				dir,
				$use_token,
				$blocks_to_mine,
				$perform_tx,
				$lock_tx,
				$finalize_tx,
					)
			.unwrap()
			.unwrap();
			if response != expected_response {
				panic!(
					"(left != right) \nleft: {}\nright: {}",
					serde_json::to_string_pretty(&response).unwrap(),
					serde_json::to_string_pretty(&expected_response).unwrap()
				);
				}*/
		}
	};
}

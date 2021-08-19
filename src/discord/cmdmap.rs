//! This module contains code for turning JSON command descriptions into bot code

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CmdMap {
    pub nologic: Vec<NoLogicCmd>,
    pub logic: Vec<LogicCmd>,
}

/// NoLogicCmd defines a command where the response never changes, and has no logic to it
/// This is to be used for help commands, and meme commands that just return a URL
#[derive(Serialize, Deserialize, Debug)]
pub struct NoLogicCmd {
    pub name: String,
    pub description: String,
    pub response: String,
    pub scope: Option<Vec<u64>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogicCmd {
    pub name: String,
    pub description: String,
    pub scope: Option<Vec<u64>>,
    pub args: Vec<CmdArg>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CmdArg {
    pub name: String,
    pub description: String,
    pub required: bool,
    #[serde(rename = "type")]
    pub kind: String
}
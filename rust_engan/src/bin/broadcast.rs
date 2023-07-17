use rust_engan::*;

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};
use ulid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Broadcast { message: usize},
    BroadcastOk {

    }
    Read,
    ReadOk {
        messages: Vec<usize>,
    },
    Topology {
        Topology: HashMap<String, Vec<String>>,

    },
    TopologyOk，
}

struct BroadcastNode {
    id: usize,
    node: String,
    messages: Vec<usize>
}

impl Node<(), Payload> for BroadcastNode {
    fn from_init(_state: (), init: rust_engan::Init) -> anyhow::Result<Self> {
        Ok(Self {
            id: 1,
            node: init.node_id,
            messages: Vec::new(),
        })
    }

    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Broadcast {message} => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::BroadcastOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize repsonse to broadcast")?;

                output.write_all(b"\n").context("write trailing new line")?;

                self.id += 1;
            }
            Payload::GenerateOk { .. } => (),
            // Payload::InitOk { .. } => bail!("Received init_ok message"),
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<_, BroadcastNode, _>(())
}

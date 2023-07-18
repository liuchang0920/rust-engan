use rust_engan::*;

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};
use ulid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Generate,
    GenerateOk {
        #[serde(rename = "id")]
        guid: String,
    },
}

struct UniqueNode {
    id: usize,
    node: String,
}

impl Node<(), Payload> for UniqueNode {
    fn from_init(
        _state: (),
        init: rust_engan::Init,
        _tx: std::sync::mpsc::Sender<Event<Payload>>,
    ) -> anyhow::Result<Self> {
        Ok(UniqueNode {
            id: 1,
            node: init.node_id,
        })
    }

    fn step(&mut self, input: Event<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        let Event::Message(input) = input else {
            panic!("got injected event when there's no event injection");
        };

        let mut reply = input.into_reply(Some(&mut self.id));
        match reply.body.payload {
            Payload::Generate => {
                // let guid = ulid::Ulid::new().to_string();
                let guid = format!("{}-{}", self.node, self.id);

                reply.body.payload = Payload::GenerateOk { guid };

                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize repsonse to init")?;

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
    main_loop::<_, UniqueNode, _, _>(())
}

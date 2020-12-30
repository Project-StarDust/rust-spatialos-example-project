use spatialos::worker::{
    connection::{
        ConnectionFuture, ConnectionParameters, NetworkConnectionType, NetworkSecurityType,
    },
    constraint::{Constraint, EntityIdConstraint},
    log_message::LogMessage,
    op::{
        AddComponentOp, CommandRequestOp, ComponentUpdateOp, DisconnectOp, EntityQueryResponseOp,
        LogMessageOp, WorkerOp,
    },
    EntityQuery, LogLevel, ResultType,
};

use spatialos_sdk::Component;

pub mod generated;

use generated::{improbable::Position, sample::ClientData};

const WORKER_TYPE: &str = "client_vtable";

#[macro_use]
extern crate spatialos_macro;

pub fn on_log_message(op: &LogMessageOp) {
    println!("log: {}", op.message)
}

pub fn on_disconnect(op: &DisconnectOp) {
    println!("disconnected. reason: {}", op.reason);
}

pub fn on_entity_query_response(op: &EntityQueryResponseOp) {
    println!(
        "entity query result: {} entities. Status: {:?}.",
        op.result_count, op.status_code
    );
    if !op.results.is_empty() {
        for index in 0..op.result_count {
            let entity = &op.results[index as usize];
            print!(
                "- entity {} with {} components: ",
                entity.entity_id, entity.component_count
            );
            for index2 in 0..entity.component_count {
                let component = &entity.components[index2 as usize];
                if component.component_id == Position::ID {
                    let position = unsafe {
                        Box::from_raw(component.user_handle as *mut <Position as Component>::Data)
                    };
                    print!("{:?}, ", *position);
                    Box::into_raw(position);
                }
                if component.component_id == ClientData::ID {
                    let client_data = unsafe {
                        Box::from_raw(component.user_handle as *mut <ClientData as Component>::Data)
                    };
                    print!("{:?}, ", *client_data);
                    Box::into_raw(client_data);
                }
            }
            println!("");
        }
    }
}

pub fn on_add_component(op: &AddComponentOp) {
    println!(
        "received add component op (entity: {}, component: {})",
        op.entity_id, op.data.component_id
    );

    if op.data.component_id == Position::ID {
        let position =
            unsafe { Box::from_raw(op.data.user_handle as *mut <Position as Component>::Data) };
        println!(
            "received improbable.Position initial data ({}, {}, {})",
            position.coords.x, position.coords.y, position.coords.z
        );
        Box::into_raw(position);
    }
}

pub fn on_component_update(op: &ComponentUpdateOp) {
    println!(
        "received component update op (entity: {}, component: {})",
        op.entity_id, op.update.component_id
    );

    if op.update.component_id == Position::ID {
        let position =
            unsafe { Box::from_raw(op.update.user_handle as *mut <Position as Component>::Update) };
        if let Some(coords) = &position.coords {
            println!(
                "received improbable.Position update data ({}, {}, {})",
                coords.x, coords.y, coords.z
            );
        }
        Box::into_raw(position);
    }
}

pub fn on_command_request(op: &CommandRequestOp) {
    println!("CommandRequest: {:?}", op)
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        eprintln!(
            "Usage: {} <hostname> <port> <worker_id>",
            args.get(0).expect("Can't have program name")
        );
        eprintln!("    <hostname>    - hostname of the receptionist to connect to.");
        eprintln!("    <port>        - port to use.");
        eprintln!("    <worker_id>   - name of the worker assigned by SpatialOS.");
        std::process::exit(1);
    }

    let hostname = args.get(1).unwrap();
    let port = args
        .get(2)
        .unwrap()
        .parse::<u16>()
        .expect("Can't parse port to u16");
    let worker_id = args.get(3).unwrap();

    let mut vtables = vec![
        Position::get_vtable().into(),
        ClientData::get_vtable().into(),
    ];
    vtables.shrink_to_fit();

    let parameters = {
        let mut parameters = ConnectionParameters::default();
        parameters.network.connection_type = NetworkConnectionType::ModularKcp;
        parameters.network.modular_kcp.security_type = NetworkSecurityType::Insecure;
        parameters.worker_type = WORKER_TYPE.to_string();
        parameters.network.tcp.multiplex_level = 4;
        parameters.default_component_vtable = std::ptr::null();
        parameters.component_vtable_count = vtables.len() as u32;
        parameters.component_vtables = vtables.as_mut_ptr();
        parameters
    };
    let mut connection = {
        let mut connection = ConnectionFuture::connect_async(hostname, port, worker_id, parameters);
        connection.get(None).expect("Timed out")
    };
    connection.send_log_message(LogMessage::new(
        LogLevel::Warn,
        WORKER_TYPE,
        "Connected successfully",
        None,
    ));

    let query = {
        let mut query = EntityQuery::default();
        query.constraint = Constraint::EntityId(EntityIdConstraint { entity_id: 1 });
        query.result_type = ResultType::Snapshot;
        query.snapshot_result_type_component_id_count = 2;
        query.snapshot_result_type_component_ids = vec![Position::ID, ClientData::ID];
        query
    };
    connection.send_entity_query_request(query, None);

    loop {
        let op_list = connection.get_op_list(0);
        for op in op_list.ops.iter() {
            match op {
                WorkerOp::Disconnect(disconnect) => on_disconnect(&disconnect),
                WorkerOp::LogMessage(log_message) => on_log_message(&log_message),
                WorkerOp::EntityQueryResponse(query_response) => {
                    on_entity_query_response(&query_response)
                }
                WorkerOp::AddComponent(add_component) => on_add_component(&add_component),
                WorkerOp::ComponentUpdate(component_update) => {
                    on_component_update(&component_update)
                }
                WorkerOp::CommandRequest(command_request) => on_command_request(&command_request),
                _ => {}
            };
        }
    }
}

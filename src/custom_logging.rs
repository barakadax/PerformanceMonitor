pub fn init_logging() {
    use json_subscriber::{
        JsonLayer,
        fmt::{Layer, layer},
    };
    use std::process::id;
    use tracing_subscriber::{layer::SubscriberExt, registry, util::SubscriberInitExt};

    let mut json_layer: Layer = layer()
        .with_level(true)
        .with_line_number(true)
        .with_target(false)
        .flatten_event(true);

    let inner_layer: &mut JsonLayer = json_layer.inner_layer_mut();

    inner_layer.with_thread_ids("thread_id");
    inner_layer.with_file("file_name");

    inner_layer.add_static_field("app", "performance monitor".into());
    inner_layer.add_static_field("logger", "tracing".into());

    inner_layer.add_dynamic_field("pid", |_, _| Some(id()));

    registry().with(json_layer).init();
}

use crate::chobit_module::chobit_module;
use crate::chobit_module::ChobitModule;

struct TestObject {
    pub value: i32
}

chobit_module! {
    input_buffer_size = 100;
    output_buffer_size = 200;

    on_created = (): TestObject => TestObject {
        value: 3
    };

    on_received = (module) => {
        let a = 3;
        assert_eq!(module.user_object().value, a);
    };
}

use anyhow::Result;
use wasmtime::{
    Engine,
    Store,
    component::{
        Component,
        ResourceTable,
        Linker,
        Instance
    }
};
use wasmtime_wasi::{
    WasiView,
    WasiCtxBuilder,
    WasiCtx
};

fn main() {
    // Setup store and linker
    let engine = Engine::default();
    let resource_table = ResourceTable::new();
    let wasi_ctx = WasiCtxBuilder::new().inherit_stdout().build();
    let state = GuestState { resource_table, wasi_ctx };    
    let mut store = Store::new(&engine, state);    
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();
    
    println!("Running guest-lib");
    let component = Component::from_file(&engine, "../target/wasm32-wasip1/release/guest_lib.wasm").unwrap();
    let instance = linker.instantiate(&mut store, &component).unwrap();
    run_guest(&mut store, instance).unwrap();

    println!();

    println!("Running guest-main");
    let component = Component::from_file(&engine, "../target/wasm32-wasip1/release/guest_main.wasm").unwrap();
    let instance = linker.instantiate(&mut store, &component).unwrap();
    run_guest(&mut store, instance).unwrap();   
}

fn run_guest(store: &mut Store<GuestState>, instance: Instance) -> Result<()> {
    // Get run function
    let run_interface_idx = instance.get_export(&mut *store, None, "wasi:cli/run@0.2.0").unwrap();
    let run_func_idx = instance.get_export(&mut *store, Some(&run_interface_idx), "run").unwrap();
    let run_func = instance.get_func(&mut *store, run_func_idx).unwrap();
    let run_func = run_func.typed::<(), (Result<(), ()>,)>(&mut *store).unwrap();

    // Call run function
    println!("Calling run function for the first time");
    run_func.call(&mut *store, ()).unwrap().0.unwrap();
    run_func.post_return(&mut *store).unwrap();
    
    // Call run function again
    println!("Calling run function for the second time");
    run_func.call(&mut *store, ())?.0.unwrap();
    run_func.post_return(&mut *store)?;

    Ok(())
}

struct GuestState {
    resource_table: ResourceTable,
    wasi_ctx: WasiCtx,
}

impl WasiView for GuestState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}
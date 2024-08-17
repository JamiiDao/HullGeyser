use log::info;
use solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;
use solana_program::clock::Slot;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = HullGeyser::default();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}

#[derive(Debug, Default)]
pub struct HullGeyser;

impl GeyserPlugin for HullGeyser {
    fn name(&self) -> &'static str {
        "HULL-GEYSER"
    }

    fn setup_logger(
        &self,
        logger: &'static dyn log::Log,
        level: log::LevelFilter,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        println!("SETUP_COMPLETE");

        Ok(())
    }

    fn on_load(
        &mut self,
        config_file: &str,
        is_reload: bool,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        solana_logger::setup_with_default("info");
        info!(
            "Loading plugin {:?} from config_file {:?}",
            self.name(),
            config_file
        );

        Ok(())
    }

    fn on_unload(&mut self) {}

    fn update_account(
        &self,
        account: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaAccountInfoVersions,
        slot: Slot,
        is_startup: bool,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        Ok(())
    }

    fn notify_end_of_startup(
        &self,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        Ok(())
    }

    fn update_slot_status(
        &self,
        slot: Slot,
        parent: Option<u64>,
        status: solana_geyser_plugin_interface::geyser_plugin_interface::SlotStatus,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        Ok(())
    }

    fn notify_transaction(
        &self,
        transaction: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfoVersions,
        slot: Slot,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        Ok(())
    }

    fn notify_entry(
        &self,
        entry: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaEntryInfoVersions,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        Ok(())
    }

    fn notify_block_metadata(
        &self,
        blockinfo: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaBlockInfoVersions,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        false
    }

    fn entry_notifications_enabled(&self) -> bool {
        false
    }
}

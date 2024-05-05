mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::prelude::*;
use substreams::store;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use pb::sf::substreams::index::v1::Keys;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

substreams_ethereum::init!();

const FACTORY_TRACKED_CONTRACT: [u8; 20] = hex!("1f98431c8ad98523631ae4a59f267346ea31f984");

fn map_factory_events(blk: &eth::Block, events: &mut contract::Events) {
    events.factory_fee_amount_enableds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == FACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::factory_contract::events::FeeAmountEnabled::match_and_decode(log) {
                        return Some(contract::FactoryFeeAmountEnabled {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            fee: event.fee.to_u64(),
                            tick_spacing: Into::<num_bigint::BigInt>::into(event.tick_spacing).to_i64().unwrap(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.factory_owner_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == FACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::factory_contract::events::OwnerChanged::match_and_decode(log) {
                        return Some(contract::FactoryOwnerChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            old_owner: event.old_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.factory_pool_createds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == FACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::factory_contract::events::PoolCreated::match_and_decode(log) {
                        return Some(contract::FactoryPoolCreated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            fee: event.fee.to_u64(),
                            pool: event.pool,
                            tick_spacing: Into::<num_bigint::BigInt>::into(event.tick_spacing).to_i64().unwrap(),
                            token0: event.token0,
                            token1: event.token1,
                        });
                    }

                    None
                })
        })
        .collect());
}

fn is_declared_dds_address(addr: &Vec<u8>, ordinal: u64, dds_store: &store::StoreGetInt64) -> bool {
    //    substreams::log::info!("Checking if address {} is declared dds address", Hex(addr).to_string());
    if dds_store.get_at(ordinal, Hex(addr).to_string()).is_some() {
        return true;
    }
    return false;
}

fn map_pool_events(
    blk: &eth::Block,
    dds_store: &store::StoreGetInt64,
    events: &mut contract::Events,
) {
    events.pool_burns.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::Burn::match_and_decode(log) {
                        return Some(contract::PoolBurn {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount: event.amount.to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            owner: event.owner,
                            tick_lower: Into::<num_bigint::BigInt>::into(event.tick_lower).to_i64().unwrap(),
                            tick_upper: Into::<num_bigint::BigInt>::into(event.tick_upper).to_i64().unwrap(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pool_collects.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::Collect::match_and_decode(log) {
                        return Some(contract::PoolCollect {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            owner: event.owner,
                            recipient: event.recipient,
                            tick_lower: Into::<num_bigint::BigInt>::into(event.tick_lower).to_i64().unwrap(),
                            tick_upper: Into::<num_bigint::BigInt>::into(event.tick_upper).to_i64().unwrap(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pool_collect_protocols.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::CollectProtocol::match_and_decode(log) {
                        return Some(contract::PoolCollectProtocol {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            recipient: event.recipient,
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());


    events.pool_flashes.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::Flash::match_and_decode(log) {
                        return Some(contract::PoolFlash {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            paid0: event.paid0.to_string(),
                            paid1: event.paid1.to_string(),
                            recipient: event.recipient,
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());

    events.pool_increase_observation_cardinality_nexts.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::IncreaseObservationCardinalityNext::match_and_decode(log) {
                        return Some(contract::PoolIncreaseObservationCardinalityNext {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            observation_cardinality_next_new: event.observation_cardinality_next_new.to_u64(),
                            observation_cardinality_next_old: event.observation_cardinality_next_old.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pool_initializes.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::Initialize::match_and_decode(log) {
                        return Some(contract::PoolInitialize {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            sqrt_price_x96: event.sqrt_price_x96.to_string(),
                            tick: Into::<num_bigint::BigInt>::into(event.tick).to_i64().unwrap(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pool_mints.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::Mint::match_and_decode(log) {
                        return Some(contract::PoolMint {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount: event.amount.to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            owner: event.owner,
                            sender: event.sender,
                            tick_lower: Into::<num_bigint::BigInt>::into(event.tick_lower).to_i64().unwrap(),
                            tick_upper: Into::<num_bigint::BigInt>::into(event.tick_upper).to_i64().unwrap(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pool_set_fee_protocols.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::SetFeeProtocol::match_and_decode(log) {
                        return Some(contract::PoolSetFeeProtocol {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            fee_protocol0_new: event.fee_protocol0_new.to_u64(),
                            fee_protocol0_old: event.fee_protocol0_old.to_u64(),
                            fee_protocol1_new: event.fee_protocol1_new.to_u64(),
                            fee_protocol1_old: event.fee_protocol1_old.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pool_swaps.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::Swap::match_and_decode(log) {
                        return Some(contract::PoolSwap {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            liquidity: event.liquidity.to_string(),
                            recipient: event.recipient,
                            sender: event.sender,
                            sqrt_price_x96: event.sqrt_price_x96.to_string(),
                            tick: Into::<num_bigint::BigInt>::into(event.tick).to_i64().unwrap(),
                        });
                    }

                    None
                })
        })
        .collect());
}

#[substreams::handlers::store]
fn store_factory_pool_created(blk: eth::Block, store: StoreSetInt64) {
    for rcpt in blk.receipts() {
        for log in rcpt
            .receipt
            .logs
            .iter()
            .filter(|log| log.address == FACTORY_TRACKED_CONTRACT)
        {
            if let Some(event) = abi::factory_contract::events::PoolCreated::match_and_decode(log) {
                store.set(log.ordinal, Hex(event.pool).to_string(), &1);
            }
        }
    }
}

#[substreams::handlers::map]
fn index_pool_events(blk: eth::Block, dds_store: store::StoreGetInt64) -> Result<Keys, substreams::errors::Error> {
        let mut keys = Keys::default();
        let mut has_uniswap = false;

        for log in blk.logs() {
            let address_vec = log.address().to_vec();

            if is_declared_dds_address(&address_vec, log.ordinal(), &dds_store) {
                has_uniswap = true;
                let pool_address = format!("pool_address:{}", Hex::encode(log.address()));
                    keys.keys.push(pool_address); 
            }

        }   

        if has_uniswap {
            keys.keys.push("uniswapv3".to_string());
        } 
        
        Ok(keys)
}


#[substreams::handlers::map]
fn map_events(
    blk: eth::Block,
    store_pool: StoreGetInt64,
) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_factory_events(&blk, &mut events);
    map_pool_events(&blk, &store_pool, &mut events);
    Ok(events)
}


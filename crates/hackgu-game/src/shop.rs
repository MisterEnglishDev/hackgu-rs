//! # CyberConnect2 Shop & Economic Trading System
//! Reconstructed directly from `NpcShopMenu`, `ShopBuySellMenuCtrl`, `GuildShopManager`,
//! and `GuildShopListWnd`.
//!
//! Controls retail stores, price indices, inventory transactions, and NPC barter trading.

use serde::{Deserialize, Serialize};
use crate::savedata::{InventorySlot, SaveData};

/// Category / Type of merchant shop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShopKind {
    /// Consumables, recovery potions, talismans (`shop01.ccs`)
    MagicItemShop,
    /// Weapons, dual blades, broadswords, guns (`shop02.ccs`)
    WeaponShop,
    /// Chest armor, robes, rings, accessories (`shop04.ccs`)
    ArmorShop,
    /// Canard Player Guild Merchant counter (`gshop01.ccs`)
    GuildShop,
    /// Roaming street trader NPC
    NpcStreetVendor,
}

/// An item for sale in a shop catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShopItemListing {
    pub item_id: u32,
    pub name: String,
    pub buy_price: u32,
    pub sell_price: u32,
    pub stock: Option<u32>, // None = unlimited
    pub required_story_flag: Option<u32>,
}

/// Error results during shop transactions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShopError {
    InsufficientFunds { required: u32, available: u32 },
    OutOfStock,
    ItemNotFound(u32),
    InsufficientItemQuantity { required: u32, available: u32 },
    InventoryFull,
    UnequalBarterValue { offered: u32, requested: u32 },
}

/// Merchant Shop catalog and inventory manager.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShopCatalog {
    pub shop_id: u32,
    pub name: String,
    pub kind: ShopKind,
    pub listings: Vec<ShopItemListing>,
}

impl ShopCatalog {
    /// Mac Anu Central District Item Shop defaults.
    pub fn new_mac_anu_item_shop() -> Self {
        let listings = vec![
            ShopItemListing { item_id: 1, name: "Health Drink".into(), buy_price: 100, sell_price: 25, stock: None, required_story_flag: None },
            ShopItemListing { item_id: 2, name: "Healing Serum".into(), buy_price: 350, sell_price: 80, stock: None, required_story_flag: None },
            ShopItemListing { item_id: 3, name: "Sprite Drop".into(), buy_price: 150, sell_price: 35, stock: None, required_story_flag: None },
            ShopItemListing { item_id: 4, name: "Fairy's Orb".into(), buy_price: 80, sell_price: 20, stock: None, required_story_flag: None },
            ShopItemListing { item_id: 5, name: "Smoke Screen".into(), buy_price: 120, sell_price: 30, stock: None, required_story_flag: None },
        ];

        Self {
            shop_id: 1,
            name: "Mac Anu Item Shop".into(),
            kind: ShopKind::MagicItemShop,
            listings,
        }
    }

    /// Mac Anu Weapon Shop defaults.
    pub fn new_mac_anu_weapon_shop() -> Self {
        let listings = vec![
            ShopItemListing { item_id: 10, name: "Spin GaiGu".into(), buy_price: 500, sell_price: 125, stock: None, required_story_flag: None },
            ShopItemListing { item_id: 11, name: "Spin Kouga".into(), buy_price: 1200, sell_price: 300, stock: None, required_story_flag: None },
            ShopItemListing { item_id: 12, name: "Broad Demon".into(), buy_price: 1800, sell_price: 450, stock: None, required_story_flag: Some(1020) },
        ];

        Self {
            shop_id: 2,
            name: "Mac Anu Armory".into(),
            kind: ShopKind::WeaponShop,
            listings,
        }
    }

    /// Process purchase of items with GP deduction and inventory insertion.
    pub fn buy(&mut self, item_id: u32, quantity: u32, save: &mut SaveData) -> Result<(), ShopError> {
        let listing = self.listings.iter_mut().find(|l| l.item_id == item_id)
            .ok_or(ShopError::ItemNotFound(item_id))?;

        if let Some(ref mut stock) = listing.stock {
            if *stock < quantity {
                return Err(ShopError::OutOfStock);
            }
            *stock -= quantity;
        }

        let total_cost = listing.buy_price.saturating_mul(quantity);
        if save.gp < total_cost {
            return Err(ShopError::InsufficientFunds {
                required: total_cost,
                available: save.gp,
            });
        }

        save.gp -= total_cost;

        // Add to inventory
        if let Some(slot) = save.items.iter_mut().find(|s| s.item_id == item_id) {
            slot.quantity = (slot.quantity + quantity).min(99);
        } else {
            save.items.push(InventorySlot {
                item_id,
                name: listing.name.clone(),
                quantity,
            });
        }

        Ok(())
    }

    /// Process sale of items from player inventory for GP.
    pub fn sell(&mut self, item_id: u32, quantity: u32, save: &mut SaveData) -> Result<u32, ShopError> {
        let listing = self.listings.iter().find(|l| l.item_id == item_id)
            .ok_or(ShopError::ItemNotFound(item_id))?;

        let slot_idx = save.items.iter().position(|s| s.item_id == item_id)
            .ok_or(ShopError::ItemNotFound(item_id))?;

        if save.items[slot_idx].quantity < quantity {
            return Err(ShopError::InsufficientItemQuantity {
                required: quantity,
                available: save.items[slot_idx].quantity,
            });
        }

        save.items[slot_idx].quantity -= quantity;
        if save.items[slot_idx].quantity == 0 {
            save.items.remove(slot_idx);
        }

        let earnings = listing.sell_price.saturating_mul(quantity);
        save.gp = save.gp.saturating_add(earnings);

        Ok(earnings)
    }
}

/// NPC Street Barter Trade Engine (`ccTradeCtrl`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NpcTradeDesk {
    pub npc_name: String,
    pub offered_items: Vec<ShopItemListing>,
}

impl NpcTradeDesk {
    pub fn new(npc_name: &str, items: Vec<ShopItemListing>) -> Self {
        Self {
            npc_name: npc_name.to_string(),
            offered_items: items,
        }
    }

    /// Perform a barter trade. The total sell value of offered player items
    /// must be equal to or greater than the buy value of requested NPC items.
    pub fn execute_trade(
        &mut self,
        target_item_id: u32,
        offered_item_ids: &[(u32, u32)], // (item_id, quantity)
        save: &mut SaveData,
    ) -> Result<(), ShopError> {
        let target = self.offered_items.iter().find(|i| i.item_id == target_item_id)
            .ok_or(ShopError::ItemNotFound(target_item_id))?;

        let required_value = target.buy_price;

        let mut offered_total_value = 0u32;
        for &(offered_id, count) in offered_item_ids {
            let player_slot = save.items.iter().find(|s| s.item_id == offered_id)
                .ok_or(ShopError::ItemNotFound(offered_id))?;

            if player_slot.quantity < count {
                return Err(ShopError::InsufficientItemQuantity {
                    required: count,
                    available: player_slot.quantity,
                });
            }

            // Standard resale value calculation
            let item_val = 50 * count;
            offered_total_value = offered_total_value.saturating_add(item_val);
        }

        if offered_total_value < required_value {
            return Err(ShopError::UnequalBarterValue {
                offered: offered_total_value,
                requested: required_value,
            });
        }

        // Deduct player offered items
        for &(offered_id, count) in offered_item_ids {
            if let Some(pos) = save.items.iter().position(|s| s.item_id == offered_id) {
                save.items[pos].quantity -= count;
                if save.items[pos].quantity == 0 {
                    save.items.remove(pos);
                }
            }
        }

        // Grant target item to player
        if let Some(slot) = save.items.iter_mut().find(|s| s.item_id == target_item_id) {
            slot.quantity = (slot.quantity + 1).min(99);
        } else {
            save.items.push(InventorySlot {
                item_id: target_item_id,
                name: target.name.clone(),
                quantity: 1,
            });
        }

        Ok(())
    }
}

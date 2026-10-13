//! # CyberConnect2 Equipment Alchemy & Customization Engine
//! Reconstructed directly from `ccCustomizeWnd` and `ItemIconListWndForCustomize`.
//!
//! Handles weapon/armor alchemy fusion (+1 to +5 level scaling), slot attachments,
//! elemental infusion crystals, and status-inflicting talismans.

use serde::{Deserialize, Serialize};
use crate::savedata::SaveData;
use crate::area::ElementKind;

/// Passive attribute installed into an equipment slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlotEffect {
    /// Infuse elemental damage/resistance
    ElementalAffinity(ElementKind),
    /// Probability of inflicting status ailment on hit
    StatusProc { ailment: String, chance_percent: u8 },
    /// Passive combat stat enhancement
    StatBoost { stat_name: String, amount: u32 },
    /// Life / SP leech on attack
    HpDrainPercent(u8),
    SpDrainPercent(u8),
}

/// An upgraded and customized weapon or armor piece.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomizedGear {
    pub base_item_id: u32,
    pub name: String,
    pub alchemy_level: u8, // +0 to +5 in Vol 1
    pub base_power: u32,
    pub max_slots: usize,
    pub installed_slots: Vec<SlotEffect>,
}

impl CustomizedGear {
    pub fn new(base_item_id: u32, name: &str, base_power: u32, max_slots: usize) -> Self {
        Self {
            base_item_id,
            name: name.to_string(),
            alchemy_level: 0,
            base_power,
            max_slots,
            installed_slots: Vec::with_capacity(max_slots),
        }
    }

    /// Calculate effective power with alchemy upgrade bonus.
    pub fn effective_power(&self) -> u32 {
        self.base_power + (self.alchemy_level as u32 * (self.base_power / 8).max(5))
    }

    /// Full display name including upgrade level (e.g. "Spin GaiGu +3").
    pub fn display_name(&self) -> String {
        if self.alchemy_level > 0 {
            format!("{} +{}", self.name, self.alchemy_level)
        } else {
            self.name.clone()
        }
    }
}

/// Errors during alchemy or socket customization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomizeError {
    ItemNotFound,
    MaxAlchemyLevelReached,
    IncompatibleItems,
    NoFreeSlotsAvailable,
    InsufficientCatalystQuantity,
}

/// The Alchemy & Customization Workbench Manager (`ccCustomizeWnd`).
#[derive(Debug, Clone, Default)]
pub struct CustomizeWorkbench;

impl CustomizeWorkbench {
    /// Perform Alchemy Fusion: fuses two matching weapons to increase upgrade level by +1.
    pub fn fuse_alchemy(
        gear: &mut CustomizedGear,
        catalyst_item_id: u32,
        save: &mut SaveData,
    ) -> Result<u8, CustomizeError> {
        if gear.alchemy_level >= 5 {
            return Err(CustomizeError::MaxAlchemyLevelReached);
        }

        if gear.base_item_id != catalyst_item_id {
            return Err(CustomizeError::IncompatibleItems);
        }

        // Locate and consume catalyst item from player inventory
        let pos = save.weapons.iter().position(|w| w.item_id == catalyst_item_id)
            .ok_or(CustomizeError::ItemNotFound)?;

        if save.weapons[pos].quantity < 1 {
            return Err(CustomizeError::InsufficientCatalystQuantity);
        }

        save.weapons[pos].quantity -= 1;
        if save.weapons[pos].quantity == 0 {
            save.weapons.remove(pos);
        }

        gear.alchemy_level += 1;
        Ok(gear.alchemy_level)
    }

    /// Install a passive talisman crystal into a free socket slot.
    pub fn install_slot(
        gear: &mut CustomizedGear,
        effect: SlotEffect,
        talisman_item_id: u32,
        save: &mut SaveData,
    ) -> Result<(), CustomizeError> {
        if gear.installed_slots.len() >= gear.max_slots {
            return Err(CustomizeError::NoFreeSlotsAvailable);
        }

        // Consume talisman material from items inventory
        let pos = save.items.iter().position(|i| i.item_id == talisman_item_id)
            .ok_or(CustomizeError::ItemNotFound)?;

        if save.items[pos].quantity < 1 {
            return Err(CustomizeError::InsufficientCatalystQuantity);
        }

        save.items[pos].quantity -= 1;
        if save.items[pos].quantity == 0 {
            save.items.remove(pos);
        }

        gear.installed_slots.push(effect);
        Ok(())
    }
}
